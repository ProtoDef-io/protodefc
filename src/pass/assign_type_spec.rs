use ::errors::*;
use ::ir::spec::{Type, TypeContainer, CompilePass};
use ::ir::compilation_unit::{CompilationUnit, TypeKind};

pub fn run(cu: &CompilationUnit) -> Result<()> {
    cu.each_type(&mut |typ_container| {
        let named_typ = typ_container.borrow();

        match named_typ.typ {
            TypeKind::Type(ref typ) => do_run_type(typ)?,
            _ => ()
        }

        Ok(())
    })
}

fn do_run_type(typ: &TypeContainer) -> Result<()> {
    let mut inner = typ.borrow_mut();

    for child in inner.data.get_children().iter() {
        do_run_type(&child)?;
    }

    use ::std::ops::DerefMut;
    let Type { ref mut data, ref mut variant } = *inner.deref_mut();

    variant.to_variant_mut().do_compile_pass(data, &mut CompilePass::MakeTypeSpecs)?;

    Ok(())
}
