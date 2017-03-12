use ::{Type, TypeData, TypeContainer};
use ::variants::{SimpleScalarVariant, ContainerVariant, ContainerField, ReferencedArrayVariant,
                 PrefixedArrayVariant, FixedArrayVariant, SwitchVariant, SwitchCase};
use ::field_reference::FieldReference;
use super::FromProtocolJson;
use super::json::JsonValue;
use super::errors::*;
use super::type_from_json;

use std::rc::Rc;
use std::cell::RefCell;

pub struct ScalarReader;
impl FromProtocolJson for ScalarReader {
    fn from_json(name: String, _arg: &JsonValue) -> Result<TypeContainer> {
        let mut data = TypeData::default();
        data.name = name;

        Ok(Rc::new(RefCell::new(Type {
            data: data,
            variant: Box::new(SimpleScalarVariant {}),
        })))
    }
}

pub struct ContainerReader;
impl FromProtocolJson for ContainerReader {
    fn from_json(name: String, arg: &JsonValue) -> Result<TypeContainer> {
        ensure!(arg.is_array(),
                "argument for 'container' must be array, got {:?}",
                arg);

        let mut children_fields: Vec<(TypeContainer, ContainerField)> = arg.members()
            .enumerate()
            .map(|(idx, member)| {
                ensure!(member.is_object(),
                        "'container' child must be object, got {:?}",
                        member);
                ensure!(member.has_key("name"),
                        "'container' child #{} missing 'name' field",
                        idx);
                ensure!(member.has_key("type"),
                        "'container' child #{} missing 'type' field",
                        idx);

                let typ = &member["type"];
                let name = &member["name"];

                let final_type = type_from_json(typ)?;
                let field = ContainerField {
                    name: name.to_string(),
                    child: Rc::downgrade(&final_type),
                    child_index: idx,
                };

                Ok((final_type, field))
            })
            .collect::<Result<Vec<(TypeContainer, ContainerField)>>>()?;

        let (children, fields): (Vec<TypeContainer>, Vec<ContainerField>) =
            children_fields.drain(0..).unzip();

        let mut data = TypeData::default();
        data.name = name;
        data.children = children;

        Ok(Rc::new(RefCell::new(Type {
            data: data,
            variant: Box::new(ContainerVariant { fields: fields }),
        })))
    }
}

pub struct ArrayReader;
impl FromProtocolJson for ArrayReader {
    fn from_json(name: String, arg: &JsonValue) -> Result<TypeContainer> {
        ensure!(arg.is_object(),
                "argument for 'array' must be object, got {:?}",
                arg);
        ensure!(arg.has_key("type"),
                "argument for 'array' must have a 'type' key");

        let has_count = arg.has_key("count");
        let has_count_type = arg.has_key("countType");

        ensure!(!(has_count && has_count_type),
                "argument for 'array' must have only one of 'count' and 'countType'");

        let child_type = type_from_json(arg).chain_err(|| "inside 'type' of 'array'")?;

        if has_count_type {
            make_prefixed_array(name, child_type, &arg["countType"])
        } else {
            let count_arg = &arg["count"];
            if let Some(count) = count_arg.as_u64() {
                make_fixed_array(name, child_type, count as usize)
            } else if let Some(path) = count_arg.as_str() {
                make_referenced_array(name, child_type, path)
            } else {
                bail!("'count' argument for 'array' must be either a path or a number");
            }
        }
    }
}

fn make_prefixed_array(name: String,
                       child: TypeContainer,
                       prefix_type_raw: &JsonValue)
                       -> Result<TypeContainer> {
    let prefix_type = type_from_json(prefix_type_raw).chain_err(|| "inside 'countType of 'array'")?;

    let mut data = TypeData::default();
    data.name = name;
    data.children.push(prefix_type.clone());
    data.children.push(child.clone());

    Ok(Rc::new(RefCell::new(Type {
        variant: Box::new(PrefixedArrayVariant {
            count: Rc::downgrade(&prefix_type),
            count_index: 0,

            child: Rc::downgrade(&child),
            child_index: 1,
        }),
        data: data,
    })))
}

fn make_referenced_array(name: String, child: TypeContainer, path: &str) -> Result<TypeContainer> {
    let path = FieldReference::parse(path)
        .ok_or("'count' field in 'array must contain a valid field reference")?;

    let mut data = TypeData::default();
    data.name = name;
    data.children.push(child.clone());

    Ok(Rc::new(RefCell::new(Type {
        variant: Box::new(ReferencedArrayVariant {
            count: None,
            count_path: path,

            child: Rc::downgrade(&child),
            child_index: 0,
        }),
        data: data,
    })))
}

fn make_fixed_array(name: String, child: TypeContainer, count: usize) -> Result<TypeContainer> {
    let mut data = TypeData::default();
    data.name = name;
    data.children.push(child.clone());

    Ok(Rc::new(RefCell::new(Type {
        variant: Box::new(FixedArrayVariant  {
            count: count,
            child: Rc::downgrade(&child),
            child_index: 0,
        }),
        data: data,
    })))
}

pub struct SwitchReader;
impl FromProtocolJson for SwitchReader {
    fn from_json(name: String, arg: &JsonValue) -> Result<TypeContainer> {
        ensure!(arg.is_object(),
                "argument for 'switch' must be object, got {:?}",
                arg);
        ensure!(arg.has_key("compareTo"),
                "argument for 'switch' must have 'compareTo' key");
        ensure!(arg.has_key("fields"),
                "argument for 'switch' must have 'fields' key");
        ensure!(arg["fields"].is_object(),
                "'fields' field in 'switch' must be object");

        let compareToStr = arg["compareTo"].as_str()
            .ok_or("'compareTo' field in 'switch' must be string")?;
        let compareTo = FieldReference::parse(compareToStr)
            .ok_or("'compareTo' field in 'switch' must contain a valid field reference")?;

        let fields_key = &arg["fields"];

        let mut children_options: Vec<(TypeContainer, SwitchCase)> =
            fields_key.entries()
            .enumerate()
            .map(|(idx, (key, typ))| {
                let final_type = type_from_json(typ)?;
                let field = SwitchCase {
                    raw_value: key.to_string(),
                    child: Rc::downgrade(&final_type),
                    child_index: idx,
                };

                Ok((final_type, field))
            })
            .collect::<Result<Vec<(TypeContainer, SwitchCase)>>>()?;

        let (mut children, fields): (Vec<TypeContainer>, Vec<SwitchCase>) =
            children_options.drain(0..).unzip();

        if arg.has_key("default") {
            let typ = type_from_json(&arg["default"])?;
            let idx = children.len();
            children.push(typ);
        }

        unreachable!();
    }
}
