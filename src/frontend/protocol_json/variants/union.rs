use super::super::{FromProtocolJson, type_from_json};
use ::json::JsonValue;
use ::TypeContainer;
use ::ir::compilation_unit::TypePath;

use ::errors::*;

pub struct UnionReader;

impl FromProtocolJson for UnionReader {
    fn from_json(name: TypePath, arg: &JsonValue) -> Result<TypeContainer> {
        unimplemented!();
    }
}
