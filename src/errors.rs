use ::ir::variant::VariantType;
use ::FieldReference;

error_chain! {
    links {
        JsonParseError(
            ::frontend::protocol_json::Error,
            ::frontend::protocol_json::ErrorKind);
    }

    errors {
        CompilerError(t: CompilerError) {
            description("error under compilation")
                display("{}", t.display())
        }
    }
}

#[derive(Debug, Clone)]
pub enum CompilerError {

    /// The given variant does not have this property.
    NoProperty { variant: VariantType, property: String, },

    /// Attempted to resolve a nonexistent field on a variant.
    ChildResolveError { parent_variant: String, name: String, },

    /// Attempted to match on a type which does not support it.
    UnmatchableType { variant: VariantType, },

    /// Error while resolving a reference.
    ReferenceError { reference: FieldReference, },

    /// Error occurred while inside a variant.
    InsideVariant { variant: VariantType, },
    /// Error occurred while inside a named field.
    InsideNamed { name: String, },

    /// An error occurred in a nom parser.
    NomParseError(::nom::verbose_errors::Err<usize>),

}

impl CompilerError {

    pub fn display(&self) -> String {
        match *self {
            CompilerError::NoProperty { ref variant, ref property } =>
                format!("'{:?}' variant has no property '{}'",
                        variant, property),
            CompilerError::ChildResolveError { ref parent_variant, ref name } =>
                format!("'{}' variant has no child with name '{}'",
                        parent_variant, name),
            CompilerError::UnmatchableType { ref variant } =>
                format!("'{:?}' does not support matching",
                        variant),
            CompilerError::ReferenceError { ref reference } =>
                format!("unable to resolve reference '{:?}'",
                        reference),
            CompilerError::InsideVariant { ref variant } =>
                format!("inside variant '{:?}'",
                        variant),
            CompilerError::InsideNamed { ref name } =>
                format!("inside named '{:?}'",
                        name),
            CompilerError::NomParseError(_) =>
                format!("nom parse errror"),
        }
    }

}

impl From<CompilerError> for Error {
    fn from(typ: CompilerError) -> Error {
        ErrorKind::CompilerError(typ).into()
    }
}
impl From<CompilerError> for ErrorKind {
    fn from(typ: CompilerError) -> ErrorKind {
        ErrorKind::CompilerError(typ).into()
    }
}
