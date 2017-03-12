use super::CompilePass;
use super::Result;

use ::{Type, TypeContainer};

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct AssignIdentPass;

impl CompilePass for AssignIdentPass {
    fn run(typ: &mut TypeContainer) -> Result<()> {
        let mut ident_counter = 0;
        do_run(typ, &mut ident_counter)
    }
}

fn do_run(typ: &mut TypeContainer, current_id: &mut u64) -> Result<()> {
    let mut inner = typ.borrow_mut();
    inner.data.ident = Some(*current_id);

    *current_id += 1;

    for mut child in &mut inner.data.children {
        do_run(&mut child, current_id)?;
    }

    Ok(())
}
