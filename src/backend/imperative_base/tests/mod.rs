use ::spec_type_to_final_ast;
use super::size_of::generate_size_of;
use super::serialize::generate_serialize;

fn test_compile(spec: &str) {
    let ir = spec_type_to_final_ast(spec).unwrap();
    let size_of = generate_size_of(ir.clone()).unwrap();
    println!("{:?}", size_of);
    let serialize = generate_serialize(ir.clone()).unwrap();
    println!("{:?}", serialize);
}

#[test]
fn simple_scalar() {
    test_compile(r#"
def_type("test") => u8;
"#);
}

#[test]
fn container() {
    test_compile(r#"
def_type("test") => container {
    field("woo") => u8;
};
"#);
    test_compile(r#"
def_type("test") => container(virtual: "true") {
    field("woo") => u8;
};
"#);
    test_compile(r#"
def_type("test") => container {
    virtual_field("len", ref: "arr", prop: "length") => u8;
    field("arr") => array(ref: "../len") => u8;
};
"#);
}
