use ::ir::spec::{TypeVariant, TypeData, Type, WeakTypeContainer, TypeContainer, CompilePass};
use ::ir::spec::reference::Reference;
use ::ir::spec::data::{SpecChildHandle, SpecReferenceHandle, ReferenceAccessTime};
use ::ir::spec::variant::{Variant, VariantType};
use ::ir::type_spec::{TypeSpecVariant, ContainerSpec, ContainerFieldSpec, WeakTypeSpecContainer};
use ::ir::name::Name;
use ::errors::*;

#[derive(Debug)]
pub struct ContainerVariant {
    pub virt: bool,
    pub fields: Vec<ContainerField>,
}
impl TypeVariant for ContainerVariant {

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::Container
    }

    fn resolve_child_name(&self, _data: &TypeData, name: &Name) -> Result<WeakTypeContainer> {
        self.fields
            .iter()
            .find(|f| &f.name == name)
            .map(|f| f.child.clone())
            .ok_or_else(|| CompilerError::ChildResolveError {
                name: name.clone(),
                parent_variant: "container".into(),
            }.into())

    }

    default_has_property_impl!();

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass)
                       -> Result<()> {
        match *pass {
            CompilePass::MakeTypeSpecs => {
                data.type_spec = Some(TypeSpecVariant::Container(ContainerSpec {
                    name: Name::new("placeholder".to_owned())?, // TODO
                    fields: self.fields.iter().map(|f| {
                        let child_rc = f.child.clone().upgrade();
                        let child = child_rc.borrow();
                        ContainerFieldSpec {
                            name: f.name.clone().into(),
                            type_spec: child.data.type_spec.clone().unwrap(),
                        }
                    }).collect(),
                }).into());
                Ok(())
            }
            CompilePass::GenerateFieldAccessOrder => {
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct ContainerField {
    pub name: Name,

    pub child: WeakTypeContainer,
    pub child_handle: SpecChildHandle,

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
    Virtual {
        reference: Reference,
        reference_handle: SpecReferenceHandle,
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
        self.num_non_virt_fields += 1;
        self.field(name, typ, ContainerFieldType::Normal);
    }

    pub fn virtual_field(&mut self, name: String, typ: TypeContainer, value_ref: Reference) {
        let handle = self.typ.data.add_reference(
            value_ref.clone(), ReferenceAccessTime::ReadWrite);
        self.field(name, typ, ContainerFieldType::Virtual {
            reference: value_ref,
            reference_handle: handle,
        });
    }

    fn field(&mut self, name: String, typ: TypeContainer, container_type: ContainerFieldType) {
        let child_handle = self.typ.data.add_child(typ.clone());

        match self.typ.variant {
            Variant::Container(ref mut variant) => {
                variant.fields.push(ContainerField {
                    name: name.into(),
                    child: typ.downgrade(),
                    child_handle: child_handle,
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
