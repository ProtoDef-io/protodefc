use super::Result;

use ::{Type, TypeContainer, WeakTypeContainer};

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub fn run(typ: &TypeContainer) -> Result<()> {
    do_run(typ, None)
}

fn do_run(typ: &TypeContainer, parent: Option<WeakTypeContainer>) -> Result<()> {
    let mut inner = typ.borrow_mut();
    inner.data.parent = parent;

    for mut child in &mut inner.data.children {
        do_run(&mut child, Some(typ.downgrade()))?;
    }

    Ok(())
}
