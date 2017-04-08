use ::ir::spec::TypeContainer;
use super::builder::Block;
use ::backend::imperative_base as ib;
use ::errors::*;

pub fn generate_deserialize(fun_name: String, typ: TypeContainer) -> Result<Block> {
    let base = ib::deserialize::generate_deserialize(typ.clone())?;

    let mut ib = Block::new();

    ib.scope(super::ib_to_js::build_block(&base)?);
    ib.return_(format!("[{}, offset]",
                       ib::utils::output_for_type(typ.clone())).into());

    let mut b = Block::new();
    b.decl_fun(
        fun_name,
        vec!["buffer".into(), "offset".into()],
        ib
    );
    Ok(b)
}
