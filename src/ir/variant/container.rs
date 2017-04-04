use ::{TypeVariant, TypeData, Type, WeakTypeContainer, Result, TypeContainer, CompilerError};
use super::{Variant, VariantType};
use ::FieldPropertyReference;
use ::context::compilation_unit::{CompilationUnit, TypePath};
use ::ir::{TargetType, CompilePass};

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ContainerVariant {
    pub virt: bool,
    pub fields: Vec<ContainerField>,
}
impl TypeVariant for ContainerVariant {

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::Container
    }

    fn resolve_child_name(&self, _data: &TypeData, name: &str) -> Result<WeakTypeContainer> {
        self.fields
            .iter()
            .find(|f| f.name == name)
            .map(|f| f.child.clone())
            .ok_or_else(|| CompilerError::ChildResolveError {
                name: name.to_owned(),
                parent_variant: "container".into(),
            }.into())

    }

    default_has_property_impl!();
    default_get_result_type_impl!();

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass)
                       -> Result<()> {
        match *pass {
            CompilePass::ResolveInternalReferences(ref resolver) => {
                let mut resolves: Vec<WeakTypeContainer> =
                    Vec::with_capacity(self.fields.len());

                for field in &self.fields {
                    match field.field_type {
                        ContainerFieldType::Virtual { ref property } => {
                            let prop_node = resolver(self, data,
                                                     &property.reference)?;

                            let prop_node_u = prop_node.upgrade();
                            let prop_node_ui = prop_node_u.borrow();

                            let prop_valid = prop_node_ui.variant
                                .to_variant()
                                .has_property(&prop_node_ui.data,
                                              &property.property);
                            ensure!(prop_valid != None, CompilerError::NoProperty {
                                property: property.property.clone(),
                                variant: prop_node_ui.variant.get_type(
                                    &prop_node_ui.data),
                            });

                            resolves.push(prop_node);
                        }
                        _ => (),
                    }
                }

                for field in self.fields.iter_mut().rev() {
                    match field.field_type {
                        ContainerFieldType::Virtual { ref mut property } => {
                            property.reference_node = resolves.pop();
                        }
                        _ => (),
                    }
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct ContainerField {
    pub name: String,

    pub child: WeakTypeContainer,
    pub child_index: usize,

    pub field_type: ContainerFieldType,
}

#[derive(Debug)]
pub enum ContainerFieldType {
    /// A field with normal behavior.
    ///
    /// It is both read and written, and exists in the output
    /// data structure.
    Normal,

    /// A virtual field will be read and written, but does
    /// not exist in the output data structure.
    ///
    /// It needs a to reference a property of another field
    /// so that it knows what to write.
    Virtual { property: FieldPropertyReference },

    /// A const field will neither be read or written. It
    /// does not exist in the output data structure.
    ///
    /// It will always have a fixed value.
    ///
    /// It can have an optional property reference. If it has
    /// one, it will validate that the property is equal to the
    /// constant.
    Const {
        validate_property: Option<FieldPropertyReference>,
        value: String,
    },
}

pub struct ContainerVariantBuilder {
    typ: Type,
    virt: bool,
    num_non_virt_fields: usize,
}
impl ContainerVariantBuilder {
    pub fn new(virt: bool) -> ContainerVariantBuilder {
        ContainerVariantBuilder {
            typ: Type {
                data: TypeData::default(),
                variant: Variant::Container(ContainerVariant {
                    virt: virt,
                    fields: vec![],
                }),
            },
            virt: virt,
            num_non_virt_fields: 0,
        }
    }

    pub fn normal_field(&mut self, name: String, typ: TypeContainer) {
        self.field(name, typ, ContainerFieldType::Normal);
    }

    pub fn field(&mut self, name: String, typ: TypeContainer, container_type: ContainerFieldType) {
        let idx = self.typ.data.children.len();
        self.typ.data.children.push(typ.clone());

        match container_type {
            ContainerFieldType::Normal => self.num_non_virt_fields += 1,
            _ => (),
        }

        match self.typ.variant {
            Variant::Container(ref mut variant) => {
                variant.fields.push(ContainerField {
                    name: name,
                    child: typ.downgrade(),
                    child_index: idx,
                    field_type: container_type,
                });
            }
            _ => unreachable!(),
        }
    }

    pub fn build(self) -> ::std::result::Result<TypeContainer, String> {
        if self.virt && self.num_non_virt_fields != 1 {
            bail!("virtual container must have exactly 1 non-virtual field");
        }
        Ok(TypeContainer::new(self.typ))
    }
}
