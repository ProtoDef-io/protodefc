use ::{TypeVariant, TypeData, WeakTypeContainer, Result};
use ::ir::TargetType;
use super::VariantType;

#[derive(Debug)]
pub struct TerminatedBufferVariant {
}
impl TypeVariant for TerminatedBufferVariant {

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::TerminatedBuffer
    }

    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
    default_get_result_type_impl!();
}
