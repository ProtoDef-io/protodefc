
pub struct Block(Vec<Statement>);

pub enum Statement {
    Assign {
        name: String,
        expr: Expr,
    },
    If {
        conds: Vec<(Expr, Block)>,
        else_: Option<Block>,
    },
    Return {
        expr: Expr,
    },
    FunctionDeclaration {
        name: String,
        args: Vec<String>,
        block: Block,
    },
    Expr {
        expr: Expr,
    },
    Block {
        block: Block,
    },
    For {
        expr: Expr,
        block: Block,
    },
    Comment {
        text: String,
    },
}

pub struct Expr(pub String);

impl Block {

    pub fn new() -> Block {
        Block(Vec::new())
    }

    pub fn assign(&mut self, name: String, expr: Expr) {
        self.0.push(Statement::Assign { name: name, expr: expr, });
    }

    pub fn if_(&mut self, cond: Expr, block: Block) {
        self.0.push(Statement::If { conds: vec![(cond, block)],
                                    else_: None, });
    }

    pub fn if_chain(&mut self, chain: Vec<(Expr, Block)>, else_: Option<Block>) {
        if chain.len() == 0 {
            if let Some(block) = else_ {
                self.block(block)
            }
        } else {
            self.0.push(Statement::If { conds: chain, else_: else_ })
        }
    }

    pub fn expr(&mut self, expr: Expr) {
        self.0.push(Statement::Expr { expr: expr, });
    }

    pub fn decl_fun(&mut self, name: String, args: Vec<String>, block: Block) {
        self.0.push(Statement::FunctionDeclaration {
            name: name,
            args: args,
            block: block,
        });
    }

    pub fn block(&mut self, block: Block) {
        self.0.push(Statement::Block { block: block });
    }

    pub fn return_(&mut self, expr: Expr) {
        self.0.push(Statement::Return { expr: expr });
    }

    pub fn comment(&mut self, text: String) {
        self.0.push(Statement::Comment { text: text });
    }

    pub fn for_(&mut self, expr: Expr, block: Block) {
        self.0.push(Statement::For {
            expr: expr,
            block: block,
        });
    }

}

impl ToPython for Block {
    fn to_python(&self, out: &mut String, level: u64) {
        for statement in &self.0 {
            statement.to_python(out, level);
        }
    }
}

impl From<String> for Expr {
    fn from(string: String) -> Self {
        Expr(string)
    }
}
impl<'a> From<&'a str> for Expr {
    fn from(string: &str) -> Self {
        Expr(string.to_owned())
    }
}

impl ToPython for Statement {
    fn to_python(&self, out: &mut String, level: u64) {

        match *self {
            Statement::Assign { ref name, ref expr } => {
                pad_level(out, level);

                out.push_str(name);
                out.push_str(" = ");
                out.push_str(&expr.0);
                out.push_str("\n");
            }
            Statement::If { ref conds, ref else_ } => {
                assert!(conds.len() >= 1);

                let mut first = true;
                for &(ref cond, ref block) in conds {
                    pad_level(out, level);
                    if first {
                        out.push_str("if ");
                    } else {
                        out.push_str("elif ");
                    }
                    first = false;

                    out.push_str(&cond.0);
                    out.push_str(":\n");

                    block.to_python(out, level+1);
                }

                if let &Some(ref block) = else_ {
                    pad_level(out, level);
                    out.push_str("else:\n");
                    block.to_python(out, level+1);
                }
            }
            Statement::FunctionDeclaration { ref name, ref args, ref block } => {
                pad_level(out, level);

                out.push_str("def ");
                out.push_str(name);
                out.push_str("(");
                if args.len() > 0 {
                    for num in 0..args.len()-1 {
                        out.push_str(&args[num]);
                        out.push_str(", ");
                    }
                    out.push_str(args.last().unwrap());
                }
                out.push_str("):\n");
                block.to_python(out, level+1);
            }
            Statement::Expr { ref expr } => {
                pad_level(out, level);

                out.push_str(&expr.0);
                out.push_str("\n");
            }
            Statement::Block { ref block } => {
                block.to_python(out, level);
            }
            Statement::Return { ref expr } => {
                pad_level(out, level);
                out.push_str("return ");
                out.push_str(&expr.0);
                out.push_str("\n");
            }
            Statement::Comment { ref text } => {
                pad_level(out, level);
                out.push_str("# ");
                out.push_str(text);
                out.push_str("\n");
            }
            Statement::For { ref expr, ref block } => {
                pad_level(out, level);
                out.push_str("for ");
                out.push_str(&expr.0);
                out.push_str(":\n");

                block.to_python(out, level+1);
            }
        }
    }
}

fn pad_level(string: &mut String, level: u64) {
    for _ in 0..level {
        string.push_str("    ");
    }
}

pub trait ToPython {
    fn to_python(&self, out: &mut String, level: u64);
}
