use ::errors::*;

use ::ir::compilation_unit::CompilationUnit;
use ::ir::spec::{Type, CompilePass};

pub fn run(cu: &CompilationUnit) -> Result<()> {
    cu.each_type_traverse_node(&mut |_, node| {
        let mut inner = node.borrow_mut();

        use ::std::ops::DerefMut;
        let Type { ref mut data, ref mut variant } = *inner.deref_mut();

        let mut pass = CompilePass::GenerateFieldAccessOrder;
        variant.to_variant_mut()
            .do_compile_pass(data, &mut pass)?;

        Ok(())
    })
}
