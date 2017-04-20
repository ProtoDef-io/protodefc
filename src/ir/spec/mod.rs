use ::errors::*;

pub mod variant;
use self::variant::VariantType;

pub mod reference;

pub mod data;
pub use self::data::TypeData;

mod debug_printer;

use std::any::Any;
use std::fmt::Debug;

use ::rc_container::{Container, WeakContainer};
use ::ir::compilation_unit::{TypePath, CompilationUnit};
use ::ir::type_spec::WeakTypeSpecContainer;
use ::ir::name::Name;

pub type TypeContainer = Container<Type>;
pub type WeakTypeContainer = WeakContainer<Type>;

pub struct Type {
    pub data: TypeData,
    pub variant: self::variant::Variant,
}

/// Every primitive type supported by the compiler needs to
/// implement this trait. It is used by compiler passes/backends
/// to get details on how the type should function.
pub trait TypeVariant: Debug + Any {

    fn has_spec_property(&self, data: &TypeData, prop_name: &Name)
                         -> Result<Option<WeakTypeSpecContainer>>;

    /// Used by the resolve_reference pass to fetch a named child
    /// for the given type.
    ///
    /// This should only be implemented for composite types, simple
    /// types should simply return an error here.
    fn resolve_child_name(&self, data: &TypeData, name: &Name)
                          -> Result<WeakTypeContainer>;

    fn get_type(&self, data: &TypeData) -> VariantType;

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass) -> Result<()>;

}

pub enum CompilePass<'a> {
    ResolveReferencedTypes(&'a TypePath, &'a CompilationUnit),
    MakeTypeSpecs,
    GenerateFieldAccessOrder,
    ValidateTypes,
}
