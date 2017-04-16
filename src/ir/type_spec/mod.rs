use ::rc_container::{Container, WeakContainer};

pub type TypeSpecContainer = Container<TypeSpec>;
pub type WeakTypeSpecContainer = WeakContainer<TypeSpec>;

#[derive(Debug)]
pub struct TypeSpec {
    pub variant: TypeSpecVariant,
}

#[derive(Debug)]
pub struct TypeName(String);

#[derive(Debug)]
pub enum TypeSpecVariant {
    Container(ContainerSpec),
    Enum(EnumSpec),
    MarkerEnum(MarkerEnumSpec),
    Array(ArraySpec),
    Option(OptionSpec),

    Binary(BinarySpec),
    Integer(IntegerSpec),
    Float(FloatSpec),
    Boolean,

    Unit,
    Box(WeakTypeSpecContainer),
    Opaque,

    Referenced(Option<WeakTypeSpecContainer>),
}

#[derive(Debug)]
pub struct ContainerSpec {
    pub name: TypeName,
    pub fields: Vec<ContainerFieldSpec>,
}
#[derive(Debug)]
pub struct ContainerFieldSpec {
    pub name: TypeName,
    pub type_spec: TypeSpecContainer,
}

#[derive(Debug)]
pub struct EnumSpec {
    pub name: TypeName,
    pub variants: Vec<EnumVariantSpec>,
}
#[derive(Debug)]
pub struct EnumVariantSpec {
    pub name: TypeName,
    pub type_spec: TypeSpecContainer,
}

#[derive(Debug)]
pub struct MarkerEnumSpec {
    pub name: TypeName,
    pub variants: Vec<TypeName>,
}

#[derive(Debug)]
pub enum ArraySize {
    Fixed(usize),
    Dynamic,
}
#[derive(Debug)]
pub struct ArraySpec {
    pub size: ArraySize,
    pub child: TypeSpecContainer,
}

#[derive(Debug)]
pub struct OptionSpec {
    pub child: TypeSpecContainer,
}

#[derive(Debug)]
pub enum IntegerSize {
    AtLeast(usize),
    IndexSize,
}
#[derive(Debug)]
pub enum Signedness {
    Signed,
    Unsigned,
}
#[derive(Debug)]
pub struct IntegerSpec {
    pub size: IntegerSize,
    pub signed: Signedness,
}

#[derive(Debug)]
pub enum FloatSize {
    F32,
    F64,
}
#[derive(Debug)]
pub struct FloatSpec {
    pub size: FloatSize,
}

#[derive(Debug)]
pub enum BinaryEncoding {
    Raw,
    Utf8,
}
#[derive(Debug)]
pub struct BinarySpec {
    pub encoding: BinaryEncoding,
}

impl TypeSpecContainer {
    pub fn new_not_assigned() -> Option<TypeSpecContainer> {
        None
    }

    pub fn follow(self) -> TypeSpecContainer {
        match &self.borrow().variant {
            &TypeSpecVariant::Referenced(ref inner) =>
                return inner.clone().unwrap().upgrade(),
            _ => (),
        }
        self
    }

}

impl TypeSpecVariant {
    pub fn get_child_name(&self, name: &str) -> Option<TypeSpecContainer> {
        match *self {
            TypeSpecVariant::Container(ref container) => {
                container.fields.iter()
                    .find(|f| f.name.0 == name)
                    .map(|f| f.type_spec.clone())
            },
            _ => None
        }
    }

    pub fn is_valid(&self) -> bool {
        match *self {
            TypeSpecVariant::Referenced(None) => false,
            _ => true,
        }
    }

    pub fn is_integer(&self) -> bool {
        assert!(self.is_valid(), "type spec is not assigned");
        match *self {
            TypeSpecVariant::Integer(_) => true,
            _ => false,
        }
    }
}

impl Into<TypeSpecContainer> for TypeSpecVariant {
    fn into(self) -> TypeSpecContainer {
        TypeSpecContainer::new(TypeSpec {
            variant: self,
        })
    }
}

impl From<String> for TypeName {
    fn from(string: String) -> TypeName {
        TypeName(string)
    }
}
