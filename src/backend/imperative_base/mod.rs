pub mod size_of;
pub mod serialize;
pub mod deserialize;

pub mod utils;
pub mod container_utils;
pub mod reference;

mod tests;

use std::fmt;

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
    // Special
    Block(Block),
    ThrowError,

    // Assignment
    Assign {
        output_var: Var,
        value: Expr,
    },
    AddCount(Var),

    // Value handling
    ControlFlow {
        input_var: Var,
        variant: ControlFlowVariant,
    },
    Construct {
        output_var: Var,
        variant: ConstructVariant,
    },
    TypeCall {
        input_var: Var,

        call_type: CallType,
        named_type: NamedTypeContainer,
        type_name: TypePath,
        arguments: Vec<Var>,
    },

}

#[derive(Debug)]
pub enum ControlFlowVariant {
    MatchUnionTag {
        cases: Vec<UnionTagCase>,
        default: (Option<Var>, Block),
    },
    MatchLiteral {
        cases: Vec<LiteralCase>,
        default: Block,
    },
    ForEachArray {
        loop_index_var: Var,
        loop_value_var: Var,
        inner: Block,
    },
}

#[derive(Debug)]
pub enum ConstructVariant {
    Container {
        fields: Vec<(String, Var)>,
    },
    Union {
        union_name: String,
        union_tag: String,
        variant_inner_var: Var,
    },
    Array {
        /// The identifier of the array node. Can be used for naming variables uniquely.
        array_node_ident: u64,
        /// Amount of iterations we should perform.
        count_input_var: Var,
        /// The result of each iteration.
        inner_result_var: Var,
        inner: Block,
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
        input_var: Var,
        field: String,
    },
    ArrayLength(Var),
    BinarySize(Var, BinaryEncoding),
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

#[derive(Debug, Clone)]
pub enum CallType {
    SizeOf(Var),
    Serialize,
    Deserialize(Var),
}
impl CallType {
    pub fn short(&self) -> &str {
        match *self {
            CallType::SizeOf(_) => "size_of",
            CallType::Serialize => "serialize",
            CallType::Deserialize(_) => "deserialize",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Var(pub String);
impl Var {
    pub fn string(&self) -> String {
        self.0.clone()
    }
    pub fn str<'a>(&'a self) -> &'a str {
        &self.0
    }
}
impl From<String> for Var {
    fn from(input: String) -> Var {
        Var(input)
    }
}
impl fmt::Display for Var {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl From<Operation> for Block {
    fn from(op: Operation) -> Block {
        Block(vec![op])
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
