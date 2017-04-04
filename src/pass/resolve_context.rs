use super::Result;

use ::{Type, TypeContainer};
use ::ir::CompilePass;
use ::context::compilation_unit::{CompilationUnit, TypePath,
                                  TypeKind, NamedTypeContainer};

pub fn run(typ: &NamedTypeContainer, compilation_unit: &CompilationUnit) -> Result<()> {
    let borrowed = typ.borrow();

    if let TypeKind::Type(ref container) = borrowed.typ {
        super::traverse(container, &mut |typ| {
            let mut inner = typ.borrow_mut();

            use ::std::ops::DerefMut;
            let Type { ref mut data, ref mut variant } = *inner.deref_mut();

            let mut pass = CompilePass::ResolveReferencedTypes(
                &borrowed.path, compilation_unit);

            variant.to_variant_mut()
                .do_compile_pass(data, &mut pass)
        })?;
    }

    Ok(())
}
