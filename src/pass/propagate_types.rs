use super::Result;

use ::{Type, TypeContainer};
use ::ir::CompilePass;
use ::context::compilation_unit::{CompilationUnit, TypePath,
                                  TypeKind, NamedTypeContainer};

pub fn run(context: &CompilationUnit) -> Result<()> {
    let mut has_changed = true;

    while has_changed {
        has_changed = false;

        let mut pass = CompilePass::PropagateTypes {
            has_changed: &mut has_changed,
        };

        context.each_type(&mut |typ| {
            let borrowed = typ.borrow();

            if let TypeKind::Type(ref container) = borrowed.typ {
                super::traverse(container, &mut |typ| {
                    let mut inner = typ.borrow_mut();

                    use ::std::ops::DerefMut;
                    let Type { ref mut data, ref mut variant } = *inner.deref_mut();

                    variant.to_variant_mut()
                        .do_compile_pass(data, &mut pass)
                })
            } else {
                Ok(())
            }
        })?;
    }

    Ok(())
}
