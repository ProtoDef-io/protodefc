use ::itertools::Itertools;
use ::errors::*;
use ::json::JsonValue;
use ::frontend::protocol_spec::ast::*;
use std::collections::HashMap;
use super::typ::typ_to_pds;

pub fn namespace_to_pds(json: &JsonValue) -> Result<Block> {
    let mut block = Block::empty();
    let mut path = Vec::new();

    namespace_to_pds_inner(&mut block, &mut path, json)?;

    Ok(block)
}

fn namespace_to_pds_inner(block: &mut Block, path: &mut Vec<String>, json: &JsonValue) -> Result<()> {

    let mut ns_block = Block::empty();
    for (key, val) in json["types"].entries() {
        let is_native = val == "native";

        let name = if is_native { "def_native" } else { "def" };
        let mut items = vec![
            Value::Item(Item{
                name: Ident::simple(name.into()),
                args: vec![
                    ItemArg::new(Value::new_string(key.to_owned())),
                ],
                block: Block::empty(),
            })
        ];

        if !is_native {
            typ_to_pds(&mut items, val)
                .chain_err(|| format!("inside type {:?}", key))?;
        }

        ns_block.statements.push(Statement {
            attributes: HashMap::new(),
            items: items,
        });
    }
    if ns_block.statements.len() != 0 {
        let path_str = path.iter().join("::");
        block.statements.push(Statement {
            attributes: HashMap::new(),
            items: vec![
                Value::Item(Item{
                    name: Ident::simple("namespace".into()),
                    args: vec![
                        ItemArg::new(Value::new_string(path_str)),
                    ],
                    block: ns_block,
                })
            ],
        });
    }

    let ns_iterator = json.entries()
        .filter(|&(key, _)| key != "types");
    for (key, val) in ns_iterator {
        path.push(key.to_owned());
        namespace_to_pds_inner(block, path, val)?;
        path.pop();
    }

    Ok(())
}
