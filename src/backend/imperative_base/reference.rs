use ::ir::spec::{TypeData, TypeVariant};
use ::ir::spec::variant::Variant;
use ::ir::spec::reference::ReferenceItem;
use ::ir::spec::data::{SpecReferenceHandle, ReferencePathEntryData,
                       ReferencePathEntryOperation};
use ::ir::type_spec::{TypeSpecVariant, EnumSpec};
use ::ir::type_spec::property::TypeSpecPropertyVariant;
use super::{Var, Block, Operation, Expr, MapOperation, Literal, UnionTagCase};
use super::utils::*;

pub fn build_reference_accessor(variant: &TypeVariant, data: &TypeData,
                                reference_handle: SpecReferenceHandle,
                                output_var: Var, is_read: bool) -> Block {
    let reference = data.get_reference(reference_handle);

    if is_read {
        // We can't reference a field from within itself, the data type is not yet
        // constructed.
        // This is a sanity assertion, it should be checked earlier in compilation.
        assert!(!(reference.up() == 0 && reference.num_operations() == 0));

        // We can't access fields that has not been deserialized yet.
        // Sanity assertion.
        //let ref_root = data.get_reference_root(reference_handle).upgrade();
        //let ref_root_ident = ref_root.borrow().data.ident.unwrap();
        //assert!(ref_root_ident < data.ident.unwrap());

        // When reading the parent containers are not created yet.
        // Sanity assertion.
        match reference.items.first() {
            Some(&ReferenceItem::Down(_)) => (),
            Some(&ReferenceItem::Property(_)) =>
                panic!("cannot reference property of parent when reading"),
            None =>
                panic!("cannot reference parent value when reading"),
        }
    }

    build_reference_accessor_inner(variant, data, reference_handle, output_var, is_read)
}

fn build_reference_accessor_inner(_variant: &TypeVariant, data: &TypeData,
                                  reference_handle: SpecReferenceHandle,
                                  output_var: Var, _is_read: bool) -> Block {
    let reference = data.get_reference_data(reference_handle);

    let ref_root_rc = data.get_reference_root(reference_handle).upgrade();
    let ref_root = ref_root_rc.borrow();

    let mut ops: Vec<Operation> = Vec::new();

    let mut res_num = 0;
    let mut prev_res = output_for(&ref_root.data);

    for elem in reference.get_path().iter().enumerate() {
        match elem {
            (0, &ReferencePathEntryData { operation: ReferencePathEntryOperation::Down(ref name), .. }) => {
                // FIXME: This should be done somewhere else in compilation
                let child = ref_root.variant.to_variant()
                    .resolve_child_name(&ref_root.data, name)
                    .unwrap().upgrade();
                prev_res = output_for_type(&child);
            }
            (_, &ReferencePathEntryData { operation: ReferencePathEntryOperation::Down(ref name), .. }) => {
                let next_res = var_for(&format!("int_val_{}", res_num), data);
                res_num += 1;

                ops.push(Operation::Assign {
                    name: next_res.clone().into(),
                    value: Expr::ContainerField {
                        value: Box::new(Expr::Var(prev_res.into())),
                        field: name.clone(),
                    },
                });

                prev_res = next_res;
            },
            (_, &ReferencePathEntryData {
                operation: ReferencePathEntryOperation::NodeProperty(ref name),
                ref node, ref type_spec, .. }) => {

                let next_res = var_for(&format!("int_val_{}", res_num), data);
                res_num += 1;

                match name.as_ref() {
                    //"length" => {
                    //    ops.push(Operation::MapValue {
                    //        input: prev_res.into(),
                    //        output: next_res.clone().into(),
                    //        operation: MapOperation::ArrayLength,
                    //    });
                    //}
                    "tag" => {
                        let node_rc = node.clone().unwrap().upgrade();
                        let node_inner = node_rc.borrow();

                        match node_inner.variant {
                            Variant::Union(ref union) => {
                                let cases = union.cases.iter().map(|case| {
                                    let block = Block(vec![
                                        Operation::Assign {
                                            name: next_res.to_owned().into(),
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
                                ops.push(Operation::MapValue {
                                    input: prev_res.into(),
                                    output: next_res.clone().into(),
                                    operation: MapOperation::UnionTagToExpr(cases),
                                });
                            }
                            // TODO: This NEEDS to be validated earlier.
                            // The way it's done right now is a hack.
                            _ => unreachable!(),
                        }
                    }
                    _ => unimplemented!(),
                }
                prev_res = next_res;
            }

            (_, &ReferencePathEntryData {
                operation: ReferencePathEntryOperation::TypeSpecProperty(ref property),
                ref type_spec, .. }) => {

                let next_res = var_for(&format!("int_val_{}", res_num), data);
                res_num += 1;

                match property.variant {
                    TypeSpecPropertyVariant::ArrayLength => {
                        ops.push(Operation::MapValue {
                            input: prev_res.into(),
                            output: next_res.clone().into(),
                            operation: MapOperation::ArrayLength,
                        });
                    }
                    TypeSpecPropertyVariant::BinarySize(ref encoding) => {
                        ops.push(Operation::MapValue {
                            input: prev_res.into(),
                            output: next_res.clone().into(),
                            operation: MapOperation::BinarySize(encoding.clone()),
                        })
                    }
                }

                prev_res = next_res;
            }

            _ => unimplemented!(),
        }
    }

    ops.push(Operation::Assign {
        name: output_var,
        value: Expr::Var(prev_res.into()),
    });

    Block(ops)
}

//pub fn build_reference_accessor(data: &TypeData, reference_handle: SpecReferenceHandle,
//                                output_var: Var, is_read: bool) -> Block {
//    if is_read {
//        // Builds a reference accessor that works in the context of reading data.
//        // This will operate by the assumption that the containing object is already
//        // constructed.
//
//        let reference = data.get_reference(reference_handle);
//
//        // We can't reference a field from within itself, the data type is not yet
//        // constructed.
//        // This is a sanity assertion, it should be checked earlier in compilation.
//        assert!(!(reference.up() == 0 && reference.num_operations() == 0));
//
//        // We can't access fields that has not been deserialized yet.
//        // Sanity assertion.
//        let ref_root_ident = data.get_reference_root(reference_handle).upgrade().borrow()
//            .data.ident.unwrap();
//        assert!(ref_root_ident < data.ident.unwrap());
//
//        if reference.up() == 0 {
//            unimplemented!()
//        } else {
//            // If we go up, this will behave exactly the same as when writing.
//            build_reference_accessor_inner(data, reference_handle, output_var, is_read)
//        }
//    } else {
//        // Builds a reference accessor that works in the context of writing data.
//        // This will operate by the assumption that the containing object is not
//        // constructed, and will access previous fields within the same object by
//        // variable name.
//
//        build_reference_accessor_inner(data, reference_handle, output_var, is_read)
//    }
//}
//
//fn build_reference_accessor_inner(data: &TypeData, reference_handle: SpecReferenceHandle,
//                                  output_var: Var, is_read: bool) -> Block {
//    let reference = data.get_reference(reference_handle);
//    let root_node = data.get_reference_root(reference_handle);
//
//    unimplemented!()
//}
