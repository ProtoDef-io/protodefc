use std::collections::HashMap;

use ::TypeContainer;
use ::ir::variant::Variant;
use super::ast::{Block, Statement, Value, Ident, Item};

fn ir_to_spec(type_name: String, typ: TypeContainer) -> Statement {
    let mut statement = ir_to_spec_inner(typ);

    statement.items.insert(
        0,
        Value::Item(Item {
            name: Ident::Simple("def_type".into()),
            args: vec![Value::String {
                string: type_name,
                is_block: false,
            }
                       .into()],
            block: Block::empty(),
        })
    );

    statement
}

fn ir_to_spec_inner(typ: TypeContainer) -> Statement {
    let typ_inner = typ.borrow();

    let typ_variant = &typ_inner.variant;
    let typ_data = &typ_inner.data;

    match *typ_variant {

        Variant::SimpleScalar(_) => {
            Statement {
                attributes: HashMap::new(),
                items: vec![
                    Value::Item(Item {
                        name: Ident::RootNs(vec![
                            "native".into(),
                            typ_data.name.to_string()
                        ]),
                        args: vec![],
                        block: Block::empty(),
                    })
                ],
            }
        }

        Variant::Container(ref inner) => {
            Statement {
                attributes: HashMap::new(),
                items: vec![
                    Value::Item(Item {
                        name: Ident::Simple("container".into()),
                        args: vec![],
                        block: Block {
                            statements: inner.fields
                                .iter()
                                .map(|field| {
                                    let child = field.child.upgrade();
                                    let mut statement = ir_to_spec_inner(child);

                                    statement.items.insert(
                                        0,
                                        Value::Item(Item {
                                            name: Ident::Simple("field".into()),
                                            args: vec![Value::String {
                                                string: field.name
                                                    .to_string(),
                                                is_block: false,
                                            }
                                                       .into()],
                                            block: Block::empty(),
                                        })
                                    );

                                    statement
                                })
                                .collect(),
                        },
                    })
                ],
            }
        }

        _ => unimplemented!(),

    }
}

#[cfg(test)]
mod tests {
    use super::ir_to_spec;

    use ::frontend::protocol_spec;
    use self::protocol_spec::ast::printer::print;
    use self::protocol_spec::ast::Block;

    //#[test]
    //fn basic_container() {
    //    let json = "[\"container\", [{\"name\": \"foo\", \"type\": \"u8\"}]]";
    //    let ast = ::json_to_final_ast(json).unwrap();
    //    let spec_ast = ir_to_spec("test_type".into(), ast);
    //    let spec = print(&Block { statements: vec![spec_ast] });
    //    println!("{}", spec);
    //}

}
