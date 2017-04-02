use ::json_to_final_ast;
use ::spec_type_to_final_ast;
use super::super::size_of::generate_size_of;

fn test_size_of(spec: &str) {
    let ir = spec_type_to_final_ast(spec).unwrap();
    let size_of = generate_size_of(ir).unwrap();
    println!("{:?}", size_of);
}

#[test]
fn simple_scalar() {
    test_size_of(r#"
def_type("test") => u8;
"#);
}

#[test]
fn container() {
    test_size_of(r#"
def_type("test") => container {
    field("woo") => u8;
};
"#);
    test_size_of(r#"
def_type("test") => container(virtual: "true") {
    field("woo") => u8;
};
"#);
    test_size_of(r#"
def_type("test") => container {
    virtual_field("len", ref: "arr", prop: "length") => u8;
    field("arr") => array(ref: "../len") => u8;
};
"#);
}
