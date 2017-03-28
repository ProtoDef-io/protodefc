use super::TypeContainer;
use ::FieldReference;
use super::errors::*;
use ::json::JsonValue;
use super::type_from_json;

#[derive(Debug, Clone)]
pub enum CountMode {
    Prefixed(TypeContainer),
    Referenced(FieldReference),
    Fixed(usize),
}

pub fn read_count(arg: &JsonValue) -> Result<CountMode> {
    ensure!(arg.is_object(), "type with count must have object argument");

    let has_count = arg.has_key("count");
    let has_count_type = arg.has_key("countType");
    ensure!(has_count != has_count_type,
            "count must have one and only one of 'count' and 'countType'");

    if has_count_type {
        type_from_json(&arg["countType"])
            .chain_err(|| "when reading 'countType'")
            .map(|v| CountMode::Prefixed(v))
    } else {
        let count_arg = &arg["count"];
        if let Some(count) = count_arg.as_u64() {
            Ok(CountMode::Fixed(count as usize))
        } else if let Some(path) = count_arg.as_str() {
            FieldReference::parse(path)
                .ok_or("error when reading field reference from 'count'".into())
                .map(|v| CountMode::Referenced(v))
        } else {
            bail!("'count' argument must be either a path or a number")
        }
    }
}
