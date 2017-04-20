use ::num_bigint as bigint;
use ::errors::*;
use ::ir::name::Name;
use super::{TypeSpecContainer, TypeSpecVariant, IntegerSpec, IntegerSize, BinarySpec};

#[derive(Debug)]
pub struct TypeSpecLiteral {
    pub type_spec: TypeSpecContainer,
    pub variant: TypeSpecLiteralVariant,
}

#[derive(Debug)]
pub enum TypeSpecLiteralVariant {
    Binary {
        data: Vec<u8>,
    },
    Integer {
        data: bigint::BigInt,
    },
    EnumTag {
        name: Name,
    },
    Boolean {
        data: bool,
    },
}

impl TypeSpecLiteral {

    pub fn parse(type_spec: &TypeSpecContainer, input: &str) -> Result<TypeSpecLiteral> {
        let type_spec_inner = type_spec.borrow();

        let res = match type_spec_inner.variant {
            TypeSpecVariant::Integer(ref data) =>
                parse_integer(data, input)?,
            TypeSpecVariant::Boolean =>
                parse_boolean(input)?,
            TypeSpecVariant::Binary(ref data) =>
                parse_binary(data, input)?,
            _ => bail!("type does not support literals"),
        };

        Ok(TypeSpecLiteral {
            type_spec: type_spec.clone(),
            variant: res,
        })
    }

}

enum ParseBinaryState {
    Byte,
    EscapeStart,

    HexStart,
    HexOne(bool),
    HexTwo(u8, bool),
}

fn parse_binary(data: &BinarySpec, input_str: &str) -> Result<TypeSpecLiteralVariant> {
    let input = input_str.bytes();

    let mut out = Vec::with_capacity(input.len());
    let mut state = ParseBinaryState::Byte;

    for byte in input {
        state = match (state, byte) {
            // General escapes
            (ParseBinaryState::Byte, b'\\') => ParseBinaryState::EscapeStart,
            (ParseBinaryState::EscapeStart, b'\\') => {
                out.push(b'\\');
                ParseBinaryState::Byte
            }
            (ParseBinaryState::EscapeStart, b'n') => {
                out.push(b'\n');
                ParseBinaryState::Byte
            }

            // Hex escapes
            (ParseBinaryState::EscapeStart, b'x') => ParseBinaryState::HexStart,
            //(ParseBinaryState::HexOne(block), _) =>

            // Catch all
            (ParseBinaryState::Byte, _) => {
                out.push(byte);
                ParseBinaryState::Byte
            }
            (ParseBinaryState::EscapeStart, _) => {
                bail!("invalid escape character");
            }
            _ => unreachable!(),
        };
    }

    Ok(TypeSpecLiteralVariant::Binary {
        data: out,
    })
}

fn parse_boolean(input: &str) -> Result<TypeSpecLiteralVariant> {
    if input == "true" {
        Ok(TypeSpecLiteralVariant::Boolean { data: true })
    } else if input == "false" {
        Ok(TypeSpecLiteralVariant::Boolean { data: false })
    } else {
        bail!("boolean literal must be either 'true' or 'false', got {:?}",
              input);
    }
}

fn parse_integer(data: &IntegerSpec, input: &str)
                     -> Result<TypeSpecLiteralVariant> {
    let bytes = input.as_bytes();

    let big_int = match () {
        _ if bytes.starts_with(b"0x") =>
            bigint::BigInt::parse_bytes(&bytes[2..], 16),
        _ if bytes.starts_with(b"0b") =>
            bigint::BigInt::parse_bytes(&bytes[2..], 2),
        _ =>
            bigint::BigInt::parse_bytes(bytes, 10),
    }.ok_or_else(|| format!("could not parse integer literal: {:?}",
                            input))?;

    if !data.signed {
        let big_int_signed = big_int.sign() == bigint::Sign::Minus;
        ensure!(!big_int_signed, "target data type is not signed");
    }

    if let IntegerSize::AtLeast(size) = data.size {
        ensure!(big_int.bits() <= size,
                "target data type is not big enough to contain this number");
    }

    Ok(TypeSpecLiteralVariant::Integer {
        data: big_int,
    })
}
