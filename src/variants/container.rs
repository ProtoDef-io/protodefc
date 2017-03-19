use ::{TypeVariant, TypeData, Type, WeakTypeContainer, Result, TypeContainer};
use super::Variant;

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
    pub virt: bool,

    pub child: WeakTypeContainer,
    pub child_index: usize,
}

pub struct ContainerVariantBuilder {
    typ: Type,
    virt: bool,
    num_virt_fields: usize,
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
            num_virt_fields: 0,
        }
    }

    pub fn field(&mut self, name: String, typ: TypeContainer, virt: bool) {
        let idx = self.typ.data.children.len();
        self.typ.data.children.push(typ.clone());

        match self.typ.variant {
            Variant::Container(ref mut variant) => {
                variant.fields.push(ContainerField {
                    name: name,
                    virt: virt,
                    child: Rc::downgrade(&typ),
                    child_index: idx,
                });
            }
            _ => unreachable!(),
        }
    }

    pub fn build(self) -> ::std::result::Result<TypeContainer, String> {
        if self.virt && self.num_virt_fields != 0 {
            bail!("virtual container must have exactly 1 non-virtual field");
        }
        Ok(Rc::new(RefCell::new(self.typ)))
    }

}
