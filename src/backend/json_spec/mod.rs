use ::errors::*;
use ::ir::compilation_unit::{CompilationUnit, CompilationUnitNS,
                             NamedTypeContainer, TypeKind, TypePath, CanonicalNSPath};
use ::ir::spec::{TypeContainer};
use ::ir::spec::variant::Variant;
use ::serde_json::{Value, to_value};

pub fn compilation_unit_to_json_spec(cu: &CompilationUnit) -> Result<String> {
    Ok(format!("{:#}", json!({
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
    Ok(json!({}))
}

fn typ_path_to_json(typ_path: &TypePath) -> Result<Value> {
    Ok(format!("{}", typ_path).into())
}
fn ns_path_to_json(ns_path: &CanonicalNSPath) -> Result<Value> {
    Ok(format!("{}", ns_path).into())
}
