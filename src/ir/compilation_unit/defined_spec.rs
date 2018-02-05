use ::TypeContainer;
use ::ir::type_spec::TypeSpecContainer;
use ::ir::spec::data::ReferenceAccessTime;
use ::rc_container::{Container, WeakContainer};
use super::TypePath;

pub type NamedTypeContainer = Container<NamedType>;
pub type WeakNamedTypeContainer = WeakContainer<NamedType>;

#[derive(Debug)]
pub struct NamedType {
    pub path: TypePath,

    pub typ: TypeKind,
    pub type_spec: TypeSpecContainer,
    pub type_id: u64,

    pub arguments: Vec<NamedTypeArgument>,

    pub export: Option<String>,

    pub docstring: String,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Native(NativeType),
    Type(TypeContainer),
}

#[derive(Debug, Clone)]
pub struct NativeType {
    pub type_spec: TypeSpecContainer,
}
#[derive(Debug, Clone)]
pub struct NamedTypeArgument {
    pub name: String,
    pub access_time: ReferenceAccessTime,
    pub type_spec: TypeSpecContainer,
}
