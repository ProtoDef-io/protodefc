use itertools::Itertools;
use ::ir::name::{name, Name};
use ::nom::IResult;
use ::errors::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferencePath {
    pub up: usize,
    pub down: Vec<Name>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reference {
    Value{
        path: ReferencePath
    },
    Property {
        path: ReferencePath,
        prop: Name
    },
}

impl Reference {

    pub fn parse(input: &str) -> Result<Self> {
        match reference(input) {
            IResult::Done(_, out) => Ok(out),
            IResult::Error(err) =>
                bail!(CompilerError::NomParseError(nom_error_to_pos(&err, input.len()))),
            IResult::Incomplete(_) => unreachable!(),
        }
    }

    pub fn to_string(&self) -> String {
        match *self {
            Reference::Value { ref path } => path.to_string(),
            Reference::Property { ref path, ref prop } => {
                let mut string = path.to_string();

                string.push_str("@");
                string.push_str(&prop.0);

                string
            }
        }
    }

}

impl ReferencePath {
    pub fn to_string(&self) -> String {
        let mut string = String::new();

        for _ in 0..(self.up) {
            string.push_str("../");
        }

        for down in self.down.iter().map(|s| s.0.as_ref()).intersperse("/") {
            string.push_str(down);
        }

        string
    }
}

named!(reference<&str, Reference>, complete!(do_parse!(
    up: fold_many0!(complete!(tag_s!("../")), 0, |num, _| num + 1) >>
        down: separated_list!(tag_s!("/"), name) >>
        has_prop: opt!(complete!(tag_s!("@"))) >>
        prop: cond!(has_prop.is_some(), name) >>
        eof!() >>
        (
            if has_prop.is_some() {
                Reference::Property {
                    path: ReferencePath { up: up, down: down },
                    prop: prop.unwrap(),
                }
            } else {
                Reference::Value {
                    path: ReferencePath { up: up, down: down },
                }
            }
        )
)));

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse(input: &str) {
        assert!(Reference::parse(input).unwrap().to_string() == input);
    }

    #[test]
    fn field_references() {
        test_parse("thing");
        test_parse("../../thing");
        test_parse("@length");
        test_parse("../@length");
        test_parse("../thing@length");
    }
}
