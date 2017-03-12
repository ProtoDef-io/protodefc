extern crate json;
use ::{Type, TypeVariant, TypeContainer};
use ::variants::{SimpleScalarVariant, ContainerVariant};

use self::json::JsonValue;
use self::variants::{ScalarReader, ContainerReader, ArrayReader};

mod variants;

mod errors {
    error_chain! {

        foreign_links {
            ParseError(super::json::Error);
        }

    }
}

pub use self::errors::*;

trait FromProtocolJson {
    fn from_json(name: String, arg: &JsonValue) -> Result<TypeContainer>;
}

fn variant_from_name(name: String, arg: &JsonValue) -> Result<TypeContainer> {
    // TODO: Add error to chain
    match name.as_str() {
        "i8" => ScalarReader::from_json(name, arg),
        "container" => ContainerReader::from_json(name, arg),
        "array" => ArrayReader::from_json(name, arg),
        _ => bail!("No variant matches name {:?}", name),
    }
}

fn type_from_json(json: &JsonValue) -> Result<TypeContainer> {
    let is_unit_type = json.is_string();
    let is_args_type = json.is_array() && json.len() == 2 && json[0].is_string();
    ensure!(is_unit_type || is_args_type, "Expected protocol type, got {:?}", json);

    let null = &JsonValue::Null;
    let (name, args) = if is_unit_type {
        let name = json.as_str().unwrap().to_string();
        (name, null)
    } else {
        let name = json[0].as_str().unwrap().to_string();
        let args = &json[1];
        (name, args)
    };

    variant_from_name(name, args)
}

pub fn from_json(json: &str) -> Result<TypeContainer> {
    let json = json::parse(json)?;
    type_from_json(&json)
}

#[cfg(test)]
mod test {
    use super::from_json;

    #[test]
    fn test_from_json() {
        let input = "[\"container\", [{\"name\": \"foo\", \"type\": \"i8\"}]]";
        //let input = "[\"i8\", {}]";
        let res = from_json(&input);
        use error_chain::ChainedError;
        match res {
            Ok(r) => println!("Ok: {:?}", r),
            Err(err) => println!("Err: {}", err.display()),
        }
        //panic!();
    }

}
