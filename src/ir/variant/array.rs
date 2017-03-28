use ::{TypeVariant, TypeData, Type, WeakTypeContainer, Result, TypeContainer};
use ::ir::TargetType;
use ::FieldReference;
use super::{Variant, VariantType};

use std::rc::Rc;
use std::cell::RefCell;

// Array
#[derive(Debug)]
pub struct ArrayVariant {
    pub count_path: FieldReference,
    pub count: Option<WeakTypeContainer>,

    pub child: WeakTypeContainer,
    pub child_index: usize,
}
impl TypeVariant for ArrayVariant {
    default_resolve_child_name_impl!();
    default_get_result_type_impl!();

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::Array
    }

    fn has_property(&self, _data: &TypeData, name: &str) -> Option<TargetType> {
        match name {
            "length" => Some(TargetType::Integer),
            _ => None,
        }
    }

    fn do_resolve_references(&mut self, data: &mut TypeData,
                             resolver: &::ReferenceResolver) -> Result<()> {
        self.count = Some(resolver(self, data, &self.count_path)?);

        let count = self.count.clone().unwrap().upgrade().unwrap();
        let count_inner = count.borrow();
        let count_type = count_inner.variant.to_variant()
            .get_result_type(&count_inner.data);

        ensure!(count_type == Some(TargetType::Integer),
                "result type of reference is non-integer");

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
