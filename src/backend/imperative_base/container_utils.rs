use ::errors::*;
use ::ir::spec::TypeData;
use ::ir::spec::variant::*;
use super::*;
use super::utils::*;
use super::reference::build_reference_accessor;

pub fn build_field_accessor(variant: &ContainerVariant, data: &TypeData,
                            block: &mut Vec<Operation>, field_num: usize, is_read: bool)
                            -> Result<()> {
    let field: &ContainerField = &variant.fields[field_num];
    let field_input_var = input_for_type(&field.child.upgrade());
    build_field_accessor_inner(variant, data, block, field_num, &field_input_var, is_read)
}

fn build_field_accessor_inner(variant: &ContainerVariant, data: &TypeData,
                            block: &mut Vec<Operation>, field_num: usize,
                            chain_var: &str, is_read: bool)
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
        ContainerFieldType::Virtual { reference_handle, .. } => {
            block.push(Operation::Block(
                build_reference_accessor(variant, data,
                                         reference_handle, chain_var.to_owned().into(),
                                         is_read)));
            Ok(())
        }
    }
}
