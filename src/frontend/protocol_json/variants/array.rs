use ::ir::spec::TypeContainer;
use ::ir::compilation_unit::TypePath;

use ::json::JsonValue;

use ::errors::*;
use super::super::FromProtocolJson;

pub struct ArrayReader;
impl FromProtocolJson for ArrayReader {
    fn from_json(_name: TypePath, _arg: &JsonValue) -> Result<TypeContainer> {
        unimplemented!();
    }
}
