use ::ir::spec::TypeContainer;
use ::ir::spec::variant::{ContainerFieldType, ContainerVariantBuilder};
use super::FromProtocolJson;
use ::json::JsonValue;
use ::errors::*;
use super::super::type_from_json;
use ::ir::compilation_unit::TypePath;
use ::ir::{FieldPropertyReference, FieldReference};

pub struct ContainerReader;
impl FromProtocolJson for ContainerReader {
    fn from_json(_name: TypePath, arg: &JsonValue) -> Result<TypeContainer> {
        let is_virtual = arg["container_type"] == "virtual";
        let mut builder = ContainerVariantBuilder::new(is_virtual);

        ensure!(arg["fields"].is_array(), "container type must have \"fields\" key");

        for (idx, member) in arg["fields"].members().enumerate() {
            ensure!(member.is_object(), "container field must be object");
            ensure!(member["_kind"] == "container_field",
                    "container field must have \"_kind\": \"container_field\"");

            ensure!(member["name"].is_string(),
                    "container field #{} must have string \"name\" field", idx);
            ensure!(member["type"].is_object(),
                    "container field #{} must have object \"type\" field", idx);

            let is_virtual = member["field_type"] == "virtual";

            let name = &member["name"];
            let final_type = type_from_json(&member["type"])?;

            if is_virtual {
                let typ = ContainerFieldType::Virtual {
                    property: read_prop_reference(&member["value"])?,
                };
                builder.field(name.to_string(), final_type, typ);
            } else {
                builder.normal_field(name.to_string(), final_type);
            }
        }

        builder.build().map_err(|e| e.into())
    }
}

fn read_prop_reference(json: &JsonValue) -> Result<FieldPropertyReference> {
    ensure!(json.is_object(), "field property reference must be object");
    ensure!(json["_kind"] == "property_ref",
            "field property reference must have \"kind\": \"property_ref\"");

    let reference_str = json["ref"].as_str()
        .ok_or("\"ref\" must be string in property_ref")?;
    let property_name = json["prop"].as_str()
        .ok_or("\"prop\" must be string in property_ref")?;

    let reference = FieldReference::parse(reference_str)
        .ok_or("invalid field reference in property_ref")?;
    Ok(FieldPropertyReference {
        property: property_name.to_owned(),
        reference: reference,
        reference_node: None,
    })
}
