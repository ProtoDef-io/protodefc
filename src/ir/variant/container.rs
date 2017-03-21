use ::{TypeVariant, TypeData, Type, WeakTypeContainer, Result, TypeContainer,
       FieldPropertyReference};
use super::Variant;
use ::field_reference::FieldReference;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ContainerVariant {
    pub virt: bool,
    pub fields: Vec<ContainerField>,
}
impl TypeVariant for ContainerVariant {
    fn resolve_child_name(&self, _data: &TypeData, name: &str) -> Result<WeakTypeContainer> {
        self.fields.iter()
            .find(|f| f.name == name)
            .map(|f| f.child.clone())
            .ok_or("container has no field".into())
    }
    default_has_property_impl!();
    fn do_resolve_references(&mut self, data: &mut TypeData,
                             resolver: &::ReferenceResolver) -> Result<()> {
        Ok(())
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
    Virtual{
        property: FieldPropertyReference,
    },

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

    pub fn field(&mut self, name: String, typ: TypeContainer,
                 container_type: ContainerFieldType) {
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
                    child: Rc::downgrade(&typ),
                    child_index: idx,
                    field_type: container_type,
                });
            }
            _ => unreachable!(),
        }
    }

    pub fn build(self) -> ::std::result::Result<TypeContainer, String> {
        if self.virt && self.num_non_virt_fields != 0 {
            bail!("virtual container must have exactly 1 non-virtual field");
        }
        Ok(Rc::new(RefCell::new(self.typ)))
    }

}
