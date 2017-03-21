use ::{TypeVariant, TypeData, Type, WeakTypeContainer, Result, TypeContainer};
use ::field_reference::FieldReference;

#[derive(Debug)]
pub struct SizedBufferVariant {
    count_ref: FieldReference,
}
impl TypeVariant for SizedBufferVariant {
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
}
