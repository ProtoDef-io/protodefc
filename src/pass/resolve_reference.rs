use super::CompilePass;
use super::Result;

use ::{Type, TypeContainer, TypeVariant, TypeData, WeakTypeContainer};
use ::field_reference::FieldReference;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct ResolveReferencePass;

impl CompilePass for ResolveReferencePass {
    fn run(typ: &mut TypeContainer) -> Result<()> {
        let mut parents: Vec<Weak<RefCell<Type>>> = Vec::new();
        do_run(typ, &mut parents)
    }
}

fn do_run(typ: &TypeContainer, parents: &mut Vec<Weak<RefCell<Type>>>)
          -> Result<()> {

    let parents_o = parents.clone();
    let resolver: &Fn(&TypeVariant, &TypeData, &FieldReference)
                      -> Result<WeakTypeContainer> =
        &move |variant, data, reference| {
        if reference.up == 0 {
            variant.resolve_child_name(data, &reference.name)
        } else {
            if reference.up >= parents_o.len() {
                bail!("reference goes up too far");
            }
            let root = &parents_o[parents_o.len() - 1 - reference.up];
            let root_upgrade = root.upgrade().unwrap();
            let root_inner = root_upgrade.borrow_mut();

            let Type { ref data, ref variant } = *root_inner;
            variant.to_variant().resolve_child_name(data, &reference.name)
        }
    };

    let mut children;
    {
        let mut inner = typ.borrow_mut();
        children = inner.data.children.clone();

        let Type { ref mut data, ref mut variant } = *inner;
        variant.to_variant_mut()
            .do_resolve_references(data, resolver)?;
    }

    parents.push(Rc::downgrade(typ));
    for mut child in &mut children {
        do_run(&mut child, parents)?;
    }
    parents.pop();

    Ok(())
}
