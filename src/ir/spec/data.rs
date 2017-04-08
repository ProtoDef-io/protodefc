use ::ir::compilation_unit::TypePath;
use ::ir::spec::{TypeContainer, WeakTypeContainer};
use ::ir::spec::reference::Reference;

#[derive(Debug)]
pub struct TypeData {
    pub name: TypePath,

    children: Vec<TypeContainer>,
    references: Vec<(Reference, Option<WeakTypeContainer>)>,

    /// Added in AssignParentPass
    pub parent: Option<WeakTypeContainer>,

    /// Added in AssignIdentPass
    /// Idents increase with causality.
    pub ident: Option<u64>,
}

#[derive(Debug, Copy, Clone)]
pub struct SpecChildHandle(usize);

#[derive(Debug, Copy, Clone)]
pub struct SpecReferenceHandle(usize);

impl TypeData {

    pub fn add_child(&mut self, child: TypeContainer) -> SpecChildHandle {
        let index = self.children.len();
        self.children.push(child);
        SpecChildHandle(index)
    }

    pub fn get_children<'a>(&'a self) -> &'a [TypeContainer] {
        &self.children
    }
    pub fn get_owned_children(&self) -> Vec<TypeContainer> {
        self.get_children().into()
    }

    pub fn add_reference(&mut self, reference: Reference) -> SpecReferenceHandle {
        let index = self.references.len();
        self.references.push((reference, None));
        SpecReferenceHandle(index)
    }

}

impl Default for TypeData {
    fn default() -> TypeData {
        TypeData {
            name: TypePath::with_no_ns("".to_owned()),

            children: Vec::new(),
            references: Vec::new(),

            parent: None,
            ident: None,
        }
    }
}
