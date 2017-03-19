use std::collections::HashMap;

pub mod parser;
pub mod printer;
mod value_helpers;

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
    Item(Item),
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: Ident,
    pub args: Vec<ItemArg>,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct ItemArg {
    pub tag: Option<String>,
    pub value: Value,
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

impl Item {

    pub fn arg<'a>(&'a self, pos: usize) -> Option<&'a Value> {
        self.args
            .get(pos)
            .map(|a| &a.value)
    }

    pub fn tagged_arg<'a>(&'a self, tag: &str) -> Option<&'a Value> {
        self.args.iter()
            .find(|a| a.tag.as_ref().map(|b| b.as_str()) == Some(tag))
            .map(|a| &a.value)
    }

}

impl ItemArg {

    pub fn new(val: Value) -> ItemArg {
        ItemArg {
            tag: None,
            value: val,
        }
    }

    pub fn with_tag(tag: String, val: Value) -> ItemArg {
        ItemArg {
            tag: Some(tag),
            value: val,
        }
    }

}
impl From<Value> for ItemArg {
    fn from(item: Value) -> ItemArg {
        ItemArg::new(item)
    }
}
