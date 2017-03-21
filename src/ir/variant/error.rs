use ::ir::{Type, TypeVariant, TypeData, Result, WeakTypeContainer, TypeContainer};

#[derive(Debug)]
pub struct ErrorVariant {
    pub message: String,
}
impl TypeVariant for ErrorVariant {
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
}
