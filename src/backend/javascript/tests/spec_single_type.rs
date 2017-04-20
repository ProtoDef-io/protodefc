use ::spec_to_final_compilation_unit;
use ::backend::javascript::cu_to_js::generate_compilation_unit;
use super::super::builder::ToJavascript;
use ::itertools::Itertools;

fn test_single(spec: &str, data: &str, bin_data: &[u8]) {
    let cu = spec_to_final_compilation_unit(spec).unwrap();

    let bin_data_arr: String = bin_data.iter()
        .map(|val| format!("{}", val))
        .join(",");

    let block = generate_compilation_unit(&cu).unwrap();
    let mut out = String::new();
    block.to_javascript(&mut out, 0);

    println!("{}", out);
    let compare = format!(
        r#"
let buffer = require("buffer");
let ref_js_data = {};
let ref_length = {};
let ref_buf = buffer.Buffer.from([{}]);

// size_of
assert.deepEqual(exports["test"]["size_of"](ref_js_data), ref_length);

// serialize
let buf = buffer.Buffer.alloc(ref_length, 0);
exports["test"]["serialize"](ref_js_data, buf, 0);
assert(buf.equals(ref_buf));

// deserialize
let ret = exports["test"]["deserialize"](ref_buf, 0);
assert.deepEqual(ret, [ref_js_data, ref_length]);
"#,
        data, bin_data.len(), bin_data_arr
    );

    super::test_with_data_eq(&out, &compare);

}

#[test]
fn simple_scalar() {
    test_single(
        r#"
@type integer("u8")
def_native("u8");

@export "test"
def("test") => u8;
"#,
        "0",
        &[0]
    );
}

#[test]
fn container() {
    test_single(
        r#"
@type integer("u8")
def_native("u8");

@export "test"
def("test") => container {
    field("foo") => u8;
    field("bar") => u8;
};
"#,
        "{foo: 0, bar: 0}",
        &[0, 0],
    );
}

#[test]
fn array() {
    test_single(
        r#"
@type integer("u8")
def_native("u8");

@export "test"
def("test") => container(virtual: "true") {
    virtual_field("len", value: "arr/@length") => u8;
    field("arr") => array(length: "../len") => u8;
};
"#,
        "[1, 2, 3]",
        &[3, 1, 2, 3]
    );
}

#[test]
fn union() {
    let spec = r#"
@type integer("u8")
def_native("u8");

@export "test"
def("test") => container(virtual: "true") {
    virtual_field("tag", value: "data/@tag") => u8;
    field("data") => union("test_union", tag: "../tag") {
        variant("zero", match: "0") => u8;
        variant("one", match: "1") => container {
            field("woo") => u8;
            field("hoo") => u8;
        };
    };
};
"#;

    test_single(
        spec,
        "{tag: \"zero\", data: 0}",
        &[0, 0],
    );
    test_single(
        spec,
        "{tag: \"one\", data: {woo: 0, hoo: 1}}",
        &[1, 0, 1],
    );
}

#[test]
fn union_default() {
    let spec = r#"
@type integer("u8")
def_native("u8");

@export "test"
def("test") => container {
    field("tag") => u8;
    field("data") => union("test_union", tag: "../tag") {
        variant("zero", match: "0") => u8;
        default("default") => container {
            field("woo") => u8;
            field("hoo") => u8;
        };
    };
};
"#;

    test_single(
        spec,
        "{tag: 0, data: {tag: \"zero\", data: 8}}",
        &[0, 8]
    );
    test_single(
        spec,
        "{tag: 8, data: {tag: \"default\", data: {woo: 2, hoo: 5}}}",
        &[8, 2, 5]
    );
}

#[test]
fn native_type_argument() {
    let spec = r#"
@type binary("utf8")
def_native("sized_string") {
    argument("size", stage: "read") => integer("usize");
};

@type integer("u8")
def_native("u8");

@export "test"
def("test") => container(virtual: "true") {
    virtual_field("size", value: "string/@size") => u8;
    field("string") => sized_string(size: "../size");
};
"#;

    test_single(
        spec,
        "\"foo\"",
        &[3, b'f', b'o', b'o']
    );
}

//#[test]
//fn protodef_spec_tests() {
//    for case in ::test_harness::json_spec_cases() {
//        println!("Testing {}", case.name);
//
//        let ast = ::json_to_final_ast(&::json::stringify(case.json_type)).unwrap();
//        let size_of = generate_size_of(ast).unwrap();
//
//        let mut out = String::new();
//        size_of.to_javascript(&mut out, 0);
//
//        //for value in case.values {
//        //    super::test_with_data_eq(
//        //        &out,
//        //        &value.json_value,
//        //        &format!("{}", value.serialized.len()),
//        //    );
//        //}
//    }
//}
