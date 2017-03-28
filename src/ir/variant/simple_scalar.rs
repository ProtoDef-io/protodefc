use std::rc::Rc;
use std::cell::RefCell;
use super::{Variant, VariantType};
use ::ir::{Type, TypeVariant, TypeData, Result, WeakTypeContainer, TypeContainer};
use ::ir::TargetType;

/// This is a simple terminal scalar.
///
/// All types that take no special arguments and that have
/// no children should be represented by this variant.
///
/// It is up to the backend to generate code for the name of
/// the type.
#[derive(Debug)]
pub struct SimpleScalarVariant {}
impl TypeVariant for SimpleScalarVariant {
    fn get_type(&self, data: &TypeData) -> VariantType {
        VariantType::SimpleScalar(data.name.clone())
    }
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
    default_get_result_type_impl!();
}
impl SimpleScalarVariant {
    pub fn new(name: String) -> TypeContainer {
        let mut data = TypeData::default();
        data.name = name;

        Rc::new(RefCell::new(Type {
            data: data,
            variant: Variant::SimpleScalar(SimpleScalarVariant {}),
        }))
    }
}
