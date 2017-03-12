#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub mod backend;
pub mod frontend;
pub mod variants;
mod field_reference;
pub mod pass;

#[derive(Debug)]
pub struct Type {
    data: TypeData,
    variant: Box<TypeVariant>,
}

pub type TypeContainer = Rc<RefCell<Type>>;
pub type WeakTypeContainer = Weak<RefCell<Type>>;

#[derive(Debug)]
pub struct TypeData {
    name: String,
    children: Vec<TypeContainer>,
    raw_references: Vec<field_reference::FieldReference>,

    /// Added in AssignParentPass
    parent: Option<Weak<RefCell<Type>>>,

    /// Added in AssignIdentPass
    /// Idents increase with causality.
    ident: Option<u64>,

    // Added in ResolveReferencePass
    references: Option<Vec<Weak<RefCell<Type>>>>,
}

impl Default for TypeData {
    fn default() -> TypeData {
        TypeData {
            name: "".to_string(),
            children: Vec::new(),
            raw_references: Vec::new(),

            parent: None,
            ident: None,
            references: None,
        }
    }
}

pub trait TypeVariant: Debug {

    fn resolve_child_name(&self, data: &TypeData, name: &str) -> Result<Weak<RefCell<Type>>>;

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
            Err(err) => println!("Err: {}", err.display()),
        }
        panic!();
    }

}
