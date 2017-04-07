use ::frontend::protocol_spec::ast::*;
use ::errors::*;
use ::json::JsonValue;
use std::collections::HashMap;

pub fn typ_to_pds(items: &mut Vec<Value>, json: &JsonValue) -> Result<()> {
    if json.is_string() {
        let name = json.as_str().unwrap();
        named_typ_to_pds(name, items, &JsonValue::Null)?;
    } else if json.is_array() {
        let name = json[0].as_str().unwrap();
        named_typ_to_pds(name, items, &json[1])?;
    } else {
        bail!("malformed type");
    }
    Ok(())
}

fn named_typ_to_pds(name: &str, items: &mut Vec<Value>, arg: &JsonValue) -> Result<()> {
    match name {
        "container" => container_to_pds(items, arg),
        "array" => array_to_pds(items, arg),
        "switch" => switch_to_pds(items, arg),
        _ => terminal_to_pds(name, items, arg)
    }
}

fn switch_to_pds(items: &mut Vec<Value>, arg: &JsonValue) -> Result<()> {
    let compare_to = arg["compareTo"].as_str()
        .ok_or("compareTo on switch must be string")?;

    let mut block = Block::empty();
    for (key, val) in arg["fields"].entries() {
        let mut inner_items = Vec::new();

        inner_items.push(Value::Item(Item {
            name: Ident::Simple("case".to_owned()),
            args: vec![ItemArg::new(Value::new_string(key.to_owned()))],
            block: Block::empty(),
        }));

        typ_to_pds(&mut inner_items, val)?;

        block.statements.push(Statement {
            attributes: HashMap::new(),
            items: inner_items,
        });
    }

    items.push(Value::Item(Item {
        name: Ident::Simple("switch".to_owned()),
        args: vec![ItemArg::with_tag("compareTo".to_owned(),
                                     Value::new_string(compare_to.to_owned()))],
        block: block,
    }));

    Ok(())
}

fn array_to_pds(items: &mut Vec<Value>, arg: &JsonValue) -> Result<()> {
    let mut args: Vec<ItemArg> = Vec::new();
    if arg["count"].is_string() || arg["count"].is_number() {
        let string = arg["count"].dump();
        args.push(ItemArg::with_tag("count".to_owned(),
                                    Value::new_string(string)))
    }
    if arg["countType"].is_string() || arg["countType"].is_array() {
        let string = arg["countType"].dump();
        args.push(ItemArg::with_tag("countType".to_owned(),
                                    Value::new_string(string)))
    }
    items.push(Value::Item(Item {
        name: Ident::Simple("array".to_owned()),
        args: args,
        block: Block::empty(),
    }));
    typ_to_pds(items, &arg["type"])?;
    Ok(())
}

fn container_to_pds(items: &mut Vec<Value>, arg: &JsonValue) -> Result<()> {
    let mut block = Block::empty();

    for field in arg.members() {
        let is_anon = field["anon"] == true;

        let item_name = if is_anon { "anon_field" } else { "field" };
        let args = if is_anon {
            vec![]
        } else {
            let field_name = field["name"].as_str()
                .ok_or("container field needs \"name\" key")?;
            vec![
                ItemArg::new(Value::new_string(field_name.into())),
            ]
        };

        let mut items = vec![
            Value::Item(Item {
                name: Ident::Simple(item_name.to_owned()),
                args: args,
                block: Block::empty(),
            }),
        ];

        typ_to_pds(&mut items, &field["type"])?;

        block.statements.push(Statement {
            attributes: HashMap::new(),
            items: items,
        });
    }

    items.push(Value::Item(Item {
        name: Ident::Simple("container".into()),
        args: vec![],
        block: block,
    }));

    Ok(())
}

fn terminal_to_pds(name: &str, items: &mut Vec<Value>, arg: &JsonValue) -> Result<()> {
    items.push(Value::Item(Item {
        name: Ident::Simple(name.into()),
        args: vec![],
        block: Block::empty(),
    }));
    if arg != &JsonValue::Null {
        items.push(Value::new_string(arg.dump()));
    }
    Ok(())
}
