#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldReference {
    pub up: usize,
    pub name: String,
}

impl FieldReference {

    pub fn parse(mut source: &str) -> Option<FieldReference> {
        let mut up = 0;
        while source.starts_with("../") {
            up += 1;
            source = &source[3..];
        }

        for character in source.chars() {
            if !(character.is_digit(36) || character == '_') {
                return None;
            }
        }

        Some(FieldReference {
            up: up,
            name: source.to_string(),
        })
    }

}

#[cfg(test)]
mod tests {
    use super::FieldReference;

    #[test]
    fn without_up() {
        let parsed = FieldReference::parse("some_field").unwrap();
        assert_eq!(parsed.up, 0);
        assert_eq!(parsed.name, "some_field");
    }

    #[test]
    fn invalid_characters() {
        assert_eq!(FieldReference::parse("some-field"), None);
        assert_eq!(FieldReference::parse("some.field"), None);
    }

    #[test]
    fn with_up() {
        let parsed = FieldReference::parse("../some_field").unwrap();
        assert_eq!(parsed.up, 1);
        assert_eq!(parsed.name, "some_field");

        let parsed = FieldReference::parse("../../../something").unwrap();
        assert_eq!(parsed.up, 3);
        assert_eq!(parsed.name, "something");
    }

}
