use ::TypeContainer;
use ::ir::compilation_unit::CompilationUnit;

pub use ::errors::*;

pub mod assign_parent;
pub mod assign_ident;
pub mod resolve_reference;
pub mod resolve_context;
//pub mod propagate_types;
pub mod assign_type_spec;
//pub mod generate_type_spec;
pub mod generate_field_access_order;

// Iterate between making types and resolving references until there
// are no more changes.

pub fn run_passes(cu: &mut CompilationUnit) -> Result<()> {
    // Compilation process:

    // 1. assign_parent
    // Gives all nodes a weak reference to their parent.
    assign_parent::run(cu)?;

    // 2. assign_ident
    // Gives all nodes a unique numeric ident.
    assign_ident::run(cu)?;

    // 3. resolve_context
    // Resolves references from within each type to other defined types
    // within the compilation unit.
    // INVARIANT: At this point all nodes should have direct reference to
    // the container of their child. (Specifically `simple_scalar` nodes).
    resolve_context::run(cu)?;

    assign_type_spec::run(cu)?;

    resolve_reference::run(cu)?;

    generate_field_access_order::run(cu)?;

    // TODO: Validate that all nodes have a valid type_spec
    // TODO: Validate that all nodes have resolved all references

    Ok(())
}

pub fn traverse<F>(typ: &TypeContainer, f: &mut F) -> Result<()>
    where F: FnMut(&TypeContainer) -> Result<()> {

    let children = typ.borrow().data.get_owned_children();

    f(typ)?;

    for child in &children {
        traverse(child, f)?;
    }

    Ok(())
}
