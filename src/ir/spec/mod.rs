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
use ::ir::{FieldReference, TargetType};
use ::ir::compilation_unit::{TypePath, CompilationUnit};

pub type TypeContainer = Container<Type>;
pub type WeakTypeContainer = WeakContainer<Type>;

pub struct Type {
    pub data: TypeData,
    pub variant: self::variant::Variant,
}

pub type ReferenceResolver = Fn(&TypeVariant, &TypeData, &FieldReference)
                                -> Result<WeakTypeContainer>;

/// Every primitive type supported by the compiler needs to
/// implement this trait. It is used by compiler passes/backends
/// to get details on how the type should function.
pub trait TypeVariant: Debug + Any {
    /// Used by the compiler to check whether it is legal to refer
    /// to a property of a given type variant.
    ///
    /// This is used by virtual container fields to get their value
    /// when writing.
    fn has_property(&self, data: &TypeData, prop_name: &str) -> Option<TargetType>;

    /// Used by the compiler to determine if the type can be used

    fn get_result_type(&self, data: &TypeData) -> Option<TargetType>;

    /// Used by the resolve_reference pass to fetch a named child
    /// for the given type.
    ///
    /// This should only be implemented for composite types, simple
    /// types should simply return an error here.
    fn resolve_child_name(&self, data: &TypeData, name: &str) -> Result<WeakTypeContainer>;

    // fn do_resolve_references(&mut self, data: &mut TypeData,
    //                         resolver: &ReferenceResolver) -> Result<()>;

    fn get_type(&self, data: &TypeData) -> VariantType;

    // fn resolve_on_context(&mut self, data: &TypeData, current_path: &TypePath,
    //                      context: &CompilationUnit) -> Result<()>;

    fn do_compile_pass(&mut self, data: &mut TypeData, pass: &mut CompilePass) -> Result<()>;
}

pub enum CompilePass<'a> {
    ResolveReferencedTypes(&'a TypePath, &'a CompilationUnit),
    ResolveInternalReferences(&'a ReferenceResolver),
    PropagateTypes { has_changed: &'a mut bool },
}
