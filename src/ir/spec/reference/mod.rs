use itertools::Itertools;
use ::ir::name::{name, Name};
use ::nom::IResult;
use ::errors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceItem {
    Down(Name),
    Property(Name),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    up: usize,
    pub items: Vec<ReferenceItem>,
}

impl Reference {

    pub fn parse(input: &str) -> Result<Self> {
        match reference(input) {
            IResult::Done(_, out) => out,
            IResult::Error(err) =>
                bail!(CompilerError::NomParseError(nom_error_to_pos(&err, input.len()))),
            IResult::Incomplete(_) => unreachable!(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();

        for is_up in (0..(self.up)).map(|_| true).intersperse(false) {
            if is_up {
                ret.push_str("..");
            } else {
                ret.push_str("/");
            }
        }

        if self.up != 0 && self.items.len() != 0 {
            ret.push_str("/");
        }

        for item in self.items.iter().map(|i| Some(i)).intersperse(None) {
            match item {
                None => ret.push_str("/"),
                Some(&ReferenceItem::Down(ref name)) => ret.push_str(name.snake()),
                Some(&ReferenceItem::Property(ref name)) => {
                    ret.push_str("@");
                    ret.push_str(name.snake());
                }
            }
        }
        ret
    }

    pub fn up(&self) -> usize {
        self.up
    }

    pub fn num_operations(&self) -> usize {
        self.items.len()
    }

}


enum ParsedReferenceItem {
    Up,
    Down(Name),
    Property(Name),
}

named!(reference<&str, Result<Reference>>, do_parse!(
    items: separated_list!(tag_s!("/"), reference_item) >>
        eof!() >>
        (make_reference(items))
));

named!(reference_item<&str, ParsedReferenceItem>, alt_complete!(
    tag_s!("..") => { |_| ParsedReferenceItem::Up }
    | do_parse!( tag_s!("@") >> name: name >> (name) ) => { |n| ParsedReferenceItem::Property(n) }
    | name => { |n| ParsedReferenceItem::Down(n) }
));

fn make_reference(mut items: Vec<ParsedReferenceItem>) -> Result<Reference> {
    let mut up: usize = 0;
    let mut allow_up = true;
    let mut out = Vec::new();

    for item in items.drain(..) {
        match item {
            ParsedReferenceItem::Up => {
                if allow_up {
                    up += 1;
                } else {
                    bail!("upward references are only allowed at beginning of reference");
                }
            }
            ParsedReferenceItem::Down(name) => {
                allow_up = false;
                out.push(ReferenceItem::Down(name));
            }
            ParsedReferenceItem::Property(name) => {
                allow_up = false;
                out.push(ReferenceItem::Property(name));
            }
        }
    }

    Ok(Reference {
        up: up,
        items: out,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(input: &str) {
        let output = Reference::parse(input).unwrap().to_string();
        println!("{:?} == {:?}", input, output);
        assert!(input == output);
    }

    #[test]
    fn successes() {
        test_parse("thing");
        test_parse("../../thing");
        test_parse("@length");
        test_parse("../@length");
        test_parse("../thing/@length");
        test_parse("../thing/@length/thing");
        test_parse("../../..");
    }

    #[test]
    #[should_panic]
    fn up_after_down() {
        test_parse("thing/../thing");
    }
}
