use super::{TypeVariant, TypeData};
use ::ir::compilation_unit::TypePath;

macro_rules! default_resolve_child_name_impl {
    () => {
        fn resolve_child_name(&self, data: &TypeData, _name: &str)
                              -> Result<WeakTypeContainer> {
            bail!("attempted to access child of unsupported type {:?}",
                  self.get_type(data));
        }
    }
}
macro_rules! default_has_property_impl {
    () => {
        fn has_property(&self, _data: &TypeData, _prop_name: &str)
                        -> Option<TargetType> {
            None
        }
    }
}
macro_rules! default_get_result_type_impl {
    () => {
        fn get_result_type(&self, _data: &TypeData) -> Option<TargetType> {
            Some(TargetType::Unknown)
        }
    }
}
macro_rules! default_resolve_references {
    () => {
        fn do_resolve_references(&mut self, _data: &mut TypeData,
                                 _resolver: &::ReferenceResolver) -> Result<()> {
            Ok(())
        }
    }
}
macro_rules! default_resolve_on_context {
    () => {
        fn resolve_on_context(&mut self, _data: &TypeData, _current_path: &TypePath,
                              _context: &CompilationUnit) -> Result<()> {
            Ok(())
        }
    }
}
macro_rules! default_do_compile_pass {
    () => {
        fn do_compile_pass(&mut self, _data: &TypeData, _pass: &mut CompilePass)
                           -> Result<()> {
            Ok(())
        }
    }
}

mod union;
pub use self::union::{UnionVariant, UnionVariantBuilder};

mod container;
pub use self::container::{ContainerVariant, ContainerField, ContainerVariantBuilder, ContainerFieldType};

mod array;
pub use self::array::ArrayVariant;

mod sized_buffer;
pub use self::sized_buffer::SizedBufferVariant;

mod terminated_buffer;
pub use self::terminated_buffer::TerminatedBufferVariant;

mod simple_scalar;
pub use self::simple_scalar::SimpleScalarVariant;

mod error;
pub use self::error::ErrorVariant;

#[derive(Debug)]
pub enum Variant {
    // Composite
    Container(ContainerVariant),
    Array(ArrayVariant),
    Union(UnionVariant),

    // Strings/Data buffers
    SizedBuffer(SizedBufferVariant),
    TerminatedBuffer(TerminatedBufferVariant),

    // Simple
    SimpleScalar(SimpleScalarVariant),
    Error(ErrorVariant),
}

impl Variant {
    pub fn to_variant<'a>(&'a self) -> &'a TypeVariant {
        match *self {
            Variant::Container(ref inner) => inner,
            Variant::Array(ref inner) => inner,
            Variant::Union(ref inner) => inner,
            Variant::SizedBuffer(ref inner) => inner,
            Variant::TerminatedBuffer(ref inner) => inner,
            Variant::SimpleScalar(ref inner) => inner,
            Variant::Error(ref inner) => inner,
        }
    }

    pub fn to_variant_mut<'a>(&'a mut self) -> &'a mut TypeVariant {
        match *self {
            Variant::Container(ref mut inner) => inner,
            Variant::Array(ref mut inner) => inner,
            Variant::Union(ref mut inner) => inner,
            Variant::SizedBuffer(ref mut inner) => inner,
            Variant::TerminatedBuffer(ref mut inner) => inner,
            Variant::SimpleScalar(ref mut inner) => inner,
            Variant::Error(ref mut inner) => inner,
        }
    }

    pub fn get_type(&self, data: &TypeData) -> VariantType {
        match *self {
            Variant::Container(_) => VariantType::Container,
            Variant::Array(_) => VariantType::Array,
            Variant::Union(_) => VariantType::Union,
            Variant::SizedBuffer(_) => VariantType::SizedBuffer,
            Variant::TerminatedBuffer(_) => VariantType::TerminatedBuffer,
            Variant::SimpleScalar(_) =>
                VariantType::SimpleScalar(data.name.clone()),
            Variant::Error(_) => VariantType::Error,
        }
    }

}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VariantType {
    Container,
    Array,
    Union,
    SizedBuffer,
    TerminatedBuffer,
    SimpleScalar(TypePath),
    Error,
}
