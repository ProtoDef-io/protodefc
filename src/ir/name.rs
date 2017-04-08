#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(pub String);

impl<'a> PartialEq<&'a str> for Name {
    fn eq(&self, rhs: &&'a str) -> bool {
        self.0 == *rhs
    }
}

named!(pub name<&str, Name>, map!(
    take_while1_s!(is_name_char),
    |s: &str| Name(s.to_owned())
));

pub fn is_name_char(c: char) -> bool {
    let cu = c as u8;
    (cu >= 48 && cu < 58) // 0-9
        || (cu >= 65 && cu < 91) // A-Z
        || (cu >= 97 && cu < 123) // a-z
        || cu == 95 // _
}
