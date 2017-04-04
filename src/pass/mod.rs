use ::TypeContainer;

pub use ::errors::*;

pub mod assign_parent;
pub mod assign_ident;
pub mod resolve_reference;
pub mod resolve_context;
pub mod propagate_types;

pub fn traverse<F>(typ: &TypeContainer, f: &mut F) -> Result<()>
    where F: FnMut(&TypeContainer) -> Result<()> {

    let children;
    {
        let inner = typ.borrow();
        children = inner.data.children.clone();
    }

    f(typ)?;

    for child in &children {
        traverse(child, f)?;
    }

    Ok(())
}
