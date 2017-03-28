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
pub use ir::{FieldPropertyReference, FieldReference};

pub mod backend;
pub mod frontend;
pub mod pass;

pub mod test_harness;

pub mod errors;
use errors::*;

pub fn run_passes(ir: &mut TypeContainer) -> Result<()> {
    use ::pass::CompilePass;
    ::pass::assign_parent::AssignParentPass::run(ir)?;
    ::pass::assign_ident::AssignIdentPass::run(ir)?;
    ::pass::resolve_reference::ResolveReferencePass::run(ir)?;
    Ok(())
}

pub fn json_to_final_ast(json: &str) -> Result<TypeContainer> {
    let mut tree = ::frontend::protocol_json::from_json(&json)?;
    run_passes(&mut tree)?;
    Ok(tree)
}

pub fn spec_type_to_final_ast(spec: &str) -> Result<TypeContainer> {
    let ast = ::frontend::protocol_spec::parse(spec)?;
    let mut ir = ::frontend::protocol_spec::type_def_to_ir(&ast.statements[0])?;
    ::run_passes(&mut ir)?;
    Ok(ir)
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

macro_rules! unwrap_ok {
    ($e:expr) => {
        match $e {
            Ok(inner) => inner,
            Err(err) => {
                use error_chain::ChainedError;
                panic!("Expected Ok, got Err:\n{}", err.display());
            },
        }
    }
}

macro_rules! unwrap_error {
    ($e:expr) => {
        match $e {
            Ok(inner) => {
                panic!("Expected Err, got Ok:\n{:?}", inner);
            },
            Err(inner) => inner,
        }
    }
}

