#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
extern crate json;
#[macro_use] extern crate nom;
extern crate rustache;

use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::any::Any;

pub mod ir;
pub use ir::{TypeContainer, WeakTypeContainer, Type, TypeData, ReferenceResolver, TypeVariant};

pub mod backend;
pub mod frontend;
mod field_reference;
use field_reference::FieldReference;
pub mod pass;

pub mod test_harness;

#[derive(Debug)]
/// Used to reference a specific property of another field.
pub struct FieldPropertyReference {
    pub reference: FieldReference,
    pub property: String,
}

pub mod errors {
    error_chain! {
        links {
            JsonParseError(
                ::frontend::protocol_json::Error,
                ::frontend::protocol_json::ErrorKind);
        }

        errors {
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
