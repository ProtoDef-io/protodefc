use super::{TypeVariant, TypeData, Result, WeakTypeContainer};
use ::field_reference::FieldReference;

macro_rules! default_resolve_child_name_impl {
    () => {
        fn resolve_child_name(&self, _data: &TypeData, _name: &str)
                              -> Result<WeakTypeContainer> {
            bail!("attempted to access child of unsupported type");
        }
    }
}
macro_rules! default_has_property_impl {
    () => {
        fn has_property(&self, _data: &TypeData, _prop_name: &str) -> bool {
            false
        }
    }
}
macro_rules! default_resolve_references {
    () => {
        fn do_resolve_references(&mut self, data: &mut TypeData,
                                 resolver: &::ReferenceResolver) -> Result<()> {
            Ok(())
        }
    }
}

mod container;
pub use self::container::{ContainerVariant, ContainerField, ContainerVariantBuilder};
mod array;
pub use self::array::ArrayVariant;

#[derive(Debug)]
pub enum Variant {
    // Composite
    Container(ContainerVariant),

    // Arrays
    Array(ArrayVariant),

    // Conditional
    Switch(SwitchVariant),
    //Union(UnionVariant),

    // Strings/Data buffers
    String(StringVariant),

    // Simple
    SimpleScalar(SimpleScalarVariant),
    Error(ErrorVariant),
}

impl Variant {
    pub fn to_variant<'a>(&'a self) -> &'a TypeVariant {
        match *self {
            Variant::Container(ref inner) => inner,
            Variant::Array(ref inner) => inner,
            Variant::Switch(ref inner) => inner,
            Variant::String(ref inner) => inner,
            Variant::SimpleScalar(ref inner) => inner,
            Variant::Error(ref inner) => inner,
        }
    }

    pub fn to_variant_mut<'a>(&'a mut self) -> &'a mut TypeVariant {
        match *self {
            Variant::Container(ref mut inner) => inner,
            Variant::Array(ref mut inner) => inner,
            Variant::Switch(ref mut inner) => inner,
            Variant::String(ref mut inner) => inner,
            Variant::SimpleScalar(ref mut inner) => inner,
            Variant::Error(ref mut inner) => inner,
        }
    }
}

/// This is a simple terminal scalar.
///
/// All types that take no special arguments and that have
/// no children should be represented by this variant.
///
/// It is up to the backend to generate code for the name of
/// the type.
#[derive(Debug)]
pub struct SimpleScalarVariant {}
impl TypeVariant for SimpleScalarVariant {
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
}

#[derive(Debug)]
pub struct SwitchVariant {
    pub options: Vec<SwitchCase>,

    pub default: Option<WeakTypeContainer>,
    pub default_index: usize,

    pub compare: Option<WeakTypeContainer>,
    pub compare_path: FieldReference,
}
impl TypeVariant for SwitchVariant {
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
}

#[derive(Debug)]
pub struct SwitchCase {
    pub raw_value: String,
    pub child: WeakTypeContainer,
    pub child_index: usize,
}

#[derive(Debug)]
pub struct ErrorVariant {
    pub message: String,
}
impl TypeVariant for ErrorVariant {
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
}

#[derive(Debug)]
pub struct StringVariant {
    pub length: WeakTypeContainer,
    pub length_index: usize,
}
impl TypeVariant for StringVariant {
    default_resolve_child_name_impl!();
    default_has_property_impl!();
    default_resolve_references!();
}
