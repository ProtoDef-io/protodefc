use ::errors::*;
use ::context::compilation_unit::{CompilationUnit, TypeKind};
use super::builder::Block;
use itertools::Itertools;

use super::size_of::generate_size_of;
use super::serialize::generate_serialize;
use super::deserialize::generate_deserialize;

pub fn generate_compilation_unit(cu: CompilationUnit) -> Result<Block> {
    let mut b = Block::new();

    let types = cu.namespaces.iter().flat_map(|ns| {
        ns.types.iter()
    });

    let mut exports: Vec<(String, String)> = Vec::new();

    for typ in types {
        let typ_inner = typ.borrow();

        let typ_base_name = format!("type_{}", typ_inner.type_id);
        let typ_name_size_of = format!("{}_size_of", typ_base_name);
        let typ_name_serialize = format!("{}_serialize", typ_base_name);
        let typ_name_deserialize = format!("{}_deserialize", typ_base_name);

        let typ_ns_name = format!("{}", typ_inner.path);
        exports.push((typ_ns_name.clone(), typ_base_name.clone()));

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
                b.var_assign(
                    typ_name_size_of.clone(),
                    format!("types[\"{}\"][\"size_of\"]", typ_ns_name).into()
                );
                b.var_assign(
                    typ_name_serialize.clone(),
                    format!("types[\"{}\"][\"serialize\"]", typ_ns_name).into()
                );
                b.var_assign(
                    typ_name_deserialize.clone(),
                    format!("types[\"{}\"][\"deserialize\"]", typ_ns_name).into()
                );
            }
            _ => (),
        }
    }

    let exports_inner = exports.iter()
        .map(|&(ref ns_path, ref internal_name)| {
            format!("\"{}\": {{\"size_of\": {i}_size_of, \"serialize\": {i}_serialize, \"deserialize\": {i}_deserialize }}",
                    ns_path, i = internal_name)
        })
        .join(",\n");
    b.var_assign("exports".into(), format!("{{\n{}\n}}", exports_inner).into());

    Ok(b)
}
