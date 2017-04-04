use ::ir::{TypeVariant, TypeData, Result, WeakTypeContainer};
use ::context::compilation_unit::{CompilationUnit, TypePath};
use ::ir::{TargetType, CompilePass};
use super::VariantType;

#[derive(Debug)]
pub struct ErrorVariant {
    pub message: String,
}
impl TypeVariant for ErrorVariant {

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::Error
    }

    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_get_result_type_impl!();

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass)
                       -> Result<()> {
        Ok(())
    }
}
