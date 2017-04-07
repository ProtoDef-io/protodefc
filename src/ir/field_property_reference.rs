use ::ir::FieldReference;
use ::ir::typ::WeakTypeContainer;

#[derive(Debug)]
/// Used to reference a specific property of another field.
pub struct FieldPropertyReference {
    pub reference: FieldReference,
    pub reference_node: Option<WeakTypeContainer>,
    pub property: String,
}

impl FieldPropertyReference {

}
