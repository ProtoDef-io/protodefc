use ::ir::spec::TypeContainer;
use ::ir::compilation_unit::{TypePath, RelativeNSPath};

use ::json::JsonValue;

use ::errors::*;
use super::super::FromProtocolJson;

pub struct ArrayReader;
impl FromProtocolJson for ArrayReader {
    fn from_json(_name: RelativeNSPath, _arg: &JsonValue) -> Result<TypeContainer> {
        unimplemented!();
    }
}
