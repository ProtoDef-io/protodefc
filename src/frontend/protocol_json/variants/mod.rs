use ::ir::spec::TypeContainer;
use ::ir::spec::variant::SimpleScalarVariant;
use super::FromProtocolJson;
use ::json::JsonValue;
use ::errors::*;
use ::ir::compilation_unit::{TypePath, RelativeNSPath};

mod array;
pub use self::array::ArrayReader;

mod union;
pub use self::union::UnionReader;

mod container;
pub use self::container::ContainerReader;

pub struct ScalarReader;
impl FromProtocolJson for ScalarReader {
    fn from_json(name: RelativeNSPath, _arg: &JsonValue) -> Result<TypeContainer> {
        Ok(SimpleScalarVariant::new(name, vec![]))
    }
}
