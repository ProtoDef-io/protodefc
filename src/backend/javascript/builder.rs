
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

    pub fn if_(&mut self, cond: Expr, block: Block) {
        self.0.push(Statement::If { conds: vec![(cond, block)],
                                    else_: None, });
    }

    pub fn expr(&mut self, expr: Expr) {
        self.0.push(Statement::Expr { expr: expr, });
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
    FunctionDecl {
        name: String,
        args: Vec<String>,
        block: Block,
    },
    Expr {
        expr: Expr,
    },
}

impl ToJavascript for Statement {
    fn to_javascript(&self, out: &mut String, level: u64) {
        pad_level(out, level);

        match *self {
            Statement::Assign { ref variant, ref name, ref expr } => {
                variant.append(out);
                out.push_str(name);
                out.push_str(" = ");
                out.push_str(&expr.0);
                out.push_str(";\n");
            }
            Statement::If { ref conds, ref else_ } => {
                assert!(conds.len() >= 1);

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
                    out.push_str("}");
                }

                out.push_str("\n");
            }
            Statement::Expr { ref expr } => {
                out.push_str(&expr.0);
                out.push_str(";\n");
            }
            _ => {}
        }
    }
}

fn pad_level(string: &mut String, level: u64) {
    for i in 0..level {
        string.push_str("    ");
    }
}

trait ToJavascript {
    fn to_javascript(&self, out: &mut String, level: u64);
}

#[cfg(test)]
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
