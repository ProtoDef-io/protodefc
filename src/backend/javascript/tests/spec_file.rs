#[test]
fn simple_type_usage() {
    ::spec_to_final_compilation_unit(r#"
@type "integer"
def_native("root_test_1");

def("root_test_2") => root_test_1;
def("root_test_3") => ::root_test_1;

namespace("test_ns") {
    @type "integer"
    def_native("ns_test_1");

    def("ns_test_2") => ::root_test_1;
    def("ns_test_3") => ns_test_1;
    def("ns_test_4") => ::test_ns::ns_test_1;

    def("ns_test_5") => container {
        field("tag") => ::root_test_1;
        field("data") => union("test_union", ref: "../tag") {
            variant("zero", match: "0") => ::root_test_1;
            variant("one", match: "1") => container {
                field("woo") => ::root_test_1;
                field("hoo") => ::root_test_1;
            };
        };
    };
};
"#).unwrap();
}
