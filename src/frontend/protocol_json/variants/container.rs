use ::ir::spec::TypeContainer;
use ::ir::spec::variant::ContainerVariantBuilder;
use super::FromProtocolJson;
use ::json::JsonValue;
use ::errors::*;
use super::super::type_from_json;
use ::ir::compilation_unit::TypePath;
use ::ir::spec::reference::Reference;

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
                let reference = read_prop_reference(&member["value"])?;
                builder.virtual_field(name.to_string(), final_type, reference);
            } else {
                builder.normal_field(name.to_string(), final_type);
            }
        }

        builder.build().map_err(|e| e.into())
    }
}

fn read_prop_reference(json: &JsonValue) -> Result<Reference> {
    ensure!(json.is_string(), "field property reference must be string");
    Reference::parse(json.as_str().unwrap())
}
