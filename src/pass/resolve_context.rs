use ::ir::typ::{Type, TypeContainer, CompilePass};
use ::ir::compilation_unit::{CompilationUnit, TypePath,
                             TypeKind, NamedTypeContainer};
use ::errors::*;

pub fn run(cu: &CompilationUnit) -> Result<()> {
    cu.each_type_traverse_node(&mut |ref named_type, ref node| {
        let named_type_inner = named_type.borrow();
        let mut node_inner = node.borrow_mut();

        use ::std::ops::DerefMut;
        let Type { ref mut data, ref mut variant } = *node_inner.deref_mut();

        let mut pass = CompilePass::ResolveReferencedTypes(
            &named_type_inner.path, cu);

        variant.to_variant_mut()
            .do_compile_pass(data, &mut pass)
    })
}
