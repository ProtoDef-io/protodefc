use ::{TypeContainer, TypeData, Type};
use ::variants::{Variant, PrefixedArrayVariant, ReferencedArrayVariant, FixedArrayVariant};
use ::field_reference::FieldReference;
use ::json::JsonValue;

use super::super::errors::*;
use super::super::{FromProtocolJson, type_from_json};
use super::super::count::{CountMode, read_count};

use std::rc::Rc;
use std::cell::RefCell;

pub struct ArrayReader;
impl FromProtocolJson for ArrayReader {
    fn from_json(name: String, arg: &JsonValue) -> Result<TypeContainer> {
        ensure!(arg.is_object(),
                "argument for 'array' must be object, got {:?}",
                arg);
        ensure!(arg.has_key("type"),
                "argument for 'array' must have a 'type' key");

        let child_type = type_from_json(&arg["type"]).chain_err(|| "inside 'type' of 'array'")?;

        match read_count(arg)? {
            CountMode::Prefixed(count_type) => make_prefixed_array(name, child_type, count_type),
            CountMode::Fixed(count) => make_fixed_array(name, child_type, count),
            CountMode::Referenced(reference) => make_referenced_array(name, child_type, reference),
        }
    }
}

fn make_prefixed_array(name: String,
                       child: TypeContainer,
                       prefix_type: TypeContainer)
                       -> Result<TypeContainer> {
    let mut data = TypeData::default();
    data.name = name;
    data.children.push(prefix_type.clone());
    data.children.push(child.clone());

    Ok(Rc::new(RefCell::new(Type {
        variant: Variant::PrefixedArray(PrefixedArrayVariant {
            count: Rc::downgrade(&prefix_type),
            count_index: 0,

            child: Rc::downgrade(&child),
            child_index: 1,
        }),
        data: data,
    })))
}

fn make_referenced_array(name: String,
                         child: TypeContainer,
                         path: FieldReference)
                         -> Result<TypeContainer> {
    let mut data = TypeData::default();
    data.name = name;
    data.children.push(child.clone());

    Ok(Rc::new(RefCell::new(Type {
        variant: Variant::ReferencedArray(ReferencedArrayVariant {
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
        variant: Variant::FixedArray(FixedArrayVariant {
            count: count,
            child: Rc::downgrade(&child),
            child_index: 0,
        }),
        data: data,
    })))
}
