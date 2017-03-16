use std::process::Command;

const BUILTIN_TYPES: &'static str =
    include_str!("../../../backend_resources/javascript/builtins.js");

fn nodejs_run(script: &str) -> Vec<u8> {
    let res = Command::new("node")
        .arg("-e")
        .arg(script)
        .output()
        .expect("failed to execute nodejs");

    println!("Full response: \"\n{:?}\"", res);
    assert!(res.status.success());

    res.stdout
}

pub fn test_with_data_eq(function: &str, data: &str, compare: &str) {
    let script = format!("
let types = {};
let test_fun = {};
let in_data = {};
let expected_out = {};

var assert = require(\"assert\");
assert.deepEqual(test_fun(in_data), expected_out);

console.log(\"ok\");
", BUILTIN_TYPES, function, data, compare);

    println!("Full test script: \"\n{}\"", script);
    let res = nodejs_run(&script);

    assert_eq!(res, b"ok\n");
}

#[cfg(all(test, feature = "js_tests"))]
mod tests {
    use super::{nodejs_run, test_with_data_eq};

    #[test]
    fn node_js_availibility() {
        let out = ::std::process::Command::new("node")
            .arg("--version")
            .output()
            .expect("failed to run node.js");
        assert!(out.status.success());
    }

    #[test]
    fn nodejs_run_test() {
        assert_eq!(nodejs_run("console.log(\"woo\");"), b"woo\n")
    }

    #[test]
    fn test_with_data_eq_test() {
        let data = "{\"some\": \"thing\", \"else\": 0}";
        test_with_data_eq(
            "function(input) { return input; }",
            data, data,
        );
    }

    #[test]
    #[should_panic]
    fn test_with_data_eq_test_fail() {
        let data1 = "{\"some\": \"thing\", \"else\": 0}";
        let data2 = "{\"some\": \"thing\", \"else\": 1}";
        test_with_data_eq(
            "function(input) { return input; }",
            data1, data2,
        );
    }

}
