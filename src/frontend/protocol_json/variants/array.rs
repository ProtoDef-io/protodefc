use ::TypeContainer;
use ::variants::{ArrayVariant, ContainerVariantBuilder};
use ::field_reference::FieldReference;
use ::json::JsonValue;

use super::super::errors::*;
use super::super::{FromProtocolJson, type_from_json};
use super::super::count::{CountMode, read_count};

pub struct ArrayReader;
impl FromProtocolJson for ArrayReader {
    fn from_json(name: String, arg: &JsonValue) -> Result<TypeContainer> {
        assert!(name == "array");
        ensure!(arg.is_object(),
                "argument for 'array' must be object, got {:?}",
                arg);
        ensure!(arg.has_key("type"),
                "argument for 'array' must have a 'type' key");

        let child_type = type_from_json(&arg["type"])
            .chain_err(|| "inside 'type' of 'array'")?;

        match read_count(arg)? {
            CountMode::Prefixed(count_type) =>
                make_prefixed_array(child_type, count_type),
            CountMode::Fixed(count) => unimplemented!(),
            //make_fixed_array(name, child_type, count),
            CountMode::Referenced(reference) =>
                make_referenced_array(child_type, reference),
        }
    }
}

/// Since length prefixed arrays are not supported natively
/// in the compiler, we support them by making a virtual container
/// containing both the length field and the array.
///
/// TODO: Since virtual containers are included in path references,
/// this breaks compatibility with node-protodef in places
/// where field references go across this boundary.
fn make_prefixed_array(child: TypeContainer, prefix_type: TypeContainer)
                       -> Result<TypeContainer> {
    let mut virt_container = ContainerVariantBuilder::new(true);

    virt_container.field("length".into(), prefix_type, true);

    let array = ArrayVariant::new(FieldReference::parse("../length").unwrap(), child);
    virt_container.field("data".into(), array, false);

    Ok(virt_container.build().unwrap())
}

fn make_referenced_array(child: TypeContainer, path: FieldReference)
                         -> Result<TypeContainer> {
    Ok(ArrayVariant::new(path, child))
}

//fn make_fixed_array(name: String, child: TypeContainer, count: usize) -> Result<TypeContainer> {
//    let mut data = TypeData::default();
//    data.name = name;
//    data.children.push(child.clone());
//
//    Ok(Rc::new(RefCell::new(Type {
//        variant: Variant::FixedArray(FixedArrayVariant {
//            count: count,
//            child: Rc::downgrade(&child),
//            child_index: 0,
//        }),
//        data: data,
//    })))
//}
