use std::rc::Rc;
use std::cell::RefCell;
use ::ir::TargetType;
use ::ir::typ::{Type, TypeVariant, TypeData, WeakTypeContainer, TypeContainer, CompilePass};
use ::ir::typ::variant::{Variant, VariantType};
use ::ir::compilation_unit::{CompilationUnit, TypePath, NamedTypeContainer};
use ::errors::*;

/// This is a simple terminal scalar.
///
/// All types that take no special arguments and that have
/// no children should be represented by this variant.
///
/// It is up to the backend to generate code for the name of
/// the type.
#[derive(Debug)]
pub struct SimpleScalarVariant {
    pub target: Option<NamedTypeContainer>,
    pub target_type: Option<TargetType>,
}

impl TypeVariant for SimpleScalarVariant {
    fn get_type(&self, data: &TypeData) -> VariantType {
        VariantType::SimpleScalar(data.name.clone())
    }
    fn get_result_type(&self, _data: &TypeData) -> Option<TargetType> {
        self.target_type
    }
    default_resolve_child_name_impl!();
    default_has_property_impl!();

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass)
                       -> Result<()> {
        match *pass {
            CompilePass::ResolveReferencedTypes(ref path, ref cu) => {
                let target_resolved = cu.resolve_type(
                    &data.name.in_context(&path.path))
                    .chain_err(|| format!("while resolving type of simple_scalar"))?;
                self.target = Some(target_resolved);

                Ok(())
            }
            CompilePass::PropagateTypes { ref mut has_changed } => {
                let target_named_type = self.target.clone().unwrap();
                let target_named_type_inner = target_named_type.borrow();
                let result_type = target_named_type_inner.typ.get_result_type();

                if result_type != self.target_type {
                    **has_changed = true;
                }

                self.target_type = result_type;

                Ok(())

            }
            _ => Ok(()),
        }
    }
}

impl SimpleScalarVariant {

    pub fn new(path: TypePath) -> TypeContainer {
        SimpleScalarVariant::with_target_type(path, None)
    }

    pub fn with_target_type(path: TypePath, target_type: Option<TargetType>)
                            -> TypeContainer {
        let mut data = TypeData::default();
        data.name = path;

        TypeContainer::new(Type {
            data: data,
            variant: Variant::SimpleScalar(SimpleScalarVariant {
                target: None,
                target_type: target_type,
            }),
        })
    }

}
