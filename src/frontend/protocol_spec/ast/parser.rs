use super::{Block, Statement, Value, Ident, Item, ItemArg};

named!(root<&str, Block>, do_parse!(
    block: block_inner >>
        space >>
        eof!() >>
        (block)
));

named!(block_inner<&str, Block>, do_parse!(
    s: many0!(complete!(statement)) >>
        (Block {
            statements: s,
        })
));

named!(statement<&str, Statement>, do_parse!(
    attributes: many0!(complete!(attribute)) >>
    items: terminated!(
        separated_nonempty_list!(contains_arrow, value),
        terminator
    ) >>
        (Statement {
            items: items,
            attributes: attributes.iter().cloned().collect(),
        })
));

named!(attribute<&str, (String, Value)>, do_parse!(
    space >>
        tag!("@") >>
        ident: base_identifier >>
        space >>
        value: value >>
        (ident.into(), value)
));

named!(statement_item<&str, Value>, do_parse!(
    // Identifier
    space >>
        ident: identifier >>

        // Optional arguments
        space >>
        has_args: opt!(tag_s!("(")) >>
        args: cond!(
            has_args.is_some(),
            separated_nonempty_list!(separator, statement_item_arg)
        ) >>
        space >>
        cond!(has_args.is_some(), tag_s!(")")) >>

        // Optional block
        space >>
        has_block: opt!(tag_s!("{")) >>
        block: cond!(has_block.is_some(), block_inner) >>
        space >>
        cond!(has_block.is_some(), tag_s!("}")) >>

        (Value::Item(Item {
            name: ident,
            args: args.unwrap_or_else(|| vec![]),
            block: block.unwrap_or_else(|| Block { statements: vec![], }),
        }))
));

named!(statement_item_arg<&str, ItemArg>, do_parse!(
    space >>
        tag: opt!(do_parse!(
            name: base_identifier >>
                colon >>
                space >>
                (name)
        )) >>
        value: value >>
        (ItemArg { tag: tag.map(|s| s.into()), value: value })
));

named!(value<&str, Value>, alt!(
    string => { |s: &str| Value::String { string: s.into(), is_block: false, } }
    | block_string => { |s: &str| Value::String { string: s.into(), is_block: true } }
    | statement_item => { |i| i }
));

named!(block_string<&str, &str>, do_parse!(
    tag_s!("\"\"\"") >>
        data: take_until_and_consume!("\"\"\"") >>
        (data)
));
named!(string<&str, &str>, delimited!(tag_s!("\""), is_not_s!("\""), tag_s!("\"")));

named!(identifier<&str, Ident>, alt!(
    base_identifier => { |s: &str| Ident::Simple(s.into()) }
    | root_ns_identifier => { |s: Vec<String>| Ident::RootNs(s) }
));
named!(base_identifier<&str, &str>, take_while1_s!(is_ident_char));
named!(root_ns_identifier<&str, Vec<String>>, many1!(do_parse!(
    space >>
        tag_s!("::") >>
        ident: base_identifier >>
        (ident.into())
)));

named!(contains_arrow<&str, ()>, do_parse!(space >> tag_s!("=>") >> ()));
named!(terminator<&str, ()>, do_parse!(space >> tag_s!(";") >> ()));
named!(separator<&str, ()>, do_parse!(space >> tag_s!(",") >> ()));
named!(space<&str, &str>, take_while!(is_space));
named!(colon<&str, ()>, do_parse!(tag_s!(":") >> ()));

fn is_ident_char(c: char) -> bool {
    let cu = c as u8;
    (cu >= 48 && cu < 58) // 0-9
        || (cu >= 65 && cu < 91) // A-Z
        || (cu >= 97 && cu < 123) // a-z
        || cu == 95 // _
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\r' || c == '\t'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_statement() {
        let a = "some => thing(\"testing\", woo: \"hoo\") => else {} => woo(\"\"\"woo\nhoo\"\"\") {};";
        match statement(a) {
            ::nom::IResult::Done(_, _) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn parse_root() {
        let a = r#"

@doc "woohoo"
namespace("thing") {

    def_type("something") {
        shadow_field("arr_len") => u8;
        field("test") => array("arr_len") => u8;
    };

};
"#;

        match root(a) {
            ::nom::IResult::Done(_, _) => (),
            _ => panic!(),
        }
    }

}
