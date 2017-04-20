mod builder;

pub mod ib_to_js;
pub mod cu_to_js;

#[cfg(all(test, feature = "js_tests"))]
mod tests;

use ::ir::compilation_unit::CompilationUnit;
use ::errors::*;

use self::builder::ToJavascript;

pub fn compilation_unit_to_javascript(cu: &CompilationUnit) -> Result<String> {
    let block = cu_to_js::generate_compilation_unit(cu)?;
    let mut out = String::new();
    block.to_javascript(&mut out, 0);
    Ok(out)
}
