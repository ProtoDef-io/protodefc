#![recursion_limit = "1024"]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate error_chain;
extern crate json;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate nom;
extern crate rustache;
extern crate itertools;
#[macro_use] extern crate matches;
extern crate regex;
extern crate inflector;
extern crate num_bigint;

pub mod ir;
pub mod backend;
pub mod frontend;
pub mod pass;
pub mod test_harness;
pub mod errors;
pub mod rc_container;
pub mod old_protocol_json_to_pds;

use errors::*;

use ir::spec::TypeContainer;
use ir::compilation_unit::CompilationUnit;

pub fn spec_to_final_compilation_unit(spec: &str) -> Result<CompilationUnit> {
    let mut unit = ::frontend::protocol_spec::to_compilation_unit(spec)?;
    unit.compile_types()?;
    Ok(unit)
}

#[cfg(test)]
mod test {

    //#[test]
    //#fn test_from_json() {
    //#   let input = "[\"container\", [{\"name\": \"foo\", \"type\": \"i8\"}]]";
    //#   let res = ::json_to_final_ast(&input);
    //#   match res {
    //#       Ok(r) => println!("Ok: {:?}", r),
    //#       _ => panic!(),
    //#   }
    //

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
