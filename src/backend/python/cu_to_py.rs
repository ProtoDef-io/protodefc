use ::errors::*;
use ::ir::compilation_unit::{CompilationUnit, TypeKind};
use ::ir::spec::TypeContainer;
use super::builder::Block;
use itertools::Itertools;
use ::backend::imperative_base as ib;

fn generate_serialize(fun_name: String, typ: TypeContainer) -> Result<Block> {
    let base = ib::serialize::generate_serialize(typ.clone())?;

    let mut ib = Block::new();

    {
        let typ_inner = typ.borrow();
        ib.assign(ib::utils::input_for(&typ_inner.data), "input".into());
    }

    ib.block(super::ib_to_py::build_block(&base)?);
    ib.return_("offset".into());

    let mut b = Block::new();
    b.decl_fun(
        fun_name,
        vec!["input".into(), "buffer".into(), "offset".into()],
        ib
    );
    Ok(b)
}

pub fn generate_size_of(fun_name: String, typ: TypeContainer) -> Result<Block> {
    let base = ib::size_of::generate_size_of(typ.clone())?;

    let mut ib = Block::new();
    ib.assign("count".into(), "0".into());

    {
        let typ_inner = typ.borrow();
        ib.assign(ib::utils::input_for(&typ_inner.data), "input".into());
    }

    ib.block(super::ib_to_py::build_block(&base)?);
    ib.return_("count".into());

    let mut b = Block::new();
    b.decl_fun(fun_name, vec!["input".into()], ib);
    Ok(b)
}

pub fn generate_deserialize(fun_name: String, typ: TypeContainer) -> Result<Block> {
    let base = ib::deserialize::generate_deserialize(typ.clone())?;

    let mut ib = Block::new();

    ib.block(super::ib_to_py::build_block(&base)?);
    ib.return_(format!("({}, offset)",
                       ib::utils::output_for_type(&typ.clone())).into());

    let mut b = Block::new();
    b.decl_fun(
        fun_name,
        vec!["buffer".into(), "offset".into()],
        ib
    );
    Ok(b)
}

pub fn generate_compilation_unit(cu: &CompilationUnit) -> Result<Block> {
    let mut b = Block::new();

    let types = cu.namespaces.iter().flat_map(|ns| {
        ns.specs_iter()
    });

    let mut exports: Vec<(String, String)> = Vec::new();

    for typ in types {
        let typ_inner = typ.borrow();

        let typ_base_name = format!("type_{}", typ_inner.type_id);
        let typ_name_size_of = format!("{}_size_of", typ_base_name);
        let typ_name_serialize = format!("{}_serialize", typ_base_name);
        let typ_name_deserialize = format!("{}_deserialize", typ_base_name);

        let typ_ns_name = format!("{}", typ_inner.path);
        if let Some(ref name) = typ_inner.export {
            exports.push((name.clone(), typ_base_name.clone()));
        }

        match typ_inner.typ {
            TypeKind::Type(ref type_container) => {
                b.block(generate_size_of(
                    typ_name_size_of.clone(),
                    type_container.clone()
                )?);
                b.block(generate_serialize(
                    typ_name_serialize.clone(),
                    type_container.clone()
                )?);
                b.block(generate_deserialize(
                    typ_name_deserialize.clone(),
                    type_container.clone()
                )?);
            }
            TypeKind::Native(_) => {
                b.assign(
                    typ_name_size_of.clone(),
                    format!("types[\"{}\"][\"size_of\"]", typ_ns_name).into()
                );
                b.assign(
                    typ_name_serialize.clone(),
                    format!("types[\"{}\"][\"serialize\"]", typ_ns_name).into()
                );
                b.assign(
                    typ_name_deserialize.clone(),
                    format!("types[\"{}\"][\"deserialize\"]", typ_ns_name).into()
                );
            }
        }
    }

    let exports_inner = exports.iter()
        .map(|&(ref ns_path, ref internal_name)| {
            format!("\"{}\": {{\"size_of\": {i}_size_of, \"serialize\": {i}_serialize, \"deserialize\": {i}_deserialize }}",
                    ns_path, i = internal_name)
        })
        .join(",\n");
    b.assign("exports".into(), format!("{{\n{}\n}}", exports_inner).into());

    Ok(b)
}
