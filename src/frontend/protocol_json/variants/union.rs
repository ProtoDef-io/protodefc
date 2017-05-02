use super::super::FromProtocolJson;
use ::json::JsonValue;
use ::TypeContainer;
use ::ir::compilation_unit::{TypePath, RelativeNSPath};

use ::errors::*;

pub struct UnionReader;

impl FromProtocolJson for UnionReader {
    fn from_json(_name: RelativeNSPath, _arg: &JsonValue) -> Result<TypeContainer> {
        unimplemented!();
    }
}
