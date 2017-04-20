use ::errors::*;
use ::ir::type_spec::*;

use super::super::ast::{Value};

pub fn items_to_type_spec(values: &[Value]) -> Result<TypeSpecContainer> {
    let head_val = values.first().ok_or("expected another item")?;
    let item = head_val.item().ok_or("type spec value must be item")?;

    let name_str = item.name.simple_str()
        .ok_or("type spec value name must be non-namespaced")?;

    let res = match name_str {
        "integer" => {
            item.validate(1, &[], &[])?;
            let inner = item.arg(0).unwrap().string()
                .ok_or("argument to integer must be string")?;
            TypeSpecVariant::Integer(parse_integer_spec(inner)?)
        }
        "binary" => {
            item.validate(1, &[], &[])?;
            let inner = item.arg(0).unwrap().string()
                .ok_or("argument to binary must be string")?;
            TypeSpecVariant::Binary(parse_binary_spec(inner)?)
        }
        "boolean" => TypeSpecVariant::Boolean,
        "opaque" => TypeSpecVariant::Opaque,
        name => bail!("'{}' not supported in type definitions", name),
    };
    Ok(res.into())
}

fn parse_binary_spec(input: &str) -> Result<BinarySpec> {
    let encoding = match input {
        "raw" => BinaryEncoding::Raw,
        "utf8" => BinaryEncoding::Utf8,
        _ => bail!("unknown binary encoding '{}'", input),
    };

    Ok(BinarySpec {
        encoding: encoding,
    })
}

fn parse_integer_spec(input: &str) -> Result<IntegerSpec> {
    let head_char = input.as_bytes().get(0).ok_or("integer spec string too short")?;
    let tail = &input[1..];

    let signed = match *head_char {
        b'i' => true,
        b'u' => false,
        _ => bail!("integer spec string must start with either i or u"),
    };

    let size = match tail {
        "size" => IntegerSize::IndexSize,
        _ => {
            let precision = tail.parse::<usize>()
                .map_err(|_| "invalid precision")?;
            IntegerSize::AtLeast(precision)
        },
    };

    Ok(IntegerSpec {
        signed: signed,
        size: size,
    })
}
