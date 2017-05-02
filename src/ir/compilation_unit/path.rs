use ::errors::*;
use ::std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CanonicalNSPath(Vec<String>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RelativeNSPath {
    Relative(Vec<String>),
    Absolute(Vec<String>),
}

impl fmt::Display for CanonicalNSPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "::")?;
        for element in &self.0 {
            write!(f, "{}::", element)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypePath {
    pub path: CanonicalNSPath,
    pub name: String,
}

impl fmt::Display for TypePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.path, self.name)
    }
}

impl fmt::Display for RelativeNSPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let elements = match *self {
            RelativeNSPath::Absolute(ref path) => {
                write!(f, "::")?;
                path
            }
            RelativeNSPath::Relative(ref path) => path
        };

        for element in elements {
            write!(f, "{}::", element)?;
        }
        Ok(())
    }
}

impl TypePath {

    pub fn new(ns: CanonicalNSPath, name: String) -> TypePath {
        TypePath {
            path: ns,
            name: name,
        }
    }

}

impl CanonicalNSPath {

    pub fn root() -> CanonicalNSPath {
        CanonicalNSPath(vec![])
    }

    pub fn concat(&self, other: &RelativeNSPath) -> RelativeNSPath {
        match *other {
            RelativeNSPath::Relative(ref path) => {
                let mut base = self.0.clone();
                base.append(&mut path.clone());
                RelativeNSPath::Absolute(base)
            }
            RelativeNSPath::Absolute(ref path) =>
                RelativeNSPath::Absolute(path.clone())
        }
    }

    pub fn from_absolute_path(path: Vec<String>) -> CanonicalNSPath {
        CanonicalNSPath(path)
    }

}

impl RelativeNSPath {

    pub fn empty() -> RelativeNSPath {
        RelativeNSPath::Relative(vec![])
    }
    pub fn root() -> RelativeNSPath {
        RelativeNSPath::Absolute(vec![])
    }
    pub fn simple(string: String) -> RelativeNSPath {
        RelativeNSPath::Relative(vec![string])
    }
    pub fn with_absolute_path(path: Vec<String>) -> RelativeNSPath {
        RelativeNSPath::Absolute(path)
    }
    pub fn with_relative_path(path: Vec<String>) -> RelativeNSPath {
        RelativeNSPath::Relative(path)
    }

    pub fn elements<'a>(&'a self) -> &'a [String] {
        match *self {
            RelativeNSPath::Absolute(ref inner) => inner,
            RelativeNSPath::Relative(ref inner) => inner,
        }
    }

    pub fn concat(&self, other: &RelativeNSPath) -> RelativeNSPath {
        match *other {
            RelativeNSPath::Absolute(ref path) =>
                RelativeNSPath::Absolute(path.clone()),
            RelativeNSPath::Relative(ref path_ext) => {
                let mut base: Vec<String> = self.elements().into();
                base.append(&mut path_ext.clone());

                match *self {
                    RelativeNSPath::Absolute(_) =>
                        RelativeNSPath::Absolute(base),
                    RelativeNSPath::Relative(_) =>
                        RelativeNSPath::Relative(base),
                }
            },
        }
    }

    pub fn into_canonical(self) -> Result<CanonicalNSPath> {
        match self {
            RelativeNSPath::Relative(_) => bail!(
                "cannot turn relative ns path into canonical ns path"),
            RelativeNSPath::Absolute(inner) => Ok(CanonicalNSPath(inner)),
        }
    }

    pub fn into_type_path(self) -> Result<TypePath> {
        match self {
            RelativeNSPath::Relative(_) => bail!(
                "cannot turn relative ns path into canonical ns path"),
            RelativeNSPath::Absolute(mut inner) => {
                let name = inner.pop()
                    .ok_or("absolute ns path needs at least one entry to be turned into type path")?;
                Ok(TypePath::new(CanonicalNSPath::from_absolute_path(inner), name))
            },
        }
    }

    // TODO
    pub fn simple_str<'a>(&'a self) -> Option<&'a str> {
        match *self {
            RelativeNSPath::Absolute(_) => None,
                //bail!("absolute path cannot be used as simple string"),
            RelativeNSPath::Relative(ref inner) => {
                //ensure!(inner.len() == 1,
                //        "relative path must only have 1 entry to be used as simple string");
                if inner.len() != 1 {
                    return None;
                }
                Some(inner[0].as_ref())
            }
        }
    }

}
