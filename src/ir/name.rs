use ::regex::Regex;
use ::std::marker::PhantomData;
use ::errors::*;
use ::inflector::cases;
use ::nom::IResult;

#[derive(Clone, PartialEq, Eq)]
pub struct Name {
    snake: String,
    pascal: String,
    camel: String,
}

// TODO REMOVE
impl PartialEq<str> for Name {
    fn eq(&self, other: &str) -> bool {
        self.snake() == other
    }
}

named!(pub name<&str, Name>, map_res!(
    take_while1_s!(is_name_char),
    |s: &str| Name::new(s.to_owned())
));

pub fn is_name_char(c: char) -> bool {
    let cu = c as u8;
    (cu >= 48 && cu < 58) // 0-9
        || (cu >= 65 && cu < 91) // A-Z
        || (cu >= 97 && cu < 123) // a-z
        || cu == 95 // _
}

impl Name {

    pub fn new(string: String) -> Result<Name> {
        lazy_static! {
            static ref RE: ::regex::Regex =
                Regex::new(r"^[a-z][a-zA-Z0-9_]*$").unwrap();
        }
        ensure!(RE.is_match(&string),
                "name is not valid, got {:?}", string);
        ensure!(cases::snakecase::is_snake_case(&string),
                "name must be snake_cased, got {:?}", string);

        Ok(Name {
            camel: cases::camelcase::to_camel_case(&string),
            pascal: cases::pascalcase::to_pascal_case(&string),
            snake: string,
        })
    }

    pub fn snake(&self) -> &str {
        &self.snake
    }
    pub fn camel(&self) -> &str {
        &self.camel
    }
    pub fn pascal(&self) -> &str {
        &self.pascal
    }

}

use ::std::fmt;
impl fmt::Debug for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "'{}'", self.pascal)
    }
}

impl<'a> From<&'a str> for Name {
    fn from(string: &str) -> Name {
        Name::new(string.to_owned()).unwrap()
    }
}
impl<'a> From<String> for Name {
    fn from(string: String) -> Name {
        Name::new(string).unwrap()
    }
}
