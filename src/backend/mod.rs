use ::ir::compilation_unit::CompilationUnit;
use ::errors::*;

pub type Backend = fn(&CompilationUnit) -> Result<String>;

pub mod common;
pub mod imperative_base;

pub mod javascript;
pub mod rust;
pub mod python;
pub mod json_spec;
