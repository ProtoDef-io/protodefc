use std::rc::Rc;
use std::cell::RefCell;
use super::{Variant, VariantType};
use ::ir::{Type, TypeVariant, TypeData, Result, WeakTypeContainer, TypeContainer};
use ::ir::TargetType;
use ::context::compilation_unit::{CompilationUnit, TypePath};

/// This is a simple terminal scalar.
///
/// All types that take no special arguments and that have
/// no children should be represented by this variant.
///
/// It is up to the backend to generate code for the name of
/// the type.
#[derive(Debug)]
pub struct SimpleScalarVariant {
    pub target_type: Option<TargetType>,
}

impl TypeVariant for SimpleScalarVariant {
    fn get_type(&self, data: &TypeData) -> VariantType {
        VariantType::SimpleScalar(data.name.clone())
    }
    fn get_result_type(&self, _data: &TypeData) -> Option<TargetType> {
        self.target_type.clone()
    }
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
    default_resolve_on_context!();
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
                target_type: target_type,
            }),
        })
    }

}
