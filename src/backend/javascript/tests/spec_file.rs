#[test]
fn simple_type_usage() {
    ::spec_to_final_compilation_unit(r#"
def_type("root_test_1") => u8;

def_type("root_test_2") => root_test_1;
def_type("root_test_3") => ::root_test_1;

namespace("test_ns") {
    def_type("ns_test_1") => u8;

    def_type("ns_test_2") => ::root_test_1;
    def_type("ns_test_3") => ns_test_1;
    def_type("ns_test_4") => ::test_ns::ns_test_1;
};
"#).unwrap();
}
