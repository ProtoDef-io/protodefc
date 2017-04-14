use ::ir::spec::{TypeContainer, WeakTypeContainer};
use ::ir::compilation_unit::{CompilationUnit, TypeKind};
use ::ir::spec::data::{ReferencePathEntryData, ReferenceData,
                       ReferencePathEntryOperation};
use ::ir::spec::reference::ReferenceItem;
use ::errors::*;

pub fn run(cu: &CompilationUnit) -> Result<()> {
    cu.each_type(&mut |typ| {
        let mut parents: Vec<WeakTypeContainer> = Vec::new();
        let named_typ_inner = typ.borrow();
        if let TypeKind::Type(ref root_node) = named_typ_inner.typ {
            do_run(root_node, &mut parents)?
        };
        Ok(())
    })

}

fn do_run(typ: &TypeContainer, parents: &mut Vec<WeakTypeContainer>)
          -> Result<()> {
    let chain;
    let children: Vec<TypeContainer>;
    let mut references: Vec<ReferenceData>;
    {
        let inner = typ.borrow();
        children = inner.data.get_owned_children();
        references = inner.data.references.clone();

        chain = CompilerError::InsideVariant {
            variant: inner.variant.get_type(&inner.data),
        };
    }

    parents.push(typ.downgrade());
    for reference_data in &mut references {
        let up = reference_data.reference.up();
        if up > parents.len() {
            bail!(chain);
        }

        let root = parents[(parents.len() - 1) - up].clone();
        resolve(root.upgrade(), reference_data)?;
    }

    {
        typ.borrow_mut().data.references = references;
    }

    for child in &children {
        do_run(&child, parents)
            .chain_err(|| chain.clone())?;
    }
    parents.pop();

    Ok(())
}

fn resolve(root: TypeContainer, target: &mut ReferenceData) -> Result<()> {
    let mut path_entries: Vec<ReferencePathEntryData> = Vec::new();

    let mut current_type = root.borrow().data.get_result_type();
    let mut current_node = Some(root);

    for item in &target.reference.items {
        match *item {
            ReferenceItem::Down(ref name) => {
                path_entries.push(ReferencePathEntryData {
                    operation: ReferencePathEntryOperation::Down(name.0.clone()),
                    node: current_node.clone().map(|i| i.downgrade()),
                    type_spec: current_type.downgrade(),
                });

                let type_next = current_type.borrow().variant
                    .get_child_name(&name.0)
                    .ok_or_else(|| format!("type has no field '{}'", name.0))?;
                current_type = type_next;

                let mut node_next = None;
                if let Some(ref current_node_inner_rc) = current_node {
                    let node_inner = current_node_inner_rc.borrow();
                    node_next = node_inner.variant.to_variant()
                        .resolve_child_name(&node_inner.data, &name.0)
                        .ok().map(|i| i.upgrade());
                }
                current_node = node_next;
            },
            ReferenceItem::Property(ref name) => {
                if let Some(ref current_node_inner_rc) = current_node {
                    path_entries.push(ReferencePathEntryData {
                        operation: ReferencePathEntryOperation::NodeProperty(name.0.clone()),
                        node: current_node.clone().map(|i| i.downgrade()),
                        type_spec: current_type.downgrade(),
                    });

                    let node_inner = current_node_inner_rc.borrow();
                    let prop_type = node_inner.variant.to_variant()
                        .has_spec_property(&node_inner.data, &name.0)?;

                    current_type = prop_type.unwrap().upgrade()
                } else {
                    unimplemented!()
                }
                current_node = None;
            },
        }
    }

    target.path = Some(path_entries);
    Ok(())
}
