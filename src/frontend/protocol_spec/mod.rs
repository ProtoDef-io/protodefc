pub mod ast;

pub mod from_ir;
pub mod to_ir;

pub use self::ast::parser::parse;
pub use self::to_ir::type_def_to_ir;

use ::context::compilation_unit::{CompilationUnit, CompilationUnitNS, NamedType,
                                  TypePath, NSPath};
use ::context::id_generator::IdGenerator;
use ::errors::*;

pub fn to_compilation_unit(input: &str) -> Result<CompilationUnit> {
    let ast = ast::parser::parse(input)?;

    let mut id_gen = ::context::id_generator::IdGenerator::new();
    let mut path = Vec::<String>::new();
    let mut cu = CompilationUnit::new();

    block_to_compilation_unit_ns(&ast, &mut cu, &mut id_gen, &mut path)?;

    Ok(cu)
}

fn block_to_compilation_unit_ns(block: &ast::Block, cu: &mut CompilationUnit,
                                gen: &mut IdGenerator, path: &mut Vec<String>)
                                -> Result<()> {
    let ns_path = NSPath::with_path(path.clone());
    let mut ns = CompilationUnitNS::new(ns_path.clone());

    for stmt in &block.statements {
        let head_item = stmt.items[0].item()
            .ok_or("statement in root must start with item")?;
        let head_item_name = head_item.name.simple_str()
            .ok_or("statement in root must start with non-namespaced item")?;

        match head_item_name.as_ref() {
            "def_type" => {
                head_item.validate(1, &[], &[])?;
                let name = head_item.arg(0).unwrap().string()
                    .ok_or("argument to def_type must be string")?;

                let typ = to_ir::type_def_to_ir(stmt)?;
                ns.add_type(NamedType {
                    path: TypePath {
                        path: ns_path.clone(),
                        name: name.to_owned(),
                    },
                    typ: typ,
                    type_id: gen.get(),
                })?;
            },
            "namespace" => {
                head_item.validate(1, &[], &[])?;
                let name = head_item.arg(0).unwrap().string()
                    .ok_or("argument to namespace must be string")?;

                if stmt.items.len() != 1 {
                    bail!("namespace statement cannot have any children");
                }

                path.push(name.to_owned());
                block_to_compilation_unit_ns(&head_item.block, cu, gen, path)?;
                path.pop();
            },
            name => bail!("'{}' item not allowed in root", name),
        }
    }

    cu.add_namespace(ns)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::to_compilation_unit;

    #[test]
    fn spec_to_compilation_unit() {
        let result = to_compilation_unit(r#"
def_type("root_type") => u8;
namespace("some_namespace") {
    def_type("inner_type") => u8;
    namespace("inner_namespace") {
        def_type("deep_type") => u8;
    };
};
"#).unwrap();
        println!("{:?}", result);
    }

}
