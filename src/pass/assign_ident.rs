use super::Result;

use ::TypeContainer;
use ::ir::compilation_unit::{CompilationUnit, TypeKind};

pub fn run(cu: &CompilationUnit) -> Result<()> {
    let mut ident_counter = 0;

    cu.each_type(&mut |named_typ| {
        let named_typ_inner = named_typ.borrow();
        if let TypeKind::Type(ref root_node) = named_typ_inner.typ {
            do_run(root_node, &mut ident_counter)?;
        }
        Ok(())
    })
}

fn do_run(typ: &TypeContainer, current_id: &mut u64) -> Result<()> {
    let mut inner = typ.borrow_mut();

    for mut child in &mut inner.data.get_children().iter() {
        do_run(&mut child, current_id)?;
    }

    *current_id += 1;
    inner.data.ident = Some(*current_id);

    Ok(())
}
