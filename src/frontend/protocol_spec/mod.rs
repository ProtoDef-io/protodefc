pub mod ast;

pub mod from_ir;
pub mod to_ir;

pub use self::ast::parser::parse;
pub use self::to_ir::spec::type_def_to_ir;
pub use self::to_ir::compilation_unit::to_compilation_unit;

#[cfg(test)]
mod tests {
    use super::to_compilation_unit;

    #[test]
    fn spec_to_compilation_unit() {
        let result = to_compilation_unit(r#"
def("root_type") => u8;
namespace("some_namespace") {
    def("inner_type") => u8;
    namespace("inner_namespace") {
        def("deep_type") => u8;
    };
};
"#)
            .unwrap();
        println!("{:?}", result);
    }

}
