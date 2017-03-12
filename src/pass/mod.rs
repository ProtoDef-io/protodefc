use ::TypeContainer;

pub use ::errors::*;

pub trait CompilePass {
    fn run(typ: &mut TypeContainer) -> Result<()>;
}

pub mod assign_parent;
pub mod assign_ident;
pub mod resolve_reference;
