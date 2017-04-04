use ::{TypeVariant, TypeData, WeakTypeContainer, Result};
use ::ir::{TargetType, CompilePass};
use super::VariantType;
use ::context::compilation_unit::{CompilationUnit, TypePath};

#[derive(Debug)]
pub struct TerminatedBufferVariant {
}
impl TypeVariant for TerminatedBufferVariant {

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
