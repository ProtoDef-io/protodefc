pub mod size_of;

mod tests;

#[derive(Debug)]
pub struct Block(Vec<Operation>);

#[derive(Debug)]
pub enum Operation {
    Assign {
        name: Var,
        value: Expr,
    },
    AddCount(Expr),
    ForEachArray {
        array: Expr,
        index: Var,
        typ: Var,
        block: Block,
    },
    Block(Block),
}

#[derive(Debug)]
pub enum Expr {
    InputData,
    Var(Var),
    PropertyAccess {
        value: Box<Expr>,
        property: Property,
    },
    ContainerField {
        value: Box<Expr>,
        field: String,
    },
    TypeCall {
        typ: CallType,
        type_name: Var,
        input: Var,
    },
}

#[derive(Debug)]
pub enum Property {
    ArrayLength,
}

#[derive(Debug)]
pub enum CallType {
    SizeOf,
}

#[derive(Debug)]
pub struct Var(String);
impl From<String> for Var {
    fn from(input: String) -> Var {
        Var(input)
    }
}
