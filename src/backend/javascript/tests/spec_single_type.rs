use ::spec_type_to_final_ast;
use ::backend::javascript::size_of::generate_size_of;
use ::backend::javascript::serialize::generate_serialize;
use ::backend::javascript::deserialize::generate_deserialize;
use super::super::builder::ToJavascript;
use ::itertools::Itertools;

fn test_single(spec: &str, data: &str, bin_data: &[u8]) {
    let ir = spec_type_to_final_ast(spec).unwrap();

    let bin_data_arr: String = bin_data.iter()
        .map(|val| format!("{}", val))
        .join(",");

    {
        let size_of = generate_size_of(ir.clone()).unwrap();

        let mut out = String::new();
        size_of.to_javascript(&mut out, 0);

        println!("{}", out);

        let compare = format!("assert.deepEqual(test_fun({}), {});",
                              data, bin_data.len());
        super::test_with_data_eq(&out, &compare);
    }

    {
        let serialize = generate_serialize(ir.clone()).unwrap();

        let mut out = String::new();
        serialize.to_javascript(&mut out, 0);

        println!("{}", out);

        let compare = format!(
            r#"
var buffer = require("buffer");
let buf = buffer.Buffer.alloc({}, 0);
test_fun({}, buf, 0);
console.log(buf);
assert(buf.equals(buffer.Buffer.from([{}])));
"#,
            bin_data.len(), data, bin_data_arr
        );

        super::test_with_data_eq(&out, &compare);
    }

    {
        let deserialize = generate_deserialize(ir.clone()).unwrap();

        let mut out = String::new();
        deserialize.to_javascript(&mut out, 0);

        println!("{}", out);

        let compare = format!(
        r#"
var buffer = require("buffer");
let buf = buffer.Buffer.from([{}]);
let ret = test_fun(buf, 0);
console.log(ret);
assert.deepEqual(ret, [{}, {}]);
"#,
            bin_data_arr, data, bin_data.len()
        );

        super::test_with_data_eq(&out, &compare);
    }
}

#[test]
fn simple_scalar() {
    test_single(
        r#"
def_type("test") => u8;
"#,
        "0",
        &[0]
    );
}

#[test]
fn container() {
    test_single(
        r#"
def_type("test") => container {
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
def_type("test") => container(virtual: "true") {
    virtual_field("len", ref: "arr", prop: "length") => u8;
    field("arr") => array(ref: "../len") => u8;
};
"#,
        "[1, 2, 3]",
        &[3, 1, 2, 3]
    );
}

#[test]
fn union() {
    let spec = r#"
def_type("test") => container(virtual: "true") {
    virtual_field("tag", ref: "data", prop: "tag") => u8;
    field("data") => union("test_union", ref: "../tag") {
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
