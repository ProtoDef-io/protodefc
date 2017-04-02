use ::ir::TypeContainer;
use super::builder::Block;
use ::backend::imperative_base as ib;
use ::errors::*;

pub fn generate_serialize(typ: TypeContainer) -> Result<Block> {
    let base = ib::serialize::generate_serialize(typ.clone())?;

    let mut ib = Block::new();

    {
        let typ_inner = typ.borrow();
        ib.let_assign(ib::utils::input_for(&typ_inner.data), "input".into());
    }

    ib.scope(super::ib_to_js::build_block(&base)?);
    ib.return_("offset".into());

    let mut b = Block::new();
    b.decl_fun(
        "".into(),
        vec!["input".into(), "buffer".into(), "offset".into()],
        ib
    );
    Ok(b)
}
