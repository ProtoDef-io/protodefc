use ::ir::spec::TypeContainer;
use super::builder::Block;
use ::backend::imperative_base as ib;
use ::errors::*;

pub fn generate_size_of(fun_name: String, typ: TypeContainer) -> Result<Block> {
    let base = ib::size_of::generate_size_of(typ.clone())?;

    let mut ib = Block::new();
    ib.var_assign("count".into(), "0".into());

    {
        let typ_inner = typ.borrow();
        ib.var_assign(ib::utils::input_for(&typ_inner.data), "input".into());
    }

    ib.scope(super::ib_to_js::build_block(&base)?);
    ib.return_("count".into());

    let mut b = Block::new();
    b.decl_fun(fun_name, vec!["input".into()], ib);
    Ok(b)
}
