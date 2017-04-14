use ::ir::spec::{TypeContainer, TypeData};

pub fn var_for(name: &str, data: &TypeData) -> String {
    format!("type_{}_{}", data.ident.unwrap(), name)
}
pub fn var_for_type(name: &str, typ: &TypeContainer) -> String {
    let typ_inner = typ.borrow();
    var_for(name, &typ_inner.data)
}

pub fn input_for(data: &TypeData) -> String {
    format!("type_var_{}", data.ident.unwrap())
}
pub fn input_for_type(typ: &TypeContainer) -> String {
    let typ_inner = typ.borrow();
    input_for(&typ_inner.data)
}

pub fn output_for(data: &TypeData) -> String {
    format!("type_var_{}", data.ident.unwrap())
}
pub fn output_for_type(typ: &TypeContainer) -> String {
    let typ_inner = typ.borrow();
    output_for(&typ_inner.data)
}
