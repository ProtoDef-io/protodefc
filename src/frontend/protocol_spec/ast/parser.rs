use super::{Block, Ident};
use ::errors::*;
use itertools::Itertools;

pub fn parse(input: &str) -> Result<Block> {
    match pds::root(input) {
        Ok(i) => Ok(i),
        Err(err) => bail!("parse error at {}:{}", err.line, err.column),
    }
}

pub fn parse_ident(input: &str) -> Result<Ident> {
    match pds::identifier(input) {
        Ok(i) => Ok(i),
        Err(_) => bail!("expected namespaced identifier"),
    }
}

fn format_error(input: &str, error: pds::ParseError) -> String {
    let line_num = error.line - 1;
    let first_line = if line_num >= 2 { line_num - 2 } else { 0 };

    let lines = input.lines()
        .skip(first_line).take(2)
        .map(|s| s.to_owned()).collect::<Vec<String>>();
    let lines_f = lines.iter()
        .join("\n");

    let pointer_padding = (0..error.column+1).map(|_| " ").join("");
    let pointer = "^";

    println!("{:?}", error);
    format!("{}\n{}{}", lines_f, pointer_padding, pointer)
}

mod pds {
    include!(concat!(env!("OUT_DIR"), "/pds.rs"));
}
