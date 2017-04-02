use ::json_to_final_ast;
use ::spec_type_to_final_ast;
use ::backend::javascript::size_of::generate_size_of;
use super::super::builder::ToJavascript;

fn test_size_of(spec: &str, data: &str, result: &str) {
    let ir = spec_type_to_final_ast(spec).unwrap();
    let size_of = generate_size_of(ir).unwrap();

    let mut out = String::new();
    size_of.to_javascript(&mut out, 0);

    println!("{}", out);
    super::test_with_data_eq(&out, data, result);
}

#[test]
fn simple_scalar() {
    test_size_of(
        r#"
def_type("test") => u8;
"#,
        "0",
        "1"
    );
}

#[test]
fn container() {
    test_size_of(
        r#"
def_type("test") => container {
    field("foo") => u8;
    field("bar") => u8;
};
"#,
        "{foo: 0, bar: 0}",
        "2"
    );
}

#[test]
fn array() {
    test_size_of(
        r#"
def_type("test") => container(virtual: "true") {
    virtual_field("len", ref: "arr", prop: "length") => u8;
    field("arr") => array(ref: "../len") => u8;
};
"#,
        "[1, 2, 3]",
        "4"
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

    test_size_of(
        spec,
        "{tag: \"zero\", data: 0}",
        "2",
    );
    test_size_of(
        spec,
        "{tag: \"one\", data: {woo: 0, hoo: 1}}",
        "3"
    );
}

//#[test]
fn protodef_spec_tests() {
    for case in ::test_harness::json_spec_cases() {
        println!("Testing {}", case.name);

        let ast = ::json_to_final_ast(&::json::stringify(case.json_type)).unwrap();
        let size_of = generate_size_of(ast).unwrap();

        let mut out = String::new();
        size_of.to_javascript(&mut out, 0);

        for value in case.values {
            super::test_with_data_eq(
                &out,
                &value.json_value,
                &format!("{}", value.serialized.len()));
        }
    }
}
