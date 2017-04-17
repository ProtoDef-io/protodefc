use super::{Block, Statement, Value, Ident, Item, ItemArg};
use ::nom::IResult;

use ::errors::*;

pub fn parse(input: &str) -> Result<Block> {
    match root(input) {
        IResult::Done(_, out) => Ok(out),
        IResult::Error(err) => {
            bail!(CompilerError::NomParseError(nom_error_to_pos(&err, input.len())));
        }
        IResult::Incomplete(_) => unreachable!(),
    }
}

pub fn parse_ident(input: &str) -> Result<Ident> {
    match identifier(input) {
        IResult::Done(_, out) => Ok(out),
        IResult::Error(err) =>
            bail!(CompilerError::NomParseError(nom_error_to_pos(&err, input.len()))),
        IResult::Incomplete(_) => unreachable!(),
    }
}

named!(root<&str, Block>,
       map!(many_till!(call!(terminated_statement), call!(eof)),
            |res: (Vec<Statement>, &str)| Block { statements: res.0 }));

named!(terminated_statement<&str, Statement>, do_parse!(
    i: terminated!(complete!(statement), terminator) >>
        space >>
        (i)
));

//named!(block_inner<&str, Block>, do_parse!(
//    s: complete!(many0!(terminated!(complete!(statement), terminator))) >>
//        (Block {
//            statements: s,
//        })
//));

named!(statement_items<&str, Vec<Value>>,
       separated_nonempty_list!(contains_arrow, value));

named!(statement<&str, Statement>, do_parse!(
    attributes: many0!(complete!(attribute)) >>
        items: complete!(statement_items) >>
        (Statement {
            items: items,
            attributes: attributes.iter().cloned().collect(),
        })
));

named!(attribute<&str, (String, Vec<Value>)>, do_parse!(
    space >>
        tag!("@") >>
        ident: base_identifier >>
        space >>
        value: statement_items >>
        (ident.into(), value)
));

named!(statements_until_block_close<&str, Vec<Statement>>,
       map!(many_till!(call!(terminated_statement), tag_s!("}")),
            |i: (Vec<Statement>, &str)| i.0));

named!(statement_item<&str, Item>, do_parse!(
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
        block: cond!(has_block.is_some(), statements_until_block_close) >>
        space >>
        cond!(has_block.is_some(), tag_s!("}")) >>

        (Item {
            name: ident,
            args: args.unwrap_or_else(|| vec![]),
            block: block
                .map(|i| Block { statements: i })
                .unwrap_or_else(|| Block { statements: vec![], }),
        })
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
    | statement_item => { |i| Value::Item(i) }
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
named!(eof<&str, &str>, eof!());

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
