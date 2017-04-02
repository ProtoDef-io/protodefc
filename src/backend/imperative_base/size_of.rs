use ::errors::*;
use ::ir::{Type, TypeVariant, TypeData, TypeContainer, FieldPropertyReference};
use ::ir::variant::{Variant, SimpleScalarVariant, ContainerVariant, ContainerField, ContainerFieldType, ArrayVariant, UnionVariant};
use super::*;

pub fn generate_size_of(typ: TypeContainer) -> Result<Block> {
    let typ_inner = typ.borrow();
    size_of_for_type(&*typ_inner)
        .size_of(&typ_inner.data)
}

fn size_of_for_type<'a>(typ: &'a Type) -> &'a BaseSizeOf {
    match typ.variant {
        Variant::SimpleScalar(ref inner) => inner,
        Variant::Container(ref inner) => inner,
        Variant::Array(ref inner) => inner,
        Variant::Union(ref inner) => inner,
        _ => unimplemented!(),
    }
}

trait BaseSizeOf: TypeVariant {
    fn size_of(&self, data: &TypeData) -> Result<Block>;
}

fn input_for(data: &TypeData) -> String {
    format!("type_input_{}", data.ident.unwrap())
}
fn input_for_type(typ: TypeContainer) -> String {
    let typ_inner = typ.borrow();
    input_for(&typ_inner.data)
}

impl BaseSizeOf for SimpleScalarVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        ops.push(Operation::AddCount(Expr::TypeCall {
            typ: CallType::SizeOf,
            type_name: data.name.clone().into(),
            input: input_for(data).into(),
        }));

        Ok(Block(ops))
    }

}

fn find_field_index(variant: &ContainerVariant, property: &FieldPropertyReference)
                    -> usize {
    let property_field_ident = {
        let rc = property.reference_node.clone().unwrap().upgrade().unwrap();
        let rc_inner = rc.borrow();
        rc_inner.data.ident.unwrap()
    };

    let (idx, _) = variant.fields
        .iter().enumerate()
        .find(|&(_, f)| {
            let rc = f.child.clone().upgrade().unwrap();
            let rc_inner = rc.borrow();
            rc_inner.data.ident.unwrap() == property_field_ident
        })
        .unwrap();

    idx
}

fn build_var_accessor(variant: &ContainerVariant, data: &TypeData,
                      block: &mut Vec<Operation>, field_num: usize)
                      -> Result<()> {
    let field: &ContainerField = &variant.fields[field_num];
    let field_input_var = input_for_type(field.child.upgrade().unwrap());

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
            let ref_node_rc = property.reference_node.clone().unwrap()
                .upgrade().unwrap();
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
                            let target_type = union.match_type.unwrap();
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

impl BaseSizeOf for ContainerVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        for (idx, field) in self.fields.iter().enumerate() {
            let child_typ = field.child.upgrade().unwrap();
            //let child_input_var = input_for_type(child_typ.clone());

            build_var_accessor(self, data, &mut ops, idx)?;
            ops.push(Operation::Block(generate_size_of(child_typ)?));
        }

        //for field in self.fields.iter() {
        //    let child_typ = field.child.upgrade().unwrap();
        //    ops.push(Operation::Block(generate_size_of(child_typ)?));
        //}

        Ok(Block(ops))
    }

}

impl BaseSizeOf for ArrayVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let ident = data.ident.unwrap();
        let index_var = format!("array_{}_index", ident);

        let child_input_var = input_for_type(self.child.upgrade().unwrap());

        ops.push(Operation::ForEachArray {
            array: input_for(data).into(),
            index: index_var.clone().into(),
            typ: child_input_var.clone().into(),
            block: generate_size_of(self.child.upgrade().unwrap())?,
        });

        Ok(Block(ops))
    }

}

impl BaseSizeOf for UnionVariant {

    fn size_of(&self, data: &TypeData) -> Result<Block> {
        let mut ops: Vec<Operation> = Vec::new();

        let cases: Result<Vec<UnionTagCase>> = self.cases.iter().map(|case| {
            let child_rc = case.child.upgrade().unwrap();
            let child_inner = child_rc.borrow();

            let mut i_ops: Vec<Operation> = Vec::new();

            let inner = generate_size_of(child_rc.clone())?;
            i_ops.push(Operation::Block(inner));

            Ok(UnionTagCase {
                variant_name: case.case_name.clone(),
                variant_var: Some(
                    input_for(&child_inner.data).into()),
                block: Block(i_ops),
            })
        }).collect();

        ops.push(Operation::MapValue {
            input: input_for(data).into(),
            output: "".to_owned().into(),
            operation: MapOperation::UnionTagToExpr(cases?),
        });

        Ok(Block(ops))
    }

}
