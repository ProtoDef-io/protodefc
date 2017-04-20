
pub struct Block(Vec<Statement>);

pub enum Statement {
    Assign {
        is_let: bool,
        lhs: String,
        rhs: String,
    },
    AssignBlock {
        is_let: bool,
        lhs: String,
        rhs: Block,
    },
    Expr {
        inner: String,
    },
    Block {
        block: Block,
    },
    Inline {
        block: Block,
    },
    Module {
        name: String,
        block: Block,
    },
    DefineFunction {
        signature: String,
        block: Block,
    },
    For {
        condition: String,
        block: Block,
    },
    Match {
        input: String,
        cases: Vec<MatchCase>,
    }
}

pub struct MatchCase {
    pub pattern: String,
    pub block: Block,
}

impl Block {

    pub fn new() -> Block {
        Block(Vec::new())
    }

    pub fn assign(&mut self, lhs: String, rhs: String) {
        self.0.push(Statement::Assign { is_let: false, lhs: lhs, rhs: rhs });
    }
    pub fn let_assign(&mut self, lhs: String, rhs: String) {
        self.0.push(Statement::Assign { is_let: true, lhs: lhs, rhs: rhs });
    }

    pub fn expr(&mut self, string: String) {
        self.0.push(Statement::Expr { inner: string });
    }

    pub fn block(&mut self, block: Block) {
        self.0.push(Statement::Block { block: block });
    }

    pub fn inline(&mut self, block: Block) {
        self.0.push(Statement::Inline { block: block });
    }

    pub fn decl_fun(&mut self, signature: String, block: Block) {
        self.0.push(Statement::DefineFunction { signature: signature, block: block });
    }

    pub fn for_(&mut self, cond: String, block: Block) {
        self.0.push(Statement::For { condition: cond, block: block });
    }

    pub fn match_(&mut self, input: String, cases: Vec<MatchCase>) {
        self.0.push(Statement::Match { input: input, cases: cases });
    }

}

fn pad_level(string: &mut String, level: u64) {
    for _ in 0..level {
        string.push_str("    ");
    }
}

pub trait ToRust {
    fn to_rust(&self, out: &mut String, level: u64);
}

impl ToRust for Block {
    fn to_rust(&self, out: &mut String, level: u64) {
        for statement in &self.0 {
            statement.to_rust(out, level);
        }
    }
}

impl ToRust for Statement {
    fn to_rust(&self, out: &mut String, level: u64) {
        match *self {
            Statement::Expr { ref inner } => {
                pad_level(out, level);
                out.push_str(inner);
                out.push_str(";\n");
            }
            Statement::Inline { ref block } => {
                block.to_rust(out, level);
            }
            Statement::Block { ref block } => {
                pad_level(out, level);
                out.push_str("{\n");
                block.to_rust(out, level+1);
                pad_level(out, level);
                out.push_str("};\n");
            }
            Statement::Assign { is_let, ref lhs, ref rhs } => {
                pad_level(out, level);
                if is_let { out.push_str("let ") };
                out.push_str(lhs);
                out.push_str(" = ");
                out.push_str(rhs);
                out.push_str(";\n");
            }
            Statement::AssignBlock { is_let, ref lhs, ref rhs } => {
                pad_level(out, level);
                if is_let { out.push_str("let ") };
                out.push_str(lhs);
                out.push_str(" = {");
                rhs.to_rust(out, level+1);
                out.push_str("};\n");
            }
            Statement::Module { ref name, ref block } => {
                pad_level(out, level);
                out.push_str("mod ");
                out.push_str(name);
                out.push_str(" {\n");
                block.to_rust(out, level+1);
                pad_level(out, level);
                out.push_str("}\n");
            }
            Statement::DefineFunction { ref signature, ref block } => {
                pad_level(out, level);
                out.push_str("fn ");
                out.push_str(signature);
                out.push_str(" {\n");
                block.to_rust(out, level+1);
                pad_level(out, level);
                out.push_str("}\n");
            }
            Statement::For { ref condition, ref block } => {
                pad_level(out, level);
                out.push_str("for ");
                out.push_str(condition);
                out.push_str(" {\n");
                block.to_rust(out, level+1);
                pad_level(out, level);
                out.push_str("}\n");
            }
            Statement::Match { ref input, ref cases } => {
                pad_level(out, level);
                out.push_str("match ");
                out.push_str(input);
                out.push_str(" {\n");

                for case in cases {
                    pad_level(out, level+1);
                    out.push_str(&case.pattern);
                    out.push_str(" => {\n");
                    case.block.to_rust(out, level+2);
                    pad_level(out, level+1);
                    out.push_str("},\n");
                }

                pad_level(out, level);
                out.push_str("};\n");
            }
        }
    }
}
