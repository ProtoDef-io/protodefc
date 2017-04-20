use ::errors::*;
use super::{TypeSpecVariant, TypeSpecContainer, IntegerSpec, IntegerSize, BinaryEncoding};
use ::ir::name::Name;

#[derive(Debug, Clone)]
pub enum TypeSpecPropertyVariant {
    ArrayLength,
    BinarySize(BinaryEncoding),
}

#[derive(Debug, Clone)]
pub struct TypeSpecProperty {
    pub variant: TypeSpecPropertyVariant,
    pub type_spec: TypeSpecContainer,
}

impl TypeSpecVariant {

    pub fn has_property(&self, name: &Name) -> Result<TypeSpecProperty> {
        Ok(match (self, name.snake()) {
            (&TypeSpecVariant::Binary(ref spec), "size") => {
                TypeSpecProperty {
                    variant: TypeSpecPropertyVariant::BinarySize(spec.encoding.clone()),
                    type_spec: TypeSpecVariant::Integer(IntegerSpec {
                        size: IntegerSize::IndexSize,
                        signed: false,
                    }).into(),
                }
            }
            (&TypeSpecVariant::Array(_), "length") => {
                TypeSpecProperty {
                    variant: TypeSpecPropertyVariant::ArrayLength,
                    type_spec: TypeSpecVariant::Integer(IntegerSpec {
                        size: IntegerSize::IndexSize,
                        signed: false,
                    }).into(),
                }
            }
            _ => bail!("type '{:?}' has no property {:?}", self, name),
        })
    }

}
