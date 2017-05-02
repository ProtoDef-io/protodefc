use ::errors::*;
use ::ir::compilation_unit::{CompilationUnit, CompilationUnitNS,
                             NamedTypeContainer, TypeKind, TypePath, CanonicalNSPath};
use ::ir::spec::{TypeContainer};
use ::ir::spec::variant::Variant;
use ::serde_json::{Value, to_value};

pub fn compile(cu: &CompilationUnit) -> Result<String> {
    Ok(format!("{:#}",
               json!({
        "namespaces": cu.namespaces.iter()
            .map(|ns| ns_to_json(ns))
            .collect::<Result<Vec<Value>>>()?,
    })))
}

fn ns_to_json(ns: &CompilationUnitNS) -> Result<Value> {
    Ok(json!({
        "path": ns_path_to_json(&ns.path)?,
        "types": ns.types.iter()
            .map(|i| ns_type_to_json(i))
            .collect::<Result<Vec<Value>>>()?,
    }))
}

fn ns_type_to_json(typ: &NamedTypeContainer) -> Result<Value> {
    let inner = typ.borrow();

    Ok(json!({
        "name": &inner.path.name,
        "path": ns_path_to_json(&inner.path.path)?,
        "doc": inner.docstring,
        "spec": match inner.typ {
            TypeKind::Native(ref native) => {
                json!({
                    "kind": "native",
                })
            },
            TypeKind::Type(ref spec) => {
                json!({
                    "kind": "spec",
                    "spec": spec_to_json(spec)?,
                })
            },
        },
    }))
}

fn spec_to_json(typ: &TypeContainer) -> Result<Value> {
    let inner = typ.borrow();

    match inner.variant {
        Variant::Container(ref variant) => {
            Ok(json!({
                "kind": "container",
                "fields": variant.fields.iter().map(|field| {
                    Ok(json!({
                        "name": field.name.snake(),
                        "doc": "",
                        "spec": spec_to_json(&field.child.upgrade())?,
                    }))
                }).collect::<Result<Vec<Value>>>()?,
            }))
        }
        Variant::Union(ref variant) => {
            Ok(json!({
                "kind": "union",
                "name": variant.union_name.snake(),
                "cases": variant.cases.iter().map(|case| {
                    Ok(json!({
                        "name": case.case_name.snake(),
                        "doc": "",
                        "spec": spec_to_json(&case.child.upgrade())?,
                    }))
                }).collect::<Result<Vec<Value>>>()?,
                // TODO default case
            }))
        }
        Variant::Array(ref variant) => {
            Ok(json!({
                "kind": "array",
            }))
        }
        Variant::SimpleScalar(ref variant) => {
            Ok(json!({
                "kind": "terminal",
                "path": typ_path_to_json(&variant.path.clone().unwrap())?,
            }))
        }
    }
}

fn typ_path_to_json(typ_path: &TypePath) -> Result<Value> {
    Ok(format!("{}", typ_path).into())
}
fn ns_path_to_json(ns_path: &CanonicalNSPath) -> Result<Value> {
    Ok(format!("{}", ns_path).into())
}
