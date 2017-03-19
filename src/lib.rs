#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
extern crate json;
#[macro_use] extern crate nom;
extern crate rustache;

use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::any::Any;

pub mod backend;
pub mod frontend;
pub mod variants;
mod field_reference;
use field_reference::FieldReference;
pub mod pass;

pub mod test_harness;

#[derive(Debug)]
pub struct Type {
    data: TypeData,
    variant: variants::Variant,
}

pub type TypeContainer = Rc<RefCell<Type>>;
pub type WeakTypeContainer = Weak<RefCell<Type>>;

#[derive(Debug)]
pub struct TypeData {
    name: String,
    children: Vec<TypeContainer>,

    /// Added in AssignParentPass
    parent: Option<WeakTypeContainer>,

    /// Added in AssignIdentPass
    /// Idents increase with causality.
    ident: Option<u64>,
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

type ReferenceResolver = Fn(&TypeVariant, &TypeData, &FieldReference)
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

pub mod errors {
    error_chain! {
        links {
            JsonParseError(
                ::frontend::protocol_json::Error,
                ::frontend::protocol_json::ErrorKind);
        }
    }
}
use errors::*;

pub fn json_to_final_ast(json: &str) -> Result<TypeContainer> {
    let mut tree = ::frontend::protocol_json::from_json(&json)?;

    use ::pass::CompilePass;
    ::pass::assign_parent::AssignParentPass::run(&mut tree)?;
    ::pass::assign_ident::AssignIdentPass::run(&mut tree)?;
    ::pass::resolve_reference::ResolveReferencePass::run(&mut tree)?;

    Ok(tree)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_from_json() {
        let input = "[\"container\", [{\"name\": \"foo\", \"type\": \"i8\"}]]";
        let res = ::json_to_final_ast(&input);
        use error_chain::ChainedError;
        match res {
            Ok(r) => println!("Ok: {:?}", r),
            _ => panic!(),
        }
    }

}
