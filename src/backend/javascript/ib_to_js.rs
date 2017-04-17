use super::builder::{Block, Expr};
use ::backend::imperative_base as ib;
use ::errors::*;
use itertools::Itertools;

pub fn build_block(block: &ib::Block) -> Result<Block> {
    let mut b = Block::new();

    for operation in &block.0 {
        match *operation {
            ib::Operation::Assign { ref output_var, ref value } =>
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
                                             ref cases } } => {
                let cases: Result<Vec<(Expr, Block)>> = cases.iter()
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

                            (format!("case \"{}\"", variant_name).into(), iib)
                        })
                    }).collect();

                b.switch(
                    format!("{}.tag", input_var).into(),
                    cases?
                );
            },
            ib::Operation::ControlFlow { ref input_var,
                                         variant: ib::ControlFlowVariant::MatchLiteral {
                                             ref cases } } => {
                let cases: Result<Vec<(Expr, Block)>> = cases.iter()
                    .map(|&ib::LiteralCase { ref value, ref block }| {
                        build_block(block).map(|ib| {
                            (
                                format!("case {}",
                                        build_literal(value).0).into(),
                                ib
                            )
                        })
                    })
                    .collect();

                b.switch(
                    format!("{}", input_var).into(),
                    cases?
                );
            }
            ib::Operation::Construct { ref output_var,
                                       variant: ib::ConstructVariant::Container {
                                           ref fields } } => {
                let obj_fields = fields.iter()
                    .map(|&(ref name, ref var)| format!("{}: {}", name, var.0))
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
                    format!("{{ tag: \"{}\", data: {} }}", union_tag, variant_inner_var).into()
                );
            }
            ib::Operation::TypeCall { ref input_var, ref type_name,
                                      ref call_type, ref named_type,
                                      ref arguments } => {
                let named_type_inner = named_type.borrow();

                let call = call_for(call_type, named_type_inner.type_id,
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
                        b.var_assign(format!("{}", output), format!("call_out[0]").into());
                        b.var_assign(format!("offset"), format!("call_out[1]").into())
                    }
                }
            }
        }
    }

    Ok(b)
}

fn call_for(typ: &ib::CallType, type_id: u64, input: &str, arguments: &[ib::Var]) -> String {
    let arguments_str = if arguments.len() > 0 {
        format!(", {}", arguments.iter().join(", "))
    } else {
        format!("")
    };

    match *typ {
        ib::CallType::SizeOf(_) =>
            format!("type_{}_size_of({}{})",
                    type_id, input, arguments_str),
        ib::CallType::Serialize =>
            format!("type_{}_serialize({}, buffer, offset{})",
                    type_id, input, arguments_str),
        ib::CallType::Deserialize(_) =>
            format!("type_{}_deserialize({}, offset{})",
                    type_id, input, arguments_str),
    }
}

fn assign_target_for(typ: &ib::CallType) -> String {
    match *typ {
        ib::CallType::SizeOf(ref output) =>
            format!("{}", output),
        ib::CallType::Serialize =>
            format!("offset"),
        ib::CallType::Deserialize(ref output) =>
            format!("[{}, offset]", output),
    }
}

fn build_expr(expr: &ib::Expr) -> Result<Expr> {
    let res = match *expr {
        ib::Expr::InputData => format!("input").into(),
        ib::Expr::Var(ref var) => var.0.clone().into(),
        ib::Expr::Literal(ib::Literal::Number(ref num)) =>
            num.clone(),
        ib::Expr::ContainerField { ref input_var, ref field } =>
            format!("{}[{:?}]", input_var, field),
        ib::Expr::ArrayLength(ref array) =>
            format!("{}.length", array),
        ib::Expr::BinarySize(ref binary, _) =>
            format!("Buffer.byteLength({}, 'utf8')", binary),
    };
    Ok(res.into())
}

fn build_literal(lit: &ib::Literal) -> Expr{
    match lit {
        &ib::Literal::Number(ref val) => val.to_owned().into(),
    }
}
