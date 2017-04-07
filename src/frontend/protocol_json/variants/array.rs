use ::ir::FieldReference;
use ::ir::typ::TypeContainer;
use ::ir::typ::variant::{ArrayVariant, ContainerVariantBuilder};
use ::ir::compilation_unit::TypePath;

use ::json::JsonValue;

use ::errors::*;
use super::super::{FromProtocolJson, type_from_json};

pub struct ArrayReader;
impl FromProtocolJson for ArrayReader {
    fn from_json(name: TypePath, arg: &JsonValue) -> Result<TypeContainer> {
        unimplemented!();
    }
}
