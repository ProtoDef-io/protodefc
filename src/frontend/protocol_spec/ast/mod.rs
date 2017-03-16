use std::collections::HashMap;

pub mod parser;
pub mod printer;

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub attributes: HashMap<String, Value>,
    pub items: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    String {
        string: String,
        is_block: bool,
    },
    Item {
        name: Ident,
        args: Vec<Value>,
        block: Block,
    },
}

#[derive(Debug, Clone)]
pub enum Ident {
    Simple(String),
    RootNs(Vec<String>),
    // Nc(Vec<String>),
}

// Helpers

impl Block {

    pub fn empty() -> Block {
        Block {
            statements: vec![],
        }
    }

}
