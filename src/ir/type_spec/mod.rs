use ::rc_container::{Container, WeakContainer};

pub type TypeSpecContainer = Container<TypeSpec>;
pub type WeakTypeSpecContainer = WeakContainer<TypeSpec>;

pub struct TypeName(String);

pub enum TypeSpec {
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
    Opaque,
}

pub struct ContainerSpec {
    pub name: TypeName,
}

pub struct EnumSpec {
    pub name: TypeName,
    pub variants: Vec<(TypeName, TypeSpec)>,
}

pub struct MarkerEnumSpec {
    pub name: TypeName,
    pub variants: Vec<TypeName>,
}

pub enum ArraySize {
    Fixed(usize),
    Dynamic,
}
pub struct ArraySpec {
    pub size: ArraySize,
    pub child: Box<TypeSpec>,
}

pub struct OptionSpec {
    pub child: Box<TypeSpec>,
}

pub enum IntegerSize {
    B8,
    B16,
    B32,
    B64,
    Big,
}
pub enum Signedness {
    Signed,
    Unsigned,
}
pub struct IntegerSpec {
    pub size: IntegerSize,
    pub signed: Signedness,
}

pub enum FloatSize {
    Float,
    Double,
}
pub struct FloatSpec {
    pub size: FloatSize,
}

pub enum BinaryEncoding {
    Raw,
    Ascii,
    Utf8,
}
pub struct BinarySpec {
    pub encoding: BinaryEncoding,
}
