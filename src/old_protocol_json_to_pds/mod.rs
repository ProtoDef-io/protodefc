use ::errors::*;

mod namespace;
mod typ;

pub fn convert(json: &str) -> Result<String> {
    let value = ::json::parse(json)?;
    let res = namespace::namespace_to_pds(&value)?;
    Ok(::frontend::protocol_spec::ast::printer::print(&res))
}
