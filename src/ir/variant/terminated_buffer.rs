use ::{TypeVariant, TypeData, Type, WeakTypeContainer, Result, TypeContainer};
use ::field_reference::FieldReference;

#[derive(Debug)]
pub struct TerminatedBufferVariant {
}
impl TypeVariant for TerminatedBufferVariant {
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
}
