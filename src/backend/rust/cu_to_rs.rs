use ::errors::*;
use ::ir::compilation_unit::{CompilationUnit, TypeKind};
use ::ir::spec::TypeContainer;
use super::builder::*;
use ::backend::imperative_base as ib;

fn generate_serialize(fun_name: String, typ: TypeContainer) -> Result<Block> {
    let base = ib::serialize::generate_serialize(typ.clone())?;

    let mut ib = Block::new();
    ib.inline(super::ib_to_rs::build_block(&base)?);

    let mut b = Block::new();
    b.decl_fun(
        format!("{}({}: TODO)", fun_name, ib::utils::input_for_type(&typ)),
        ib
    );
    Ok(b)
}

pub fn generate_compilation_unit(cu: &CompilationUnit, out: &mut String)
                                 -> Result<()> {
    let mut b = Block::new();

    let types = cu.namespaces.iter().flat_map(|ns| {
        ns.types.iter()
    });

    for typ in types {
        let typ_inner = typ.borrow();

        let typ_base_name = format!("type_{}", typ_inner.type_id);
        let typ_name_size_of = format!("{}_size_of", typ_base_name);
        let typ_name_serialize = format!("{}_serialize", typ_base_name);
        let typ_name_deserialize = format!("{}_deserialize", typ_base_name);

        match typ_inner.typ {
            TypeKind::Type(ref type_container) => {
                b.inline(generate_serialize(
                    typ_name_serialize.clone(),
                    type_container.clone()
                )?);
            }
            _ => (),
        }
    }

    b.to_rust(out, 0);
    Ok(())
}
