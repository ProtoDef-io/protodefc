use ::errors::Result;
use ::ir::spec::{Type, TypeContainer, WeakTypeContainer};
use ::ir::compilation_unit::{CompilationUnit, TypeKind};

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub fn run(cu: &CompilationUnit) -> Result<()> {
    cu.each_type(&mut |named_typ| {
        let named_typ_inner = named_typ.borrow();
        if let TypeKind::Type(ref root_node) = named_typ_inner.typ {
            do_run(root_node, None)?;
        }
        Ok(())
    })
}

fn do_run(typ: &TypeContainer, parent: Option<WeakTypeContainer>) -> Result<()> {
    let mut inner = typ.borrow_mut();
    inner.data.parent = parent;

    for mut child in &mut inner.data.get_children().iter() {
        do_run(&mut child, Some(typ.downgrade()))?;
    }

    Ok(())
}
