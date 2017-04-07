use ::errors::*;
use ::ir::{TargetType, FieldReference};
use ::ir::typ::{TypeVariant, TypeData, WeakTypeContainer, CompilePass};
use super::VariantType;
use ::ir::compilation_unit::{CompilationUnit, TypePath};

#[derive(Debug)]
pub struct SizedBufferVariant {
    count_ref: FieldReference,
}
impl TypeVariant for SizedBufferVariant {

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::TerminatedBuffer
    }

    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_get_result_type_impl!();

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass)
                       -> Result<()> {
        Ok(())
    }
}
