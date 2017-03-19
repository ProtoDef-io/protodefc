use super::{Block, Statement, Value, Ident, Item, ItemArg};

pub fn print(block: &Block) -> String {
    let mut out = String::new();

    print_block(block, &mut out, 0);

    out
}

fn pad_level(string: &mut String, level: u64) {
    for _ in 0..level {
        string.push_str("    ");
    }
}

fn print_block(block: &Block, out: &mut String, level: u64) {
    for statement in &block.statements {
        print_statement(statement, out, level);
    }
}

fn print_statement(stmt: &Statement, out: &mut String, level: u64) {
    for (attr_name, attr_value) in &stmt.attributes {
        pad_level(out, level);
        out.push_str("@");
        out.push_str(attr_name);
        out.push_str(" ");
        print_value(attr_value, out, level);
        out.push_str("\n");
    }

    pad_level(out, level);
    for (idx, item) in stmt.items.iter().enumerate() {
        print_value(item, out, level);
        if idx != stmt.items.len() - 1 {
            out.push_str(" => ");
        }
    }
    out.push_str(";\n");
}

fn print_value(value: &Value, out: &mut String, level: u64) {
    match *value {
        Value::String { is_block: false, ref string } => {
            out.push_str("\"");
            out.push_str(string);
            out.push_str("\"");
        },
        Value::Item(Item { ref name, ref args, ref block }) => {
            print_ident(name, out, level);

            // Arguments
            if args.len() != 0 {
                out.push_str("(");
                for (idx, &ItemArg { ref tag, ref value }) in args.iter().enumerate() {
                    if let &Some(ref tag_i) = tag {
                        out.push_str(tag_i);
                        out.push_str(": ");
                    }
                    print_value(&value, out, level);
                    if idx != args.len() - 1 {
                        out.push_str(", ");
                    }
                }
                out.push_str(")");
            }

            // Block
            if block.statements.len() != 0 {
                out.push_str(" {\n");

                print_block(&block, out, level+1);

                pad_level(out, level);
                out.push_str("}");
            }
        },
        _ => unimplemented!(),
    }
}

fn print_ident(ident: &Ident, out: &mut String, level: u64) {
    match *ident {
        Ident::Simple(ref string) => {
            out.push_str(string);
        }
        Ident::RootNs(ref path) => {
            for node in path {
                out.push_str("::");
                out.push_str(node);
            }
        }
    }
}
