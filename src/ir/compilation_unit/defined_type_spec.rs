use ::rc_container::{Container, WeakContainer};
use super::TypePath;

pub type DefinedTypeSpecContainer = Container<DefinedTypeSpec>;
pub type WeakDefinedTypeSpecContainer = WeakContainer<DefinedTypeSpec>;

#[derive(Debug)]
pub struct DefinedTypeSpec {
    pub path: TypePath,
}
