use super::builder::{Block, Expr};
use ::ir::type_spec::{TypeSpecContainer, TypeSpecVariant, BinarySpec, BinaryEncoding};
use ::ir::type_spec::literal::{TypeSpecLiteral, TypeSpecLiteralVariant};
use ::backend::imperative_base as ib;
use ::errors::*;
use itertools::Itertools;

pub fn build_block(block: &ib::Block) -> Result<Block> {
    let mut b = Block::new();

    for operation in &block.0 {
        match *operation {
            ib::Operation::ThrowError =>
                b.expr(format!("throw new Error(\"protodef error\")").into()),
            ib::Operation::Declare { .. } => (),
            ib::Operation::Assign { ref output_var, ref value, .. } =>
                b.var_assign(output_var.string(), build_expr(value)?.into()),
            ib::Operation::AddCount(ref var) =>
                b.assign("count".into(),
                         format!("count + {}", var).into()),
            ib::Operation::Block(ref inner_block) => b.scope(build_block(inner_block)?),
            ib::Operation::ControlFlow { ref input_var,
                                         variant: ib::ControlFlowVariant::ForEachArray {
                                             ref loop_index_var, ref loop_value_var,
                                             ref inner } } => {
                let index_var = loop_index_var;
                let length_var = format!("{}_length", index_var);

                let mut ib = Block::new();
                {
                    let expr = format!("{}[{}]", input_var, index_var);
                    ib.var_assign(loop_value_var.string(), expr.into());

                    ib.scope(build_block(inner)?);
                }

                b.var_assign(length_var.clone(),
                             format!("{}.length", input_var).into());
                b.for_(
                    format!("var {} = 0", index_var).into(),
                    format!("{} < {}", index_var, length_var).into(),
                    format!("{}++", index_var).into(),
                    ib
                );
            },
            ib::Operation::ControlFlow { ref input_var,
                                         variant: ib::ControlFlowVariant::MatchUnionTag {
                                             ref cases, ref default, .. } } => {
                let mut cases: Vec<(Expr, Block)> = cases.iter()
                    .map(|&ib::UnionTagCase { ref variant_name, ref block,
                                              ref variant_var }| {
                        build_block(block).map(|ib| {
                            let mut iib = Block::new();

                            if let &Some(ref variant_var_inner) = variant_var {
                                iib.var_assign(
                                    variant_var_inner.string(),
                                    format!("{}.data", input_var).into()
                                );
                            }
                            iib.block(ib);

                            (format!("case \"{}\"", variant_name.snake()).into(), iib)
                        })
                    }).collect::<Result<_>>()?;

                let mut default_block = Block::new();
                if let Some(ref variant_var_inner) = default.0 {
                    default_block.var_assign(
                        variant_var_inner.string(),
                        format!("{}.data", input_var).into()
                    );
                }
                default_block.block(build_block(&default.1)?);
                cases.push((
                    format!("default").into(),
                    default_block,
                ));

                b.switch(
                    format!("{}.tag", input_var).into(),
                    cases
                );
            },
            ib::Operation::ControlFlow { ref input_var,
                                         variant: ib::ControlFlowVariant::MatchValue {
                                             ref value_type, ref cases,
                                             ref default } } => {
                build_control_flow(input_var, value_type, cases, default, &mut b)?;
            }
            ib::Operation::Construct { ref output_var,
                                       variant: ib::ConstructVariant::Container {
                                           ref fields } } => {
                let obj_fields = fields.iter()
                    .map(|&(ref name, ref var)| format!("{}: {}", name.snake(), var))
                    .join(", ");

                b.var_assign(output_var.string(), format!("{{ {} }}", obj_fields).into());
            }
            ib::Operation::Construct { ref output_var,
                                       variant: ib::ConstructVariant::Array {
                                           ref array_node_ident, ref count_input_var,
                                           ref inner_result_var, ref inner } } => {
                let index_var = format!("array_{}_index", array_node_ident);

                let mut ib = Block::new();
                ib.block(build_block(inner)?);
                ib.expr(format!("{}.push({})", output_var, inner_result_var).into());

                b.var_assign(output_var.string(), "[]".into());
                b.for_(
                    format!("var {} = 0", index_var).into(),
                    format!("{} < {}", index_var, count_input_var).into(),
                    format!("{}++", index_var).into(),
                    ib
                );
            }
            ib::Operation::Construct { ref output_var,
                                       variant: ib::ConstructVariant::Union {
                                           ref union_tag, ref variant_inner_var, .. } } => {
                b.var_assign(
                    output_var.string(),
                    format!("{{ tag: \"{}\", data: {} }}", union_tag.snake(), variant_inner_var).into()
                );
            }
            ib::Operation::TypeCall { ref input_var, ref type_name,
                                      ref call_type, ref named_type,
                                      ref arguments } => {
                let named_type_inner = named_type.borrow();

                let call = call_for(call_type, format!("{}_{}", named_type_inner.type_id, named_type_inner.path.str_name()),
                                    input_var.str(), arguments);

                match *call_type {
                    ib::CallType::SizeOf(ref output) => {
                        b.var_assign(format!("{}", output), call.into());
                    }
                    ib::CallType::Serialize => {
                        b.var_assign(format!("offset"), call.into());
                    }
                    ib::CallType::Deserialize(ref output) => {
                        b.var_assign(format!("call_out"), call.into());
                        b.var_assign(format!("{}", output), format!("call_out.value").into());
                        b.var_assign(format!("offset"), format!("call_out.size").into())
                    }
                }
            }
        }
    }

    Ok(b)
}

fn build_control_flow(input_var: &ib::Var, value_type: &TypeSpecContainer,
                      cases: &Vec<ib::MatchCase>,
                      default: &(Option<ib::Var>, ib::Block),
                      b: &mut Block) -> Result<()> {
    let value_type_rc = value_type.clone().follow();
    let value_type_inner = value_type_rc.borrow();

    match value_type_inner.variant {
        TypeSpecVariant::Integer(_)
            | TypeSpecVariant::Binary(BinarySpec { encoding: BinaryEncoding::Utf8 })
            | TypeSpecVariant::Boolean => {

            let mut cases: Vec<(Expr, Block)> = cases.iter()
                .map(|&ib::MatchCase { ref match_value, ref block, .. }| {
                    build_block(block).map(|ib| {
                        (
                            format!("case {}",
                                    build_literal(match_value).0).into(),
                            ib
                        )
                    })
                })
                .collect::<Result<_>>()?;

            cases.push((
                format!("default").into(),
                build_block(&default.1)?,
            ));

            b.switch(
                format!("{}", input_var).into(),
                cases
            );
        }
        TypeSpecVariant::Enum(_) => {
            let mut cases: Vec<(Expr, Block)> = cases.iter()
                .map(|&ib::MatchCase { ref match_value, ref block,
                                       ref inner_value_var }| {
                    build_block(block).map(|ib| {
                        let mut iib = Block::new();

                        if let &Some(ref variant_var_inner) = inner_value_var {
                            iib.var_assign(
                                variant_var_inner.string(),
                                format!("{}.data", input_var).into()
                            );
                        }
                        iib.block(ib);

                        (format!("case {}", build_literal(match_value).0).into(), iib)
                    })
                }).collect::<Result<_>>()?;

            let mut default_block = Block::new();
            if let Some(ref variant_var_inner) = default.0 {
                default_block.var_assign(
                    variant_var_inner.string(),
                    format!("{}.data", input_var).into()
                );
            }
            default_block.block(build_block(&default.1)?);
            cases.push((
                format!("default").into(),
                default_block,
            ));

            b.switch(
                format!("{}.tag", input_var).into(),
                cases
            );
        }
        ref i => panic!("{:?}", i),
    }

    Ok(())
}

fn call_for(typ: &ib::CallType, type_name: String, input: &str, arguments: &[ib::Var]) -> String {
    let arguments_str = if arguments.len() > 0 {
        format!(", {}", arguments.iter().join(", "))
    } else {
        format!("")
    };

    match *typ {
        ib::CallType::SizeOf(_) =>
            format!("sizeOf_{}({}{})",
                    type_name, input, arguments_str),
        ib::CallType::Serialize =>
            format!("serialize_{}({}, buffer, offset{})",
                    type_name, input, arguments_str),
        ib::CallType::Deserialize(_) =>
            format!("deserialize_{}({}, offset{})",
                    type_name, input, arguments_str),
    }
}

fn build_expr(expr: &ib::Expr) -> Result<Expr> {
    let res = match *expr {
        ib::Expr::InputData => format!("input").into(),
        ib::Expr::Var(ref var) => var.0.clone().into(),
        ib::Expr::Literal(ib::Literal::Number(ref num)) =>
            num.clone(),
        ib::Expr::ContainerField { ref input_var, ref field } =>
            format!("{}[\"{}\"]", input_var, field.snake()),
        ib::Expr::ArrayLength(ref array) =>
            format!("{}.length", array),
        ib::Expr::BinarySize(ref binary, _) =>
            format!("Buffer.byteLength({}, 'utf8')", binary),
    };
    Ok(res.into())
}

fn build_literal(lit: &TypeSpecLiteral) -> Expr {
    match lit.variant {
        TypeSpecLiteralVariant::Integer { ref data } => format!("{}", data).into(),
        TypeSpecLiteralVariant::EnumTag { ref enum_variant } =>
            format!("\"{}\"", enum_variant.name.snake()).into(),
        TypeSpecLiteralVariant::Boolean { ref data } => format!("{}", data).into(),
        // TODO
        TypeSpecLiteralVariant::Binary { ref data } =>
            format!("\"{}\"", ::std::str::from_utf8(data).unwrap()).into(),
        _ => unimplemented!(),
    }
}
