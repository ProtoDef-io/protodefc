use ::errors::*;

use ::ir::typ::{Type, TypeContainer, CompilePass};
use ::ir::compilation_unit::{CompilationUnit, TypePath,
                             TypeKind, NamedTypeContainer};

pub fn run(cu: &CompilationUnit) -> Result<()> {
    let mut has_changed = true;

    while has_changed {
        has_changed = false;

        let mut pass = CompilePass::PropagateTypes {
            has_changed: &mut has_changed,
        };

        cu.each_type_traverse_node(&mut |_, node| {
            let mut node_inner = node.borrow_mut();

            use ::std::ops::DerefMut;
            let Type { ref mut data, ref mut variant } = *node_inner.deref_mut();

            variant.to_variant_mut()
                .do_compile_pass(data, &mut pass)
        })?
    }

    Ok(())
}
