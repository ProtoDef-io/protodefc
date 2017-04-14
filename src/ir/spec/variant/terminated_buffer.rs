use ::errors::*;
use ::ir::TargetType;
use ::ir::spec::{TypeVariant, TypeData, WeakTypeContainer, CompilePass};
use ::ir::spec::variant::VariantType;
use ::ir::type_spec::{TypeSpecContainer, TypeSpecVariant, WeakTypeSpecContainer};
use ::ir::compilation_unit::{CompilationUnit, TypePath};

#[derive(Debug)]
pub struct TerminatedBufferVariant {
}
impl TypeVariant for TerminatedBufferVariant {

    fn get_type(&self, _data: &TypeData) -> VariantType {
        VariantType::TerminatedBuffer
    }

    default_resolve_child_name_impl!();
    default_has_property_impl!();

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass)
                       -> Result<()> {
        match *pass {
            CompilePass::MakeTypeSpecs => {
                unimplemented!();
            }
            _ => Ok(()),
        }
    }

}
