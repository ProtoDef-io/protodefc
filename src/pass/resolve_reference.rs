use super::Result;

use ::{Type, TypeContainer, TypeVariant, TypeData, WeakTypeContainer, CompilerError};
use ::ir::CompilePass;
use ::errors::*;
use ::FieldReference;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub fn run(typ: &TypeContainer) -> Result<()> {
    let mut parents: Vec<WeakTypeContainer> = Vec::new();
    do_run(typ, &mut parents)
}

fn do_run(typ: &TypeContainer, parents: &mut Vec<WeakTypeContainer>)
          -> Result<()> {

    let parents_o = parents.clone();
    let resolver: &Fn(&TypeVariant, &TypeData, &FieldReference)
                      -> Result<WeakTypeContainer> =
        &move |variant, data, reference| {
            let chain = || CompilerError::ReferenceError {
                reference: reference.clone(),
            };

            if reference.up == 0 {
                variant.resolve_child_name(data, &reference.name)
                    .chain_err(chain)
            } else {
                if reference.up > parents_o.len() {
                    bail!(chain());
                }
                let root = &parents_o[parents_o.len() - 1 - (reference.up - 1)];
                let root_upgrade = root.upgrade();
                let root_inner = root_upgrade.borrow_mut();

                let Type { ref data, ref variant } = *root_inner;
                variant.to_variant().resolve_child_name(data, &reference.name)
                    .chain_err(chain)
            }
    };

    let chain;
    let mut children;
    {
        let mut inner = typ.borrow_mut();
        children = inner.data.children.clone();

        chain = CompilerError::InsideVariant {
            variant: inner.variant.get_type(&inner.data),
        };

        let Type { ref mut data, ref mut variant } = *inner;

        let mut pass = CompilePass::ResolveInternalReferences(resolver);
        variant.to_variant_mut()
            .do_compile_pass(data, &mut pass)
            .chain_err(|| chain.clone())?;
    }

    parents.push(typ.downgrade());
    for mut child in &mut children {
        do_run(&mut child, parents)
            .chain_err(|| chain.clone())?;
    }
    parents.pop();

    Ok(())
}
