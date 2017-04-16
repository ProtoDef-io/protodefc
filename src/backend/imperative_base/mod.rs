pub mod size_of;
pub mod serialize;
pub mod deserialize;

pub mod utils;
pub mod container_utils;
pub mod reference;

mod tests;

use ::ir::compilation_unit::{TypePath, NamedTypeContainer};
use ::ir::type_spec::BinaryEncoding;

#[derive(Debug)]
pub struct Block(pub Vec<Operation>);

#[derive(Debug)]
pub struct ResultBlock {
    block: Block,
    result_var: Var,
}

#[derive(Debug)]
pub enum Operation {
    Assign {
        name: Var,
        value: Expr,
    },
    AddCount(Expr),
    ForEachArray {
        array: Var,
        index: Var,
        typ: Var,
        block: Block,
    },
    MapValue {
        input: Var,
        output: Var,
        operation: MapOperation,
    },
    Block(Block),
    ConstructContainer {
        output: Var,
        fields: Vec<(String, Var)>,
    },
    ConstructArray {
        count: Var,
        ident: u64,
        item_var: Var,
        block: Block,
        output: Var,
    },
    ConstructUnion {
        union_name: String,
        union_tag: String,
        output: Var,
        input: Var,
    },
    TypeCall {
        typ: CallType,
        named_type: NamedTypeContainer,
        type_name: TypePath,
        input: Var,
        output: Var,
    },
}

#[derive(Debug)]
pub enum Literal {
    Number(String),
}

#[derive(Debug)]
pub enum Expr {
    InputData,
    Var(Var),
    Literal(Literal),
    ContainerField {
        value: Box<Expr>,
        field: String,
    },
}

#[derive(Debug)]
pub enum MapOperation {
    ArrayLength,
    BinarySize(BinaryEncoding),
    UnionTagToExpr(Vec<UnionTagCase>),
    LiteralToExpr(Vec<LiteralCase>),
}

#[derive(Debug)]
pub struct UnionTagCase {
    pub variant_name: String,
    pub variant_var: Option<Var>,
    pub block: Block,
}

#[derive(Debug)]
pub struct LiteralCase {
    pub value: Literal,
    pub block: Block,
}

#[derive(Debug, Copy, Clone)]
pub enum CallType {
    SizeOf,
    Serialize,
    Deserialize,
}
impl CallType {
    pub fn short(&self) -> &str {
        match *self {
            CallType::SizeOf => "size_of",
            CallType::Serialize => "serialize",
            CallType::Deserialize => "deserialize",
        }
    }
}

#[derive(Debug)]
pub struct Var(pub String);
impl From<String> for Var {
    fn from(input: String) -> Var {
        Var(input)
    }
}

trait BaseCodegen : size_of::BaseSizeOf + serialize::BaseSerialize + deserialize::BaseDeserialize {}

impl BaseCodegen for ::ir::spec::variant::SimpleScalarVariant {}
impl BaseCodegen for ::ir::spec::variant::ContainerVariant {}
impl BaseCodegen for ::ir::spec::variant::ArrayVariant {}
impl BaseCodegen for ::ir::spec::variant::UnionVariant {}

fn codegen_for_type<'a>(typ: &'a ::ir::spec::Type) -> &'a BaseCodegen {
    use ::ir::spec::variant::Variant;
    match typ.variant {
        Variant::SimpleScalar(ref inner) => inner,
        Variant::Container(ref inner) => inner,
        Variant::Array(ref inner) => inner,
        Variant::Union(ref inner) => inner,
        _ => unimplemented!(),
    }
}
