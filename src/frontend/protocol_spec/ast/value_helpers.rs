use super::{Item, Value};

impl Item {

    pub fn get_tagged<'a>(&'a self, tag: &str) -> Option<&'a Value> {
        for arg in &self.args {
            if let Some(ref inner_tag) = arg.tag {
                if inner_tag == tag {
                    return Some(&arg.value);
                }
            }
        }
        None
    }

    pub fn get_num<'a>(&'a self, num: usize) -> Option<&'a Value> {
        self.args.get(num).map(|v| &v.value)
    }

    pub fn validate(&self, untagged_count: usize, accepted: &[&str], required: &[&str])
                       -> Result<(), String> {

        // Untagged
        let mut is_head = true;
        let mut num_leading = 0;

        for arg in &self.args {
            if is_head && arg.tag == None {
                num_leading += 1;
            } else if !is_head && arg.tag == None {
                return Err(format!(
                    "untagged properties can only exist at the beginning"));
            } else {
                is_head = false;
            }
        }

        if untagged_count != num_leading {
            return Err(format!(
                "item must have exactly {} untagged properties, got {}",
                untagged_count, num_leading));
        }

        // Tagged
        let mut required_found: Vec<&str> = Vec::new();

        for arg in &self.args {
            if let Some(ref tag) = arg.tag {
                if !accepted.contains(&tag.as_str()) {
                    return Err(format!(
                        "{:?} is not an accepted tag for this item. (accepted: {:?})",
                        tag, accepted));
                }
                if required.contains(&tag.as_str()) {
                    if required_found.contains(&tag.as_str()) {
                        return Err(format!(
                            "tag {:?} provided more then once", tag));
                    }
                    required_found.push(tag);
                }
            }
        }

        if required.len() != required_found.len() {
            return Err(format!(
                "required tags {:?}, got only {:?}", required, required_found));
        }

        Ok(())
    }

    pub fn is_name_only(&self) -> bool {
        self.args.len() == 0 && self.block.statements.len() == 0
    }

}

#[cfg(test)]
mod tests {
    use super::super::{Ident, Item, ItemArg, Value, Block};

    macro_rules! test_constr_item_arg {
        ($tag:expr) => {
            ItemArg {
                tag: $tag,
                value: Value::String {
                    string: "".into(),
                    is_block: false,
                },
            }
        }
    }

    #[test]
    fn tag_validators() {
        let proper = Item {
            name: Ident::Simple("".into()),
            args: vec![
                test_constr_item_arg!(None),
                test_constr_item_arg!(None),
                test_constr_item_arg!(None),
                test_constr_item_arg!(Some("foo".into())),
                test_constr_item_arg!(Some("bar".into())),
            ],
            block: Block::empty(),
        };

        let improper = Item {
            name: Ident::Simple("".into()),
            args: vec![
                test_constr_item_arg!(None),
                test_constr_item_arg!(Some("foo".into())),
                test_constr_item_arg!(None),
                test_constr_item_arg!(Some("bar".into())),
            ],
            block: Block::empty(),
        };

        assert!(proper.validate(
            3,
            &["foo", "bar", "baz"],
            &["foo"]
        ).is_ok());

        assert!(proper.validate(
            3,
            &["foo", "baz"],
            &["foo"]
        ).is_err());

        assert!(proper.validate(
            3,
            &["foo", "bar", "baz"],
            &["baz"]
        ).is_err());

        assert!(proper.validate(
            2,
            &["foo", "bar", "baz"],
            &["foo"]
        ).is_err());

        assert!(improper.validate(
            1,
            &["foo", "bar"],
            &["foo"]
        ).is_err());

        assert!(improper.validate(
            2,
            &["foo", "bar"],
            &["foo"]
        ).is_err());

    }

    #[test]
    fn tag_getters() {
        let proper = Item {
            name: Ident::Simple("".into()),
            args: vec![
                test_constr_item_arg!(None),
                test_constr_item_arg!(None),
                test_constr_item_arg!(None),
                test_constr_item_arg!(Some("foo".into())),
                test_constr_item_arg!(Some("bar".into())),
            ],
            block: Block::empty(),
        };

        assert!(proper.get_num(1).is_some());
        assert!(proper.get_num(4).is_some());
        assert!(proper.get_num(8).is_none());
        assert!(proper.get_tagged("foo").is_some());
        assert!(proper.get_tagged("baz").is_none());

    }

}
