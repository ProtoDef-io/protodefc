mod builder;

pub mod ib_to_py;
pub mod cu_to_py;

use ::ir::compilation_unit::CompilationUnit;
use ::errors::*;

use self::builder::ToPython;

pub fn compile(cu: &CompilationUnit) -> Result<String> {
    let block = cu_to_py::generate_compilation_unit(cu)?;
    let mut out = String::new();
    block.to_python(&mut out, 0);
    Ok(out)
}
