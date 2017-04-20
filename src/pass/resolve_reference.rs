use ::ir::spec::{TypeContainer, WeakTypeContainer};
use ::ir::type_spec::TypeSpecContainer;
use ::ir::compilation_unit::{CompilationUnit, TypeKind};
use ::ir::spec::data::{ReferencePathEntryData, ReferenceData,
                       ReferencePathEntryOperation};
use ::ir::spec::reference::ReferenceItem;
use ::errors::*;

struct ResolveReferenceState {
    unfinished: bool,
    changed: bool,
}
impl ResolveReferenceState {
    pub fn mark_unfinished(&mut self) {
        self.unfinished = true;
    }
    pub fn mark_changed(&mut self) {
        self.changed = true;
    }
}
impl Default for ResolveReferenceState {
    fn default() -> ResolveReferenceState {
        ResolveReferenceState {
            unfinished: false,
            changed: false,
        }
    }
}

pub fn run(cu: &CompilationUnit) -> Result<()> {

    cu.each_type(&mut |typ| {
        let mut parents: Vec<WeakTypeContainer> = Vec::new();
        let named_typ_inner = typ.borrow();
        if let TypeKind::Type(ref root_node) = named_typ_inner.typ {
            let mut pass_data = ResolveReferenceState::default();
            pass_data.mark_changed();
            pass_data.mark_unfinished();

            while pass_data.changed && pass_data.unfinished {
                pass_data = ResolveReferenceState::default();
                do_run(root_node, &mut parents, &mut pass_data)?
            }

            if pass_data.unfinished {
                bail!("could not resolve all references");
            }
        };
        Ok(())
    })?;

    Ok(())
}

fn do_run(typ: &TypeContainer, parents: &mut Vec<WeakTypeContainer>,
          pass: &mut ResolveReferenceState) -> Result<()> {
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
        resolve(root.upgrade(), reference_data, pass)?;
    }

    {
        typ.borrow_mut().data.references = references;
    }

    for child in &children {
        do_run(&child, parents, pass)
            .chain_err(|| chain.clone())?;
    }
    parents.pop();

    Ok(())
}

fn resolve(root: TypeContainer, target: &mut ReferenceData,
           pass: &mut ResolveReferenceState) -> Result<()> {
    let mut path_entries: Vec<ReferencePathEntryData> = Vec::new();

    let mut current_type = root.borrow().data.get_result_type();
    let mut current_node = Some(root);

    if target.path.is_some() {
        return Ok(());
    }

    for item in &target.reference.items {
        match resolve_item(item, &mut path_entries, current_node, current_type.follow())? {
            ResolveItemResult::NotAvailible => {
                pass.mark_unfinished();
                return Ok(());
            }
            ResolveItemResult::Ok((spec_next, node_next)) => {
                current_type = spec_next;
                current_node = node_next;
            }
        }
    }

    target.path = Some(path_entries);
    target.target_type_spec = Some(current_type);
    pass.mark_changed();

    validate_causality(target)?;

    Ok(())
}

enum ResolveItemResult {
    NotAvailible,
    Ok((TypeSpecContainer, Option<TypeContainer>))
}

fn resolve_item(item: &ReferenceItem, path_entries: &mut Vec<ReferencePathEntryData>,
                current_node: Option<TypeContainer>, current_type: TypeSpecContainer,
                ) -> Result<ResolveItemResult> {
    let type_next;
    let node_next;

    match *item {
        ReferenceItem::Down(ref name) => {
            path_entries.push(ReferencePathEntryData {
                operation: ReferencePathEntryOperation::Down(name.clone()),
                node: current_node.clone().map(|i| i.downgrade()),
                type_spec: current_type.downgrade(),
            });

            type_next = current_type.borrow().variant
                .get_child_name(name)
                .ok_or_else(|| format!("type has no field '{}'", name.snake()))?
            .follow();

            if let Some(ref current_node_inner_rc) = current_node {
                let node_inner = current_node_inner_rc.borrow();
                node_next = node_inner.variant.to_variant()
                    .resolve_child_name(&node_inner.data, name)
                    .ok().map(|i| i.upgrade());
            } else {
                node_next = None;
            }

            return Ok(ResolveItemResult::Ok((type_next, node_next)));
        },
        ReferenceItem::Property(ref name) => {
            node_next = None;

            if let Some(ref current_node_inner_rc) = current_node {
                let node_inner = current_node_inner_rc.borrow();
                let prop_type_res = node_inner.variant.to_variant()
                    .has_spec_property(&node_inner.data, name);

                if let Some(ref prop_type) = prop_type_res.ok() {
                    path_entries.push(ReferencePathEntryData {
                        operation: ReferencePathEntryOperation::NodeProperty(name.clone()),
                        node: current_node.clone().map(|i| i.downgrade()),
                        type_spec: current_type.downgrade(),
                    });

                    if let Some(ref inner) = *prop_type {
                        type_next = inner.upgrade().follow();
                        return Ok(ResolveItemResult::Ok((type_next, node_next)));
                    } else {
                        return Ok(ResolveItemResult::NotAvailible);
                    }
                }
            }

            let type_inner = current_type.borrow();
            let property = type_inner.variant.has_property(name)?;
            type_next = property.type_spec.clone().follow();

            path_entries.push(ReferencePathEntryData {
                operation: ReferencePathEntryOperation::TypeSpecProperty(property),
                node: current_node.clone().map(|i| i.downgrade()),
                type_spec: current_type.downgrade(),
            });

            return Ok(ResolveItemResult::Ok((type_next, node_next)));
        },
    }

}

fn validate_causality(target: &mut ReferenceData) -> Result<()> {
    Ok(())
}
