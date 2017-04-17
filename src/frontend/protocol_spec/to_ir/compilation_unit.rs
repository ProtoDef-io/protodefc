use ::errors::*;
use ::ir::IdGenerator;
use ::ir::compilation_unit::{CompilationUnit, NSPath, CompilationUnitNS, NamedType,
                             TypePath, TypeKind, NativeType};
use ::ir::type_spec::{TypeSpecVariant, IntegerSpec, IntegerSize, Signedness};

use super::super::ast;
use super::spec;
use super::type_spec;

pub fn to_compilation_unit(input: &str) -> Result<CompilationUnit> {
    let ast = ast::parser::parse(input)?;

    let mut id_gen = IdGenerator::new();
    let mut path = Vec::<String>::new();
    let mut cu = CompilationUnit::new();

    block_to_compilation_unit_ns(&ast, &mut cu, &mut id_gen, &mut path)?;

    Ok(cu)
}

fn block_to_compilation_unit_ns(block: &ast::Block,
                                cu: &mut CompilationUnit,
                                gen: &mut IdGenerator,
                                path: &mut Vec<String>)
                                -> Result<()> {
    let ns_path = NSPath::with_path(path.clone());
    let mut ns = CompilationUnitNS::new(ns_path.clone());

    for stmt in &block.statements {
        let head_item = stmt.items[0].item()
            .ok_or("statement in root must start with item")?;
        let head_item_name = head_item.name
            .simple_str()
            .ok_or("statement in root must start with non-namespaced item")?;

        match head_item_name.as_ref() {
            "def" => {
                head_item.validate(1, &[], &[])?;
                let name = head_item.arg(0)
                    .unwrap()
                    .string()
                    .ok_or("argument to def must be string")?;

                let typ = spec::type_def_to_ir(stmt)
                    .chain_err(|| format!("inside def(\"{}\")", name))?;

                ns.add_type(NamedType {
                    path: TypePath {
                        path: ns_path.clone(),
                        name: name.to_owned(),
                    },
                    typ: TypeKind::Type(typ),
                    type_id: gen.get(),
                    type_spec: TypeSpecVariant::Referenced(None).into(),
                    arguments: vec![],
                })?;
            }
            "def_native" => {
                head_item.validate(1, &[], &[])?;
                let name = head_item.arg(0)
                    .unwrap()
                    .string()
                    .ok_or("argument to def_native must be string")?;

                if stmt.items.len() != 1 {
                    bail!("def_native statement cannot have any children");
                }

                let target_type = if let Some(ref values) = stmt.attributes.get("type") {
                    type_spec::items_to_type_spec(values)?
                } else {
                    TypeSpecVariant::Opaque.into()
                };

                ns.add_type(NamedType {
                    path: TypePath {
                        path: ns_path.clone(),
                        name: name.to_owned(),
                    },
                    typ: TypeKind::Native(NativeType {
                        type_spec: target_type,
                    }),
                    type_id: gen.get(),
                    type_spec: TypeSpecVariant::Referenced(None).into(),
                    arguments: super::native_type::block_to_args(&head_item.block)?,
                })?;
            }
            "namespace" => {
                head_item.validate(1, &[], &[])?;
                let name = head_item.arg(0)
                    .unwrap()
                    .string()
                    .ok_or("argument to namespace must be string")?;

                if stmt.items.len() != 1 {
                    bail!("namespace statement cannot have any children");
                }

                path.push(name.to_owned());
                block_to_compilation_unit_ns(&head_item.block, cu, gen, path)
                    .chain_err(|| format!("inside namespace '{}'", name))?;
                path.pop();
            }
            name => bail!("'{}' item not allowed in root", name),
        }
    }

    cu.add_namespace(ns)?;
    Ok(())
}
