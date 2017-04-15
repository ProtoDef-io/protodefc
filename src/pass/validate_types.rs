use ::errors::*;
use ::ir::compilation_unit::CompilationUnit;
use ::ir::spec::{Type, CompilePass};

pub fn run(cu: &mut CompilationUnit) -> Result<()> {
    cu.each_type_traverse_node(&mut |_, node| {
        let mut node_inner = node.borrow_mut();

        use ::std::ops::DerefMut;
        let Type { ref mut data, ref mut variant } = *node_inner.deref_mut();

        let mut pass = CompilePass::ValidateTypes;
        variant.to_variant_mut().do_compile_pass(data, &mut pass)
    })
}
