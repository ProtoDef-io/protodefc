use ::spec_to_final_compilation_unit;
fn test_compile(spec: &str) {
    let _ir = spec_to_final_compilation_unit(spec).unwrap();
    //let size_of = generate_size_of(ir.clone()).unwrap();
    //println!("{:?}", size_of);
    //let serialize = generate_serialize(ir.clone()).unwrap();
    //println!("{:?}", serialize);
}

#[test]
fn simple_scalar() {
    test_compile(r#"
@type "integer"
def_native("u8");

def("test") => u8;
"#);
}

#[test]
fn container() {
    test_compile(r#"
@type "integer"
def_native("u8");

def("test") => container {
    field("woo") => u8;
};
"#);
    test_compile(r#"
@type "integer"
def_native("u8");

def("test") => container(virtual: "true") {
    field("woo") => u8;
};
"#);
    test_compile(r#"
@type "integer"
def_native("u8");

def("test") => container {
    virtual_field("len", value: "arr/@length") => u8;
    field("arr") => array(length: "../len") => u8;
};
"#);
}
