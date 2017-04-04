use super::Result;

use ::{Type, TypeContainer, TypeVariant, TypeData, WeakTypeContainer, CompilerError};
use ::errors::*;
use ::FieldReference;
use ::context::compilation_unit::{NamedType, CompilationUnit, NSPath, TypePath};

struct ResolveContextState<'a> {
    compilation_unit: &'a CompilationUnit,
    current_path: &'a TypePath,
}

pub fn run(typ: &NamedType, compilation_unit: &CompilationUnit) -> Result<()> {
    run_inner(&typ.typ, &ResolveContextState {
        compilation_unit: compilation_unit,
        current_path: &typ.path,
    })
}

fn run_inner(typ: &TypeContainer, state: &ResolveContextState)
                 -> Result<()> {
    let mut children;
    {
        let mut inner = typ.borrow_mut();
        children = inner.data.children.clone();

        inner.variant.to_variant().resolve_on_context(
            &inner.data, state.current_path, state.compilation_unit)?;
    }

    for mut child in &mut children {
        run_inner(child, state)?;
    }

    Ok(())
}
