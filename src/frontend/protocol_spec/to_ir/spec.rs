use ::ir::spec::TypeContainer;
use ::errors::*;
use ::ir::spec::variant::*;
use ::ir::spec::reference::Reference;

use super::super::ast::{Ident, Statement, Value};

pub fn type_def_to_ir(stmt: &Statement) -> Result<TypeContainer> {
    let item = stmt.items[0].item().unwrap();
    if item.name.simple_str() != Some("def") {
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

    // TODO
    match item.name {
        Ident::Simple(ref s) => {
            match s.as_str() {
                "container" => ContainerVariant::values_to_ir(items),
                "array" => ArrayVariant::values_to_ir(items),
                "union" => UnionVariant::values_to_ir(items),
                _ => SimpleScalarVariant::values_to_ir(items),
            }
        }
        _ => SimpleScalarVariant::values_to_ir(items),
    }
}

trait ValuesToIr {
    fn values_to_ir(items: &[Value]) -> Result<TypeContainer>;
}

impl ValuesToIr for ContainerVariant {
    fn values_to_ir(items: &[Value]) -> Result<TypeContainer> {
        let container_item = items[0].item().unwrap();

        container_item.validate(0, &["virtual"], &[])?;

        let is_virtual = container_item.get_tagged("virtual")
            .and_then(|i| i.string())
            .map(|i| i == "true")
            .unwrap_or(false);

        let mut builder = ContainerVariantBuilder::new(is_virtual);

        for stmt in &container_item.block.statements {
            let block_item = stmt.items[0].item()
                .ok_or("container block can only contain items")?;

            match block_item.name.simple_str() {
                Some("field") => {
                    block_item.validate(1, &[], &[])?;

                    let field_name = block_item.get_num(0)
                        .unwrap()
                        .string()
                        .ok_or("first argument in 'field' must be a field name")?;

                    let field_type = type_values_to_ir(&stmt.items[1..])
                        .chain_err(|| format!("inside '{}' field", field_name))?;

                    builder.normal_field(field_name.into(), field_type);
                }
                Some("virtual_field") => {
                    block_item.validate(1, &["value"], &["value"])?;

                    let field_name = block_item.get_num(0)
                        .unwrap()
                        .string()
                        .ok_or("first argument in 'virtual_field' must be field name")?;

                    let field_type = type_values_to_ir(&stmt.items[1..])
                        .chain_err(|| format!("inside '{}' virtual_field",
                                              field_name))?;

                    let reference = {
                        let ref_str = block_item
                            .get_tagged("value").unwrap()
                            .string().ok_or("'value' is not a string")?;
                        Reference::parse(ref_str)?
                    };

                    builder.virtual_field(
                        field_name.into(), field_type, reference);
                }
                Some("const_field") => {
                    block_item.validate(1, &["ref", "prop"], &[])?;
                    unimplemented!();
                }
                _ => {
                    bail!("container block can only contain either 'field', 'virtual_field' or \
                           'const_field'")
                }
            }

        }

        builder.build().map_err(|e| e.into())
    }
}

impl ValuesToIr for ArrayVariant {
    fn values_to_ir(items: &[Value]) -> Result<TypeContainer> {
        let array_item = items[0].item().unwrap();
        array_item.validate(0, &["length"], &["length"])?;

        let reference = array_item
            .tagged_arg("length").unwrap()
            .string().ok_or("length in array must be reference".into())
            .and_then(|string| Reference::parse(string))?;

        let field_type = type_values_to_ir(&items[1..]).chain_err(|| "inside array".to_owned())?;

        Ok(ArrayVariant::new(reference, field_type))
    }
}

impl ValuesToIr for UnionVariant {
    fn values_to_ir(items: &[Value]) -> Result<TypeContainer> {
        let union_item = items[0].item().unwrap();
        union_item.validate(1, &["tag"], &["tag"])?;

        let union_name = union_item
            .arg(0).unwrap()
            .string().ok_or("union name must be a string")?;
        let tag_ref = union_item
            .tagged_arg("tag").unwrap()
            .reference()?;

        let mut builder = UnionVariantBuilder::new(union_name.into(), tag_ref);

        for stmt in &union_item.block.statements {
            let block_item = stmt.items[0].item()
                .ok_or("container block can only contain items")?;

            match block_item.name.simple_str() {
                Some("variant") => {
                    block_item.validate(1, &["match"], &["match"])?;

                    let variant_name = block_item.arg(0)
                        .unwrap()
                        .string()
                        .ok_or("variant name arg must be string")?;
                    let variant_match = block_item.tagged_arg("match")
                        .unwrap()
                        .string()
                        .ok_or("variant match arg must be string")?;

                    let field_type = type_values_to_ir(&stmt.items[1..])?;

                    builder.case(variant_match.into(), variant_name.into(), field_type);
                }
                _ => bail!("union block can only contain 'variant'"),
            }
        }

        builder.build().map_err(|e| e.into())
    }
}

impl ValuesToIr for SimpleScalarVariant {
    fn values_to_ir(items: &[Value]) -> Result<TypeContainer> {
        let scalar_item = items[0].item().unwrap();

        //ensure!(scalar_item.is_name_only(),
        //        "simple scalars takes no arguments and no block");

        let mut all_tagged = scalar_item.all_tagged()?;
        let arguments = all_tagged.drain()
            .map(|(name, value)| {
                value.reference().map(|reference| (name, reference))
            })
            .collect::<Result<Vec<(String, Reference)>>>()?;

        let path = scalar_item.name.to_type_path();
        Ok(SimpleScalarVariant::new(path, arguments))
    }
}
