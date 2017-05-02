use ::TypeContainer;
use ::errors::*;
use itertools::Itertools;
use ::std::fmt;
use ::std::collections::HashMap;

use ::rc_container::{Container, WeakContainer};
use ::ir::type_spec::TypeSpecContainer;
use ::ir::spec::data::ReferenceAccessTime;

pub type NamedTypeContainer = Container<NamedType>;
pub type WeakNamedTypeContainer = WeakContainer<NamedType>;

mod path;
pub use self::path::{CanonicalNSPath, RelativeNSPath, TypePath};

#[derive(Debug)]
pub struct CompilationUnit {
    pub namespaces: Vec<CompilationUnitNS>,
    pub exports: HashMap<String, NamedTypeContainer>,
}

#[derive(Debug)]
pub struct CompilationUnitNS {
    pub path: CanonicalNSPath,
    pub types: Vec<NamedTypeContainer>,
    pub exports: HashMap<String, NamedTypeContainer>,
}

#[derive(Debug)]
pub struct NamedType {
    pub path: TypePath,

    pub typ: TypeKind,
    pub type_spec: TypeSpecContainer,
    pub type_id: u64,

    pub arguments: Vec<NamedTypeArgument>,

    pub export: Option<String>,

    pub docstring: String,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Native(NativeType),
    Type(TypeContainer),
}

#[derive(Debug, Clone)]
pub struct NativeType {
    pub type_spec: TypeSpecContainer,
}
#[derive(Debug, Clone)]
pub struct NamedTypeArgument {
    pub name: String,
    pub access_time: ReferenceAccessTime,
    pub type_spec: TypeSpecContainer,
}

impl CompilationUnit {

    pub fn new() -> CompilationUnit {
        CompilationUnit {
            namespaces: Vec::new(),
            exports: HashMap::new(),
        }
    }

    pub fn compile_types(&mut self) -> Result<()> {
        ::pass::run_passes(self)
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

        for (name, typ) in &ns.exports {
            ensure!(!self.exports.contains_key(name),
                    "duplicate export '{}'", name);
            self.exports.insert(name.clone(), typ.clone());
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

    pub fn each_type_traverse_node<F>(&self, f: &mut F) -> Result<()>
        where F: FnMut(&NamedTypeContainer, &TypeContainer) -> Result<()> {

        fn traverse_type<I>(cont: &NamedTypeContainer, typ: &TypeContainer, f: &mut I) -> Result<()>
            where I: FnMut(&NamedTypeContainer, &TypeContainer) -> Result<()> {

            let children = typ.borrow().data.get_owned_children();

            f(cont, typ)?;

            for child in &children {
                traverse_type(cont, child, f)?;
            }

            Ok(())
        }

        self.each_type(&mut |typ| {
            let root = {
                typ.borrow().typ.clone()
            };

            if let TypeKind::Type(ref container) = root {
                traverse_type(typ, container, f)?;
            };

            Ok(())
        })
    }

    pub fn resolve_type(&self, path: &TypePath) -> Result<NamedTypeContainer> {
        self.namespaces.iter()
            .find(|ns| ns.path == path.path)
            .ok_or_else(|| format!("no type '{}' in compilation unit", path).into())
            .and_then(|ns| ns.resolve_type(path))
    }

}

impl CompilationUnitNS {

    pub fn new(path: CanonicalNSPath) -> CompilationUnitNS {
        CompilationUnitNS {
            path: path,
            types: Vec::new(),
            exports: HashMap::new(),
        }
    }

    pub fn add_type(&mut self, typ: NamedType) -> Result<()> {
        if let Some(_) = self.types.iter().find(|t| t.borrow().path == typ.path) {
            bail!("duplicate named type '{:?}'",
                  typ.path);
        }

        let export = typ.export.clone();
        let container = NamedTypeContainer::new(typ);

        if let Some(name) = export {
            ensure!(!self.exports.contains_key(&name),
                    "duplicate export '{}'", name);
            self.exports.insert(name, container.clone());
        }

        self.types.push(container);
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

impl TypeKind {

    pub fn get_result_type(&self) -> Option<TypeSpecContainer> {
        match *self {
            TypeKind::Native(ref typ) => Some(typ.type_spec.clone()),
            TypeKind::Type(ref container) => {
                let inner = container.borrow();
                Some(inner.data.get_result_type())
            }
        }
    }

}
