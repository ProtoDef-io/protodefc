use ::errors::*;
use ::ir::spec::{TypeVariant, TypeData, TypeContainer};
use ::ir::spec::variant::*;
use ::ir::spec::data::ReferenceAccessTime;
use super::*;
use super::utils::*;
use super::reference::build_reference_accessor;

pub fn generate_deserialize(typ: TypeContainer) -> Result<Block> {
    let typ_inner = typ.borrow();
    codegen_for_type(&*typ_inner)
        .deserialize(&typ_inner.data)
}

pub trait BaseDeserialize: TypeVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block>;
}

impl BaseDeserialize for SimpleScalarVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let arguments = self.arguments.iter()
            //.filter(|arg| data.get_reference_data(arg.handle.unwrap()).access_time == ReferenceAccessTime::ReadWrite)
            .enumerate()
            .map(|(idx, arg)| {
                let arg_var = format!("arg_{}", idx);
                let accessor_block = build_reference_accessor(self, data, arg.handle.unwrap(),
                                                              arg_var.clone().into(), false);
                ops.push(Operation::Block(accessor_block));
                arg_var.into()
            })
            .collect();

        ops.push(Operation::TypeCall {
            input_var: "buffer".to_owned().into(),
            call_type: CallType::Deserialize(output_for(data).into()),
            type_name: data.name.clone().into(),
            named_type: self.target.clone().unwrap(),
            arguments: arguments,
        });

        Ok(Block(ops))
    }
}

fn field_is_normal(field: &ContainerField) -> bool {
    match field.field_type {
        ContainerFieldType::Normal => true,
        _ => false,
    }
}

impl BaseDeserialize for ContainerVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        for (_idx, field) in self.fields.iter().enumerate() {
            let child_typ = field.child.upgrade();

            ops.push(Operation::Block(generate_deserialize(child_typ)?));
        }

        if self.virt {
            let real_field = self.fields.iter()
                .find(|f| field_is_normal(f))
                .unwrap();
            let real_field_out = output_for_type(&real_field.child.upgrade());

            ops.push(Operation::Assign {
                output_var: output_for(data).into(),
                value: Expr::Var(real_field_out.into()),
            })
        } else {
            let fields = self.fields.iter()
                .filter(|f| field_is_normal(f))
                .map(|field| (
                    field.name.clone(),
                    output_for_type(&field.child.upgrade()).into()
                ))
                .collect();

            ops.push(Operation::Construct {
                output_var: output_for(data).into(),
                variant: ConstructVariant::Container {
                    fields: fields,
                }
            });
        }

        Ok(Block(ops))
    }
}

impl BaseDeserialize for ArrayVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let count_var = var_for("count", data);
        let count_root_node = data.get_reference_root(self.count_handle).upgrade();
        ops.push(Operation::Block(build_reference_accessor(
            self, data, self.count_handle, count_var.clone().into(), true)));

        let child_rc = self.child.upgrade();
        let child_var = output_for_type(&child_rc);

        let ident = data.ident.unwrap();
        let item_var = format!("array_{}_index", ident);

        ops.push(Operation::Construct {
            output_var: output_for(data).into(),
            variant: ConstructVariant::Array {
                array_node_ident: ident,
                count_input_var: count_var.into(),
                inner_result_var: child_var.into(),
                inner: generate_deserialize(child_rc)?,
            }
        });

        Ok(Block(ops))
    }
}

impl BaseDeserialize for UnionVariant {
    fn deserialize(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let tag_root_node = data.get_reference_root(self.match_target_handle).upgrade();
        let tag_var = var_for_type("tag", &tag_root_node);
        ops.push(Operation::Block(build_reference_accessor(
            self, data, self.match_target_handle, tag_var.clone().into(), true)));

        let out_var = output_for(data);

        let cases: Result<Vec<LiteralCase>> = self.cases.iter()
            .map(|case| {
                let child_rc = case.child.upgrade();
                let mut i_ops: Vec<Operation> = Vec::new();

                generate_deserialize(child_rc.clone()).map(|v| {
                    i_ops.push(Operation::Block(v));
                    i_ops.push(Operation::Construct {
                        output_var: out_var.clone().into(),
                        variant: ConstructVariant::Union {
                            union_name: self.union_name.clone(),
                            union_tag: case.case_name.clone(),
                            variant_inner_var: output_for_type(&child_rc).into(),
                        },
                    });

                    LiteralCase {
                        value: Literal::Number(case.match_val_str.clone()),
                        block: Block(i_ops),
                    }
                })
            })
            .collect();

        let mut default_ops = Vec::new();
        if let Some(ref case) = self.default_case {
            let child_rc = case.child.upgrade();
            default_ops.push(Operation::Block(generate_deserialize(child_rc.clone())?));
            default_ops.push(Operation::Construct {
                output_var: out_var.clone().into(),
                variant: ConstructVariant::Union {
                    union_name: self.union_name.clone(),
                    union_tag: case.case_name.clone(),
                    variant_inner_var: output_for_type(&child_rc).into(),
                },
            });
        } else {
            default_ops.push(Operation::ThrowError);
        }

        ops.push(Operation::ControlFlow {
            input_var: tag_var.into(),
            variant: ControlFlowVariant::MatchLiteral {
                cases: cases?,
                default: Block(default_ops),
            },
        });

        Ok(Block(ops))
    }
}
