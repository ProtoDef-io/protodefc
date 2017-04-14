use ::ir::spec::TypeContainer;

use ::json::JsonValue;
use self::variants::{ScalarReader, ContainerReader, ArrayReader, UnionReader};
use ::ir::compilation_unit::TypePath;

mod variants;

use ::errors::*;

pub fn from_json(json: &str) -> Result<TypeContainer> {
    let json = ::json::parse(json)?;
    type_from_json(&json)
}

fn type_from_json(json: &JsonValue) -> Result<TypeContainer> {
    ensure!(json.is_object(), "json type must be object");
    ensure!(json["_kind"] == "type", "json type must have \"_kind\": \"type\"");

    let json_type = json["type_name"].as_str()
        .ok_or("expected \"type_name\" key to be string")?;
    let ident = ::frontend::protocol_spec::ast::parser::parse_ident(
        json_type)?.to_type_path();

    match json_type {
        "container" => ContainerReader::from_json(ident, json),
        "array" => ArrayReader::from_json(ident, json),
        "union" => UnionReader::from_json(ident, json),
        _ => ScalarReader::from_json(ident, json),
    }
}

trait FromProtocolJson {
    fn from_json(name: TypePath, arg: &JsonValue) -> Result<TypeContainer>;
}

#[cfg(test)]
mod test {
    use super::from_json;

    #[test]
    fn test_from_json() {
        let input = r#"
{
    "_kind": "type",
    "type_name": "container",
    "fields": [
        {
            "_kind": "container_field",
            "name": "foo",
            "type": {
                "_kind": "type",
                "type_name": "::u8"
            }
        }
    ]
}
"#;
        let res = from_json(&input);
        use error_chain::ChainedError;
        match res {
            Ok(r) => println!("Ok: {:?}", r),
            Err(err) => panic!("Err: {}", err.display()),
        }
    }

}
