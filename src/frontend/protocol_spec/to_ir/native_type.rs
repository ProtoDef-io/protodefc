use ::errors::*;
use ::ir::compilation_unit::NamedTypeArgument;
use ::ir::spec::data::ReferenceAccessTime;

use super::super::ast::{Block};
use super::type_spec::items_to_type_spec;

pub fn block_to_args(block: &Block) -> Result<Vec<NamedTypeArgument>> {
    let mut arguments = Vec::new();

    for stmt in &block.statements {
        let root_item = stmt.items[0].item().ok_or("root item must be item")?;
        let root_name = root_item.name
            .simple_str().ok_or("root item must be non-namespaced")?;

        match root_name {
            "argument" => {
                root_item.validate(1, &["stage"], &["stage"])?;

                let argument_name = root_item.arg(0).unwrap()
                    .string().ok_or("argument name must be string")?;

                let access_time_str = root_item.tagged_arg("stage").unwrap()
                    .string().ok_or("stage must be string")?;
                let access_time = match access_time_str {
                    "read" => ReferenceAccessTime::Read,
                    "read_write" => ReferenceAccessTime::ReadWrite,
                    _ => bail!("'stage' must be either read or read_write"),
                };

                let type_spec = items_to_type_spec(&stmt.items[1..])?;

                arguments.push(NamedTypeArgument {
                    name: argument_name.to_owned(),
                    access_time: access_time,
                    type_spec: type_spec,
                });
            }
            _ => bail!("invalid root item name"),
        }
    }

    Ok(arguments)
}
