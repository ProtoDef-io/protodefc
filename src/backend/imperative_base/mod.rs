pub mod size_of;

pub mod utils;
pub mod container_utils;

mod tests;

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
    TypeCall {
        typ: CallType,
        type_name: String,
        input: Var,
    },
}

#[derive(Debug)]
pub enum MapOperation {
    ArrayLength,
    UnionTagToExpr(Vec<UnionTagCase>),
}

#[derive(Debug)]
pub struct UnionTagCase {
    pub variant_name: String,
    pub variant_var: Option<Var>,
    pub block: Block,
}

#[derive(Debug)]
pub enum CallType {
    SizeOf,
}
impl CallType {
    pub fn short(&self) -> &str {
        match *self {
            CallType::SizeOf => "size_of",
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
