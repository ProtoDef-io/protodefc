
use ::json_to_final_ast;
use super::generate_size_of;
use super::super::builder::ToJavascript;

#[test]
fn simple_scalar() {
    let ast = json_to_final_ast("[\"i8\", null]").unwrap();
    let size_of = generate_size_of(ast).unwrap();

    let mut out = String::new();
    size_of.to_javascript(&mut out, 0)
}

#[test]
fn container() {
    let ast = json_to_final_ast(r#"
["container", [
    {"name": "foo", "type": "i8"},
    {"name": "bar", "type": "i8"}
]]"#).unwrap();
    let size_of = generate_size_of(ast).unwrap();

    let mut out = String::new();
    size_of.to_javascript(&mut out, 0);

    println!("{}", out);
    super::super::test_harness::test_with_data_eq(&out, "{foo: 0, bar: 0}", "2");
}

//#[test]
fn protodef_spec_tests() {
    for case in ::test_harness::cases() {
        println!("Testing {}", case.name);

        let ast = ::json_to_final_ast(&::json::stringify(case.json_type)).unwrap();
        let size_of = generate_size_of(ast).unwrap();

        let mut out = String::new();
        size_of.to_javascript(&mut out, 0);

        for value in case.values {
            super::super::test_harness::test_with_data_eq(
                &out,
                &value.json_value,
                &format!("{}", value.serialized.len()));
        }
    }
}
