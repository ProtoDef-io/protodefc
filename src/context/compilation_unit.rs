use ::TypeContainer;
use ::errors::*;
use itertools::Itertools;
use ::std::fmt;

use ::rc_container::{Container, WeakContainer};
use ::ir::TargetType;

pub type NamedTypeContainer = Container<NamedType>;
pub type WeakNamedTypeContainer = WeakContainer<NamedType>;

#[derive(Debug)]
pub struct CompilationUnit {
    pub namespaces: Vec<CompilationUnitNS>,
}

#[derive(Debug)]
pub struct CompilationUnitNS {
    pub path: NSPath,
    pub types: Vec<NamedTypeContainer>,
}

#[derive(Debug)]
pub struct NamedType {
    pub path: TypePath,
    pub typ: TypeKind,
    pub type_id: u64,
}

#[derive(Debug)]
pub enum TypeKind {
    Native(TargetType),
    Type(TypeContainer),
}

#[derive(Debug)]
pub struct NativeType {
    pub path: TypePath,
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
            NSPath::Path(ref path) => {
                if path.len() == 0 {
                    write!(f, "::")
                } else {
                    let joined: String = path.iter().join("::");
                    write!(f, "::{}::", joined)
                }
            }
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
        self.each_type(&mut |typ| {
            if let TypeKind::Type(ref container) = typ.borrow().typ {
                ::pass::assign_parent::run(container)?;
                ::pass::assign_ident::run(container)?;
                ::pass::resolve_context::run(typ, self)?;
            }
            Ok(())
        })?;

        ::pass::propagate_types::run(self)?;

        self.each_type(&mut |typ| {
            if let TypeKind::Type(ref container) = typ.borrow().typ {
                ::pass::resolve_reference::run(container)?;
            }
            Ok(())
        })?;

        Ok(())
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

    pub fn each_type<F>(&self, f: &mut F) -> Result<()>
        where F: FnMut(&NamedTypeContainer) -> Result<()> {
        for ns in &self.namespaces {
            for typ in &ns.types {
                f(typ).chain_err(|| format!("within type '{}'", typ.borrow().path))?;
            }
        }
        Ok(())
    }

    pub fn resolve_type(&self, path: &TypePath) -> Result<NamedTypeContainer> {
        self.namespaces.iter()
            .find(|ns| ns.path == path.path)
            .ok_or_else(|| format!("no type '{}' in compilation unit", path).into())
            .and_then(|ns| ns.resolve_type(path))
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
        if let Some(_) = self.types.iter().find(|t| t.borrow().path == typ.path) {
            bail!("duplicate named type '{:?}'",
                  typ.path);
        }

        self.types.push(NamedTypeContainer::new(typ));
        Ok(())
    }

    fn get_type_id(&self, path: &TypePath) -> Result<u64> {
        self.types.iter()
            .find(|typ| &typ.borrow().path == path)
            .map(|typ| typ.borrow().type_id)
            .ok_or_else(|| {
                format!("type '{}' not found", path).into()
            })
    }

    fn resolve_type(&self, path: &TypePath) -> Result<NamedTypeContainer> {
        self.types.iter()
            .find(|typ| &typ.borrow().path == path)
            .ok_or(format!("type '{}' not found", path).into())
            .map(|t| t.clone())
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

    pub fn in_context(&self, path: &NSPath) -> TypePath {
        match self.path {
            NSPath::Path(_) => self.clone(),
            NSPath::None => match *path {
                NSPath::None => self.clone(),
                NSPath::Path(_) => TypePath {
                    path: path.clone(),
                    name: self.name.clone(),
                },
            },
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

impl TypeKind {

    pub fn get_result_type(&self) -> Option<TargetType> {
        match *self {
            TypeKind::Native(ref typ) => Some(typ.clone()),
            TypeKind::Type(ref container) => {
                let inner = container.borrow();
                inner.variant.to_variant().get_result_type(&inner.data)
            }
        }
    }

}
