
pub struct Block(Vec<Statement>);

impl Block {

    pub fn new() -> Block {
        Block(Vec::new())
    }

    pub fn assign(&mut self, name: String, expr: Expr) {
        self.0.push(Statement::Assign { variant: AssignVariant::None,
                                        name: name, expr: expr, });
    }
    pub fn var_assign(&mut self, name: String, expr: Expr) {
        self.0.push(Statement::Assign { variant: AssignVariant::Var,
                                        name: name, expr: expr, });
    }
    pub fn let_assign(&mut self, name: String, expr: Expr) {
        self.0.push(Statement::Assign { variant: AssignVariant::Let,
                                        name: name, expr: expr, });
    }

    pub fn if_(&mut self, cond: Expr, block: Block) {
        self.0.push(Statement::If { conds: vec![(cond, block)],
                                    else_: None, });
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

    pub fn scope(&mut self, block: Block) {
        self.0.push(Statement::Scope { block: block });
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

    pub fn for_(&mut self, init: Expr, cond: Expr, incr: Expr, block: Block) {
        self.0.push(Statement::For {
            init: init,
            cond: cond,
            incr: incr,
            block: block,
        });
    }

}

impl ToJavascript for Block {
    fn to_javascript(&self, out: &mut String, level: u64) {
        for statement in &self.0 {
            statement.to_javascript(out, level);
        }
    }
}

pub struct Expr(String);
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

#[derive(Debug, Copy, Clone)]
pub enum AssignVariant {
    None,
    Var,
    Let,
    Const,
}
impl AssignVariant {
    fn append(self, out: &mut String) {
        let s = match self {
            AssignVariant::None => "",
            AssignVariant::Var => "var ",
            AssignVariant::Let => "let ",
            AssignVariant::Const => "const ",
        };
        out.push_str(s);
    }
}

pub enum Statement {
    Assign {
        variant: AssignVariant,
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
    Scope {
        block: Block,
    },
    Block {
        block: Block,
    },
    Comment {
        text: String,
    },
    For {
        init: Expr,
        cond: Expr,
        incr: Expr,
        block: Block,
    },
    //Switch {

    //},
}

impl ToJavascript for Statement {
    fn to_javascript(&self, out: &mut String, level: u64) {

        match *self {
            Statement::Assign { ref variant, ref name, ref expr } => {
                pad_level(out, level);

                variant.append(out);
                out.push_str(name);
                out.push_str(" = ");
                out.push_str(&expr.0);
                out.push_str(";\n");
            }
            Statement::If { ref conds, ref else_ } => {
                assert!(conds.len() >= 1);

                pad_level(out, level);

                let mut first = true;
                for &(ref cond, ref block) in conds {

                    if first {
                        out.push_str("if (");
                    } else {
                        out.push_str(" else if (");
                    }
                    first = false;

                    out.push_str(&cond.0);
                    out.push_str(") {\n");

                    block.to_javascript(out, level+1);

                    pad_level(out, level);
                    out.push_str("}");
                }

                if let &Some(ref block) = else_ {
                    out.push_str(" else {\n");
                    block.to_javascript(out, level+1);

                    pad_level(out, level);
                    out.push_str("}");
                }

                out.push_str("\n");
            }
            Statement::FunctionDeclaration { ref name, ref args, ref block } => {
                pad_level(out, level);

                out.push_str("function ");
                out.push_str(name);
                out.push_str("(");
                if args.len() > 0 {
                    for num in 0..args.len()-1 {
                        out.push_str(&args[num]);
                        out.push_str(", ");
                    }
                    out.push_str(args.last().unwrap());
                }
                out.push_str(") {\n");
                block.to_javascript(out, level+1);

                pad_level(out, level);
                out.push_str("}\n");
            }
            Statement::Expr { ref expr } => {
                pad_level(out, level);

                out.push_str(&expr.0);
                out.push_str(";\n");
            }
            Statement::Scope { ref block } => {
                pad_level(out, level);

                out.push_str("{\n");
                block.to_javascript(out, level+1);
                pad_level(out, level);
                out.push_str("}\n");
            }
            Statement::Block { ref block } => {
                block.to_javascript(out, level);
            }
            Statement::Return { ref expr } => {
                pad_level(out, level);
                out.push_str("return ");
                out.push_str(&expr.0);
                out.push_str(";\n");
            }
            Statement::Comment { ref text } => {
                pad_level(out, level);
                out.push_str("// ");
                out.push_str(text);
                out.push_str("\n");
            }
            Statement::For { ref init, ref cond, ref incr, ref block } => {
                pad_level(out, level);
                out.push_str("for (");
                out.push_str(&init.0);
                out.push_str("; ");
                out.push_str(&cond.0);
                out.push_str("; ");
                out.push_str(&incr.0);
                out.push_str(") {\n");

                block.to_javascript(out, level+1);

                pad_level(out, level);
                out.push_str("}\n");
            }
            //_ => unimplemented!(),
        }
    }
}

fn pad_level(string: &mut String, level: u64) {
    for _ in 0..level {
        string.push_str("    ");
    }
}

pub trait ToJavascript {
    fn to_javascript(&self, out: &mut String, level: u64);
}

#[cfg(all(test, feature = "js_tests"))]
mod tests {

    use super::Block;
    use super::ToJavascript;

    fn to_string<T>(block: T) -> String where T: ToJavascript {
        let mut out = String::new();
        block.to_javascript(&mut out, 0);
        out
    }

    #[test]
    fn simple_expr() {
        let mut b = Block::new();
        b.expr("some_call(1)".into());
        assert_eq!(to_string(b), "some_call(1);\n");
    }

    #[test]
    fn simple_if() {
        let mut b = Block::new();
        b.if_("test == 0".into(), {
            let mut b = Block::new();
            b.assign("test".into(), "1".into());
            b
        });
        assert_eq!(to_string(b), "if (test == 0) {\n    test = 1;\n}\n");
    }

}
