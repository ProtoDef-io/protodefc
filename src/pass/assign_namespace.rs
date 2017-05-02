use ::errors::*;

use ::ir::spec::{Type, CompilePass};
use ::ir::compilation_unit::CompilationUnit;

pub fn run(cu: &CompilationUnit) -> Result<()> {
    cu.each_type_traverse_node(&mut |typ, node| {
        let typ_inner = typ.borrow();

        let mut inner = node.borrow_mut();
        use ::std::ops::DerefMut;
        let Type { ref mut data, ref mut variant } = *inner.deref_mut();

        variant.to_variant_mut().do_compile_pass(
            data, &mut CompilePass::AssignNamespace(&typ_inner.path))?;

        Ok(())
    })
}
