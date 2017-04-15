use ::errors::*;
use ::ir::type_spec::*;

use super::super::ast::{Value};

pub fn items_to_type_spec(values: &[Value]) -> Result<TypeSpecContainer> {
    let head_val = values.first().ok_or("expected another item")?;
    let item = head_val.item().ok_or("type spec value must be item")?;

    let name_str = item.name.simple_str()
        .ok_or("type spec value name must be non-namespaced")?;

    let res = match name_str {
        "integer" => TypeSpecVariant::Integer(IntegerSpec {
            size: IntegerSize::B64,
            signed: Signedness::Signed,
        }),
        name => bail!("'{}' not supported in type definitions", name),
    };
    Ok(res.into())
}
