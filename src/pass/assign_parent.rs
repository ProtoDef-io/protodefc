use super::CompilePass;
use super::Result;

use ::{Type, TypeContainer};

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct AssignParentPass;

impl CompilePass for AssignParentPass {
    fn run(typ: &mut TypeContainer) -> Result<()> {
        do_run(typ, None)
    }
}

fn do_run(typ: &mut TypeContainer, parent: Option<Weak<RefCell<Type>>>) -> Result<()> {
    let mut inner = typ.borrow_mut();
    inner.data.parent = parent;

    for mut child in &mut inner.data.children {
        do_run(&mut child, Some(Rc::downgrade(typ)))?;
    }

    Ok(())
}
