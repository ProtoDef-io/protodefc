use ::TypeContainer;
use ::errors::*;
use itertools::Itertools;
use ::std::fmt;

#[derive(Debug)]
pub struct CompilationUnit {
    pub namespaces: Vec<CompilationUnitNS>,
}

#[derive(Debug)]
pub struct CompilationUnitNS {
    pub path: NSPath,
    pub types: Vec<NamedType>,
}

#[derive(Debug)]
pub struct NamedType {
    pub path: TypePath,
    pub typ: TypeContainer,
    pub type_id: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypePath {
    pub path: NSPath,
    pub name: String,
}
impl fmt::Display for TypePath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.path, self.name)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NSPath {
    None,
    Path(Vec<String>),
}
impl fmt::Display for NSPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NSPath::None => write!(f, ""),
            NSPath::Path(ref path) =>
                write!(f, "::{}::", path.iter().join("::")),
        }
    }
}

impl CompilationUnit {

    pub fn new() -> CompilationUnit {
        CompilationUnit {
            namespaces: Vec::new(),
        }
    }

    pub fn compile_types(&self) -> Result<()> {
        self.each_type(|typ| {
            ::pass::assign_parent::run(&typ.typ)?;
            ::pass::assign_ident::run(&typ.typ)?;
            ::pass::resolve_reference::run(&typ.typ)?;
            ::pass::resolve_context::run(typ, self)?;
            Ok(())
        })
    }

    pub fn get_type_id(&self, path: &TypePath) -> Result<u64> {
        for ns in &self.namespaces {
            if ns.path == path.path {
                return ns.get_type_id(path);
            }
        }
        bail!("compilation unit contains no namespace '{:?}'", path);
    }

    pub fn add_namespace(&mut self, ns: CompilationUnitNS) -> Result<()> {
        if let Some(_) = self.namespaces.iter().find(|t| t.path == ns.path) {
            bail!("duplicate namespace '{:?}'", ns.path);
        }
        self.namespaces.push(ns);
        Ok(())
    }

    pub fn each_type<F>(&self, f: F) -> Result<()>
        where F: Fn(&NamedType) -> Result<()> {
        for ns in &self.namespaces {
            for typ in &ns.types {
                f(typ).chain_err(|| format!("within type '{}'", ns.path))?;
            }
        }
        Ok(())
    }

}

impl CompilationUnitNS {

    pub fn new(path: NSPath) -> CompilationUnitNS {
        CompilationUnitNS {
            path: path,
            types: Vec::new(),
        }
    }

    pub fn add_type(&mut self, typ: NamedType) -> Result<()> {
        if let Some(_) = self.types.iter().find(|t| t.path == typ.path) {
            bail!("duplicate named type '{:?}'",
                  typ.path);
        }

        self.types.push(typ);
        Ok(())
    }

    fn get_type_id(&self, path: &TypePath) -> Result<u64> {
        self.types.iter()
            .find(|typ| &typ.path == path)
            .map(|typ| typ.type_id)
            .ok_or_else(|| {
                format!("type '{}' not found", path).into()
            })
    }

}

impl NamedType {

    fn compile(&mut self) -> Result<()> {
        let name = &self.path.name;
        ::run_passes(&mut self.typ)
            .chain_err(|| format!("in named type '{}'", name))
    }

}

impl TypePath {

    pub fn with_ns(path: Vec<String>, name: String) -> TypePath {
        TypePath {
            path: NSPath::Path(path),
            name: name,
        }
    }

    pub fn with_no_ns(name: String) -> TypePath {
        TypePath {
            path: NSPath::None,
            name: name,
        }
    }

}

impl NSPath {

    pub fn new() -> NSPath {
        NSPath::None
    }

    pub fn with_path(path: Vec<String>) -> NSPath {
        NSPath::Path(path)
    }

}
