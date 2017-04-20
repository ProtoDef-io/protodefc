use ::ir::compilation_unit::TypePath;
use ::ir::spec::{TypeContainer, WeakTypeContainer};
use ::ir::spec::reference::Reference;
use ::ir::type_spec::{TypeSpecContainer, WeakTypeSpecContainer};
use ::ir::type_spec::property::TypeSpecProperty;
use ::ir::name::Name;

#[derive(Debug)]
pub struct TypeData {
    pub name: TypePath,

    children: Vec<TypeContainer>,
    pub references: Vec<ReferenceData>,

    /// Added in AssignParentPass
    pub parent: Option<WeakTypeContainer>,

    /// Added in AssignIdentPass
    /// Idents increase with causality.
    pub ident: Option<u64>,

    pub type_spec: Option<TypeSpecContainer>,
}

#[derive(Debug, Copy, Clone)]
pub struct SpecChildHandle(usize);

#[derive(Debug, Copy, Clone)]
pub struct SpecReferenceHandle(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReferenceAccessTime {
    Read,
    ReadWrite,
}

#[derive(Debug, Clone)]
pub struct ReferenceData {
    pub access_time: ReferenceAccessTime,
    pub reference: Reference,
    pub path: Option<Vec<ReferencePathEntryData>>,
    pub target_type_spec: Option<TypeSpecContainer>,
}
#[derive(Debug, Clone)]
pub struct ReferencePathEntryData {
    pub node: Option<WeakTypeContainer>,
    pub type_spec: WeakTypeSpecContainer,
    pub operation: ReferencePathEntryOperation,
}
#[derive(Debug, Clone)]
pub enum ReferencePathEntryOperation {
    Down(Name),
    NodeProperty(Name),
    TypeProperty(Name),
    TypeSpecProperty(TypeSpecProperty),
}

impl TypeData {

    pub fn get_result_type(&self) -> TypeSpecContainer {
        self.type_spec.clone().expect("type not assigned yet")
    }

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

    pub fn add_reference(&mut self, reference: Reference,
                         access_time: ReferenceAccessTime) -> SpecReferenceHandle {
        let index = self.references.len();
        self.references.push(ReferenceData {
            reference: reference,
            path: None,
            access_time: access_time,
            target_type_spec: None,
        });
        SpecReferenceHandle(index)
    }

    pub fn get_reference<'a>(&'a self, handle: SpecReferenceHandle) -> &'a Reference {
        &self.references[handle.0].reference
    }
    pub fn get_reference_root(&self, handle: SpecReferenceHandle) -> WeakTypeContainer {
        self.references[handle.0].path.as_ref().unwrap()[0].node.clone().unwrap()
    }
    pub fn get_reference_data<'a>(&'a self, handle: SpecReferenceHandle)
                                  -> &'a ReferenceData {
        &self.references[handle.0]
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

            type_spec: TypeSpecContainer::new_not_assigned(),
        }
    }
}

impl ReferenceData {

    pub fn get_path<'a>(&'a self) -> &'a [ReferencePathEntryData] {
        self.path.as_ref().unwrap()
    }

}
