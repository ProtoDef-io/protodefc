use ::errors::*;
use ::ir::spec::{Type, TypeContainer, CompilePass};
use ::ir::compilation_unit::{CompilationUnit, TypeKind, DefinedItemType};
use ::ir::type_spec::TypeSpecVariant;

pub fn run(cu: &CompilationUnit) -> Result<()> {
    cu.each_type(&mut |typ_container| {
        let named_typ = match typ_container.item {
            DefinedItemType::Spec(ref inner) => inner.borrow(),
        };

        let type_spec_rc = named_typ.type_spec.clone();
        let mut type_spec = type_spec_rc.borrow_mut();

        match named_typ.typ {
            TypeKind::Type(ref typ) => {
                do_run_type(typ)?;
                type_spec.variant = TypeSpecVariant::Referenced(
                    Some(typ.borrow().data.type_spec.clone().unwrap().downgrade()))
            }
            TypeKind::Native(ref inner) => {
                type_spec.variant = TypeSpecVariant::Referenced(
                    Some(inner.type_spec.clone().downgrade()));
            }
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
