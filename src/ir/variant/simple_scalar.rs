use std::rc::Rc;
use std::cell::RefCell;
use super::Variant;
use ::ir::{Type, TypeVariant, TypeData, Result, WeakTypeContainer, TypeContainer};

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
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
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
