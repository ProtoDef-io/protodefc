use ::{TypeVariant, TypeData, Type, WeakTypeContainer, Result, TypeContainer};
use ::field_reference::FieldReference;

#[derive(Debug)]
pub struct UnionVariant {
    union_name: String,

    match_field_ref: FieldReference,
    match_field: Option<WeakTypeContainer>,

    cases: Vec<UnionCase>,
}

#[derive(Debug)]
pub struct UnionCase {
    match_val_str: String,
    case_name: String,
    child: Option<WeakTypeContainer>,
}

impl TypeVariant for UnionVariant {
    default_resolve_child_name_impl!();
    default_has_property_impl!();

    fn do_resolve_references(&mut self, data: &mut TypeData,
                             resolver: &::ReferenceResolver) -> Result<()> {

        self.match_field = Some(resolver(self, data, &self.match_field_ref)?);

        Ok(())
    }
}
