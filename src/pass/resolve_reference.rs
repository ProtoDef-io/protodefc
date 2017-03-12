use super::CompilePass;
use super::Result;

use ::{Type, TypeContainer};

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct ResolveReferencePass;

impl CompilePass for ResolveReferencePass {
    fn run(typ: &mut TypeContainer) -> Result<()> {
        let mut parents: Vec<Weak<RefCell<Type>>> = Vec::new();
        do_run(typ, &mut parents)
    }
}

fn do_run(typ: &mut TypeContainer, parents: &mut Vec<Weak<RefCell<Type>>>) -> Result<()> {
    let mut children;
    {
        let mut inner = typ.borrow_mut();
        let ident = inner.data.ident.unwrap();

        let mut resolved_references: Vec<Weak<RefCell<Type>>> = Vec::new();
        for raw_reference in &inner.data.raw_references {
            if raw_reference.up >= parents.len() {
                bail!("reference goes up too far");
            }
            let root = &parents[parents.len() - 1 - raw_reference.up];
            let root_upgrade = root.upgrade().unwrap();
            let root_inner = root_upgrade.borrow_mut();

            let node = root_inner.variant.resolve_child_name(
                &root_inner.data, &raw_reference.name)?;

            {
                let node_upgrade = node.upgrade().unwrap();
                let node_inner = node_upgrade.borrow();
                if ident >= node_inner.data.ident.unwrap() {
                    bail!("reference to field defined after");
                }
            }

            resolved_references.push(node);
        }
        inner.data.references = Some(resolved_references);

        children = inner.data.children.clone();
    }

    parents.push(Rc::downgrade(typ));
    for mut child in &mut children {
        do_run(&mut child, parents)?;
    }
    parents.pop();

    Ok(())
}
