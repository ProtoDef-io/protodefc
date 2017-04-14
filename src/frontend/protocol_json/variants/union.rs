use super::super::FromProtocolJson;
use ::json::JsonValue;
use ::TypeContainer;
use ::ir::compilation_unit::TypePath;

use ::errors::*;

pub struct UnionReader;

impl FromProtocolJson for UnionReader {
    fn from_json(_name: TypePath, _arg: &JsonValue) -> Result<TypeContainer> {
        unimplemented!();
    }
}
