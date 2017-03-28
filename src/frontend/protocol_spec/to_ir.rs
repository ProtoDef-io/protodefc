use ::{TypeContainer};
use ::FieldPropertyReference;
use ::ir::variant::{ContainerVariant, ContainerVariantBuilder,
                    SimpleScalarVariant, ContainerFieldType, ArrayVariant};

use super::ast::{Statement, Value, Item, Ident};
use ::errors::*;

pub fn type_def_to_ir(stmt: &Statement) -> Result<TypeContainer> {
    let item = stmt.items[0].item().unwrap();
    if item.name.simple_str() != Some("def_type") {
        unreachable!();
    }
    type_values_to_ir(&stmt.items[1..])
}

fn type_values_to_ir(items: &[Value]) -> Result<TypeContainer> {
    if items.len() == 0 {
        bail!("unexpected end of item chain");
    }
    let item = items[0].item()
        .ok_or("expected type item, got something else")?;

    match item.name {
        Ident::Simple(ref s) => {
            match s.as_str() {
                "container" => ContainerVariant::values_to_ir(items),
                "u8" => SimpleScalarVariant::values_to_ir(items),
                "array" => ArrayVariant::values_to_ir(items),
                _ => unimplemented!(),
            }.chain_err(|| format!("inside '{}' node", s))
        }
        _ => unimplemented!(),
    }
}

trait ValuesToIr {
    fn values_to_ir(items: &[Value]) -> Result<TypeContainer>;
}

impl ValuesToIr for ContainerVariant {
    fn values_to_ir(items: &[Value]) -> Result<TypeContainer> {
        let container_item = items[0].item().unwrap();

        ensure!(container_item.args.len() == 0,
                "container item takes no arguments");

        let mut builder = ContainerVariantBuilder::new(false);

        for stmt in &container_item.block.statements {
            let block_item = stmt.items[0].item()
                .ok_or("container block can only contain items")?;

            match block_item.name.simple_str() {
                Some("field") => {
                    block_item.validate(1, &[], &[])?;

                    let field_name = block_item.get_num(0).unwrap()
                        .string()
                        .ok_or("first argument in 'field' must be a field name")?;

                    let field_type = type_values_to_ir(&stmt.items[1..])
                        .chain_err(|| format!("inside '{}' field", field_name))?;

                    builder.normal_field(
                        field_name.into(),
                        field_type
                    );
                },
                Some("virtual_field") => {
                    block_item.validate(1, &["ref", "prop"], &["ref", "prop"])?;

                    let field_name = block_item.get_num(0).unwrap()
                        .string()
                        .ok_or("first argument in 'virtual_field' must be field name")?;

                    let field_type = type_values_to_ir(&stmt.items[1..])
                        .chain_err(|| format!("inside '{}' virtual_field",
                                              field_name))?;

                    let property = {
                        let reference = block_item.get_tagged("ref").unwrap()
                            .field_reference()
                            .ok_or("'ref' field is not a valid reference")?;
                        let name = block_item.get_tagged("prop").unwrap()
                            .string()
                            .ok_or("'prop' field is not a string")?;
                        FieldPropertyReference {
                            reference: reference,
                            reference_node: None,
                            property: name.into(),
                        }
                    };

                    builder.field(
                        field_name.into(),
                        field_type,
                        ContainerFieldType::Virtual {
                            property: property,
                        }
                    );
                },
                Some("const_field") => {
                    block_item.validate(1, &["ref", "prop"], &[])?;
                    unimplemented!();
                },
                _ => bail!("container block can only contain either 'field', 'virtual_field' or 'const_field'"),
            }

        }

        builder.build().map_err(|e| e.into())
    }
}

impl ValuesToIr for ArrayVariant {
    fn values_to_ir(items: &[Value]) -> Result<TypeContainer> {
        let array_item = items[0].item().unwrap();
        array_item.validate(0, &["ref"], &["ref"])?;

        let reference = array_item.tagged_arg("ref").unwrap()
            .field_reference()
            .ok_or("array does not contain a valid reference")?;

        let field_type = type_values_to_ir(&items[1..])
            .chain_err(|| "inside array".to_owned())?;

        Ok(ArrayVariant::new(reference, field_type))
    }
}

impl ValuesToIr for SimpleScalarVariant {
    fn values_to_ir(items: &[Value]) -> Result<TypeContainer> {
        let scalar_item = items[0].item().unwrap();

        ensure!(scalar_item.is_name_only(),
                "simple scalars takes no arguments and no block");

        match scalar_item.name {
            Ident::Simple(ref string) =>
                Ok(SimpleScalarVariant::new(string.clone())),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::ast::parser::parse;
    use super::type_def_to_ir;

    use ::TypeContainer;
    use ::errors::*;

    macro_rules! unwrap_ok {
        ($e:expr) => {
            match $e {
                Ok(inner) => inner,
                Err(err) => {
                    use error_chain::ChainedError;
                    panic!("Expected Ok, got Err:\n{}", err.display());
                },
            }
        }
    }

    macro_rules! unwrap_error {
        ($e:expr) => {
            match $e {
                Ok(inner) => {
                    panic!("Expected Err, got Ok:\n{:?}", inner);
                },
                Err(inner) => inner,
            }
        }
    }

    fn compile(spec: &str) -> Result<TypeContainer> {
        let ast = parse(spec)?;
        let mut ir = type_def_to_ir(&ast.statements[0])?;
        ::run_passes(&mut ir)?;
        Ok(ir)
    }

    #[test]
    fn simple_spec() {
        let spec = r#"
def_type("test") => container {
    field("test_ffield") => u8;
    virtual_field("something", ref: "test_field", prop: "nonexistent") => u8;
};
"#;
        compile(&spec);
    }

}
