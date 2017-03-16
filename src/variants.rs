use super::{TypeVariant, TypeData, Result, WeakTypeContainer};
use ::field_reference::FieldReference;

#[derive(Debug)]
pub enum Variant {
    // Composite
    Container(ContainerVariant),

    // Arrays
    ReferencedArray(ReferencedArrayVariant),
    PrefixedArray(PrefixedArrayVariant),
    FixedArray(FixedArrayVariant),

    // Conditional
    Switch(SwitchVariant),

    // Strings/Data buffers
    PrefixedString(PrefixedStringVariant),

    // Simple
    SimpleScalar(SimpleScalarVariant),
    Error(ErrorVariant),
}

impl Variant {
    pub fn to_variant<'a>(&'a self) -> &'a TypeVariant {
        match *self {
            Variant::Container(ref inner) => inner,
            Variant::ReferencedArray(ref inner) => inner,
            Variant::PrefixedArray(ref inner) => inner,
            Variant::FixedArray(ref inner) => inner,
            Variant::Switch(ref inner) => inner,
            Variant::PrefixedString(ref inner) => inner,
            Variant::SimpleScalar(ref inner) => inner,
            Variant::Error(ref inner) => inner,
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
    fn resolve_child_name(&self, _data: &TypeData, _name: &str) -> Result<WeakTypeContainer> {
        bail!("attempted to access child of scalar");
    }
}

// Container
#[derive(Debug)]
pub struct ContainerVariant {
    pub fields: Vec<ContainerField>,
}
impl TypeVariant for ContainerVariant {
    fn resolve_child_name(&self, _data: &TypeData, name: &str) -> Result<WeakTypeContainer> {
        self.fields.iter()
            .find(|f| f.name == name)
            .map(|f| f.child.clone())
            .ok_or("container has no field".into())
    }
}

#[derive(Debug)]
pub struct ContainerField {
    pub name: String,
    pub child: WeakTypeContainer,
    pub child_index: usize,
}

// Array
#[derive(Debug)]
pub struct ReferencedArrayVariant {
    pub count: Option<WeakTypeContainer>,
    pub count_path: FieldReference,

    pub child: WeakTypeContainer,
    pub child_index: usize,
}
impl TypeVariant for ReferencedArrayVariant {
    fn resolve_child_name(&self, _data: &TypeData, _name: &str) -> Result<WeakTypeContainer> {
        bail!("attempted to access child of array");
    }
}

#[derive(Debug)]
pub struct PrefixedArrayVariant {
    pub count: WeakTypeContainer,
    pub count_index: usize,

    pub child: WeakTypeContainer,
    pub child_index: usize,
}
impl TypeVariant for PrefixedArrayVariant {
    fn resolve_child_name(&self, _data: &TypeData, _name: &str) -> Result<WeakTypeContainer> {
        bail!("attempted to access child of array");
    }
}

#[derive(Debug)]
pub struct FixedArrayVariant {
    pub count: usize,

    pub child: WeakTypeContainer,
    pub child_index: usize,
}
impl TypeVariant for FixedArrayVariant {
    fn resolve_child_name(&self, _data: &TypeData, _name: &str) -> Result<WeakTypeContainer> {
        bail!("attempted to access child of array");
    }
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
    fn resolve_child_name(&self, _data: &TypeData, _name: &str) -> Result<WeakTypeContainer> {
        bail!("attempted to access child of switch");
    }
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
    fn resolve_child_name(&self, _data: &TypeData, _name: &str) -> Result<WeakTypeContainer> {
        bail!("attempted to access child of error");
    }
}

#[derive(Debug)]
pub struct PrefixedStringVariant {
    pub length: WeakTypeContainer,
    pub length_index: usize,
}
impl TypeVariant for PrefixedStringVariant {
    fn resolve_child_name(&self, _data: &TypeData, _name: &str) -> Result<WeakTypeContainer> {
        bail!("attempted to access child of string");
    }
}
