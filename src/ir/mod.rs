use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt::Debug;
use std::any::Any;
use std::fmt;

use ::errors::*;
use ::FieldReference;

pub mod variant;
mod debug_printer;

pub type TypeContainer = Rc<RefCell<Type>>;
pub type WeakTypeContainer = Weak<RefCell<Type>>;

pub struct Type {
    pub data: TypeData,
    pub variant: self::variant::Variant,
}

#[derive(Debug)]
pub struct TypeData {
    pub name: String,
    pub children: Vec<TypeContainer>,

    /// Added in AssignParentPass
    pub parent: Option<WeakTypeContainer>,

    /// Added in AssignIdentPass
    /// Idents increase with causality.
    pub ident: Option<u64>,
}

impl Default for TypeData {
    fn default() -> TypeData {
        TypeData {
            name: "".to_string(),
            children: Vec::new(),

            parent: None,
            ident: None,
        }
    }
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
    fn has_property(&self, data: &TypeData, prop_name: &str) -> bool;

    /// Used by the resolve_reference pass to fetch a named child
    /// for the given type.
    ///
    /// This should only be implemented for composite types, simple
    /// types should simply return an error here.
    fn resolve_child_name(&self, data: &TypeData, name: &str)
                          -> Result<WeakTypeContainer>;

    fn do_resolve_references(&mut self, data: &mut TypeData,
                             resolver: &ReferenceResolver) -> Result<()>;

}
