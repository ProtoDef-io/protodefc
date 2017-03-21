use ::{TypeVariant, TypeData, Type, WeakTypeContainer, Result, TypeContainer};
use ::field_reference::FieldReference;
use super::Variant;

use std::rc::Rc;
use std::cell::RefCell;

// Array
#[derive(Debug)]
pub struct ArrayVariant {
    pub count: Option<WeakTypeContainer>,
    pub count_path: FieldReference,

    pub child: WeakTypeContainer,
    pub child_index: usize,
}
impl TypeVariant for ArrayVariant {
    default_resolve_child_name_impl!();
    default_has_property_impl!();

    fn do_resolve_references(&mut self, data: &mut TypeData,
                             resolver: &::ReferenceResolver) -> Result<()> {
        self.count = Some(resolver(self, data, &self.count_path)?);
        Ok(())
    }
}

impl ArrayVariant {

    pub fn new(count_ref: FieldReference, child: TypeContainer) -> TypeContainer {
        let mut data = TypeData::default();
        data.name = "array".into();
        data.children.push(child.clone());

        Rc::new(RefCell::new(Type {
            variant: Variant::Array(ArrayVariant {
                count: None,
                count_path: count_ref,

                child: Rc::downgrade(&child),
                child_index: 0,
            }),
            data: data,
        }))
    }

}
