use ::{TypeContainer, TypeData};

pub fn input_for(data: &TypeData) -> String {
    format!("type_input_{}", data.ident.unwrap())
}
pub fn input_for_type(typ: TypeContainer) -> String {
    let typ_inner = typ.borrow();
    input_for(&typ_inner.data)
}
