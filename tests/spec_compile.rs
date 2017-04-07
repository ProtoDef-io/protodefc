extern crate protodefc;
extern crate error_chain;

use protodefc::ir::typ::TypeContainer;
use protodefc::ir::compilation_unit::CompilationUnit;
use protodefc::errors::*;

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

fn compile(spec: &str) -> Result<CompilationUnit> {
    protodefc::spec_to_final_compilation_unit(spec)
}

#[test]
fn simple_container() {
    unwrap_ok!(compile(r#"
@type "integer"
def_native("u8");

def("test") => container {
    field("field_1") => u8;
};
"#));
}

#[test]
fn container_virtual_field() {
    unwrap_ok!(compile(r#"
@type "integer"
def_native("u8");

def("test") => container {
    virtual_field("field_1", ref: "field_2", prop: "length") => u8;
    field("field_2") => array(ref: "../field_1") => u8;
};
"#));
}

#[test]
fn container_virtual_field_nonexistent_ref() {
    unwrap_error!(compile(r#"
@type "integer"
def_native("u8");

def("test") => container {
    virtual_field("field_1", ref: "field_2", prop: "length") => u8;
};
"#));
}

#[test]
fn parse_error() {
    unwrap_error!(compile(" ofajsdfj => ;"));
}
