use ::errors::*;
use ::ir::{FieldPropertyReference};
use ::ir::typ::{Type, TypeVariant, TypeData, TypeContainer};
use ::ir::typ::variant::*;
use super::*;
use super::utils::*;

pub fn find_field_index(variant: &ContainerVariant, property: &FieldPropertyReference)
                        -> usize {
    let property_field_ident = {
        let rc = property.reference_node.clone().unwrap().upgrade();
        let rc_inner = rc.borrow();
        rc_inner.data.ident.unwrap()
    };

    let (idx, _) = variant.fields
        .iter().enumerate()
        .find(|&(_, f)| {
            let rc = f.child.clone().upgrade();
            let rc_inner = rc.borrow();
            rc_inner.data.ident.unwrap() == property_field_ident
        })
        .unwrap();

    idx
}

pub fn build_var_accessor(variant: &ContainerVariant, data: &TypeData,
                      block: &mut Vec<Operation>, field_num: usize)
                      -> Result<()> {
    let field: &ContainerField = &variant.fields[field_num];
    let field_input_var = input_for_type(field.child.upgrade());

    build_var_accessor_inner(variant, data, block, field_num, &field_input_var)
}

fn build_var_accessor_inner(variant: &ContainerVariant, data: &TypeData,
                            block: &mut Vec<Operation>, field_num: usize,
                            chain_var: &str)
                            -> Result<()> {
    let field: &ContainerField = &variant.fields[field_num];

    match field.field_type {
        ContainerFieldType::Normal => {
            if variant.virt {
                block.push(Operation::Assign {
                    name: chain_var.to_owned().into(),
                    value: Expr::Var(input_for(data).into()),
                });
                Ok(())
            } else {
                block.push(Operation::Assign {
                    name: chain_var.to_owned().into(),
                    value: Expr::ContainerField {
                        value: Box::new(Expr::Var(input_for(data).into())),
                        field: field.name.clone(),
                    },
                });
                Ok(())
            }
        }
        ContainerFieldType::Virtual { ref property } => {
            let ref_node_rc = property.reference_node.clone().unwrap().upgrade();
            let ref_node = ref_node_rc.borrow();

            if property.reference.up == 0 {
                let next_index = find_field_index(variant, property);
                build_var_accessor_inner(
                    variant, data, block, next_index, chain_var)?;
            } else {
                block.push(Operation::Assign {
                    name: chain_var.to_owned().into(),
                    value: Expr::Var(input_for(&ref_node.data).into()),
                });
            };

            let property_variant = match property.property.as_ref() {
                "length" => MapOperation::ArrayLength,
                "tag" => {
                    match ref_node.variant {
                        Variant::Union(ref union) => {
                            let cases = union.cases.iter().map(|case| {
                                let block = Block(vec![
                                    Operation::Assign {
                                        name: chain_var.to_owned().into(),
                                        value: Expr::Literal(Literal::Number(
                                            case.match_val_str.clone()))
                                    }
                                ]);

                                UnionTagCase {
                                    variant_name: case.case_name.clone(),
                                    variant_var: None,
                                    block: block,
                                }
                            }).collect();
                            MapOperation::UnionTagToExpr(cases)
                        },
                        // TODO: This NEEDS to be validated earlier.
                        // The way it's done right now is a hack.
                        _ => unreachable!(),
                    }
                },
                _ => unimplemented!(),
            };

            block.push(Operation::MapValue {
                input: chain_var.to_owned().into(),
                output: chain_var.to_owned().into(),
                operation: property_variant,
            });


            Ok(())
        }
        _ => unimplemented!(),
    }
}
