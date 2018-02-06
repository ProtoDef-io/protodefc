use ::TypeContainer;
use ::errors::*;
use ::std::collections::HashMap;

use ::rc_container::{Container, WeakContainer};
use ::ir::type_spec::TypeSpecContainer;
use ::ir::spec::data::ReferenceAccessTime;

mod path;
pub use self::path::{CanonicalNSPath, RelativeNSPath, TypePath};

mod defined_spec;
pub use self::defined_spec::{ NamedTypeContainer, WeakNamedTypeContainer, NamedType,
                              TypeKind, NativeType, NamedTypeArgument };
mod defined_type_spec;
pub use self::defined_type_spec::{ DefinedTypeSpecContainer };

#[derive(Debug)]
pub struct CompilationUnit {
    pub namespaces: Vec<CompilationUnitNS>,
    pub exports: HashMap<String, NamedTypeContainer>,
}

#[derive(Debug)]
pub struct CompilationUnitNS {
    pub path: CanonicalNSPath,
    pub defines: Vec<DefinedItem>,
    //pub specs: Vec<NamedTypeContainer>,
    //pub defined_type_specs: Vec<DefinedTypeSpecContainer>,
    pub exports: HashMap<String, NamedTypeContainer>,
}

#[derive(Debug)]
pub struct DefinedItem {
    pub path: TypePath,
    pub item: DefinedItemType,
}
#[derive(Debug)]
pub enum DefinedItemType {
    Spec(NamedTypeContainer),
}
impl DefinedItemType {
    pub fn as_spec<'a>(&'a self) -> Option<&'a NamedTypeContainer> {
        match self {
            &DefinedItemType::Spec(ref inner) => Some(inner),
            _ => None,
        }
    }
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
        where F: FnMut(&DefinedItem) -> Result<()> {
        for ns in &self.namespaces {
            for typ in &ns.defines {
                f(typ).chain_err(|| format!("within type '{}'", typ.path))?;
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
            match typ.item {
                DefinedItemType::Spec(ref inner) => {
                    let root = {
                        inner.borrow().typ.clone()
                    };

                    if let TypeKind::Type(ref container) = root {
                        traverse_type(&inner, container, f)?;
                    };
                },
                _ => (),
            }
            Ok(())
        })
    }

    pub fn resolve_type<'a>(&'a self, path: &TypePath) -> Result<&'a DefinedItem> {
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
            defines: Vec::new(),
            exports: HashMap::new(),
        }
    }

    pub fn add_type(&mut self, typ: NamedType) -> Result<()> {
        if let Some(_) = self.defines.iter().find(|t| t.path == typ.path) {
            bail!("duplicate named type '{:?}'",
                  typ.path);
        }

        let path = typ.path.clone();
        let export = typ.export.clone();
        let container = NamedTypeContainer::new(typ);

        if let Some(name) = export {
            ensure!(!self.exports.contains_key(&name),
                    "duplicate export '{}'", name);
            self.exports.insert(name, container.clone());
        }

        self.defines.push(DefinedItem {
            path: path,
            item: DefinedItemType::Spec(container),
        });
        Ok(())
    }

    fn get_type_id(&self, path: &TypePath) -> Result<u64> {
        self.defines.iter()
            .find(|typ| &typ.path == path)
            .map(|typ| match typ.item {
                DefinedItemType::Spec(ref i) => i.borrow().type_id,
                _ => panic!(),
            })
            .ok_or_else(|| {
                format!("type '{}' not found", path).into()
            })
    }

    fn resolve_type<'a>(&'a self, path: &TypePath) -> Result<&'a DefinedItem> {
        self.defines.iter()
            .find(|typ| &typ.path == path)
            .ok_or(format!("type '{}' not found", path).into())
            .map(|t| t.clone())
    }

    pub fn specs_iter<'a>(&'a self) -> Box<Iterator<Item = &'a NamedTypeContainer> + 'a> {
        Box::new(
            self.defines.iter().flat_map(|item| item.item.as_spec())
        )
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
