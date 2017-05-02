mod builder;
mod ib_to_rs;
mod cu_to_rs;

use ::ir::compilation_unit::CompilationUnit;
use ::errors::*;

pub fn compile(cu: &CompilationUnit) -> Result<String> {
    let mut out = String::new();
    cu_to_rs::generate_compilation_unit(cu, &mut out)?;
    Ok(out)
}
