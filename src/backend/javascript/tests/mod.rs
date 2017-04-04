mod spec_single_type;
mod spec_file;

use std::process::Command;

const BUILTIN_TYPES: &'static str =
    include_str!("../../../../backend_resources/javascript/builtins.js");

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

pub fn test_with_data_eq(function: &str, compare: &str) {
    let script = format!("
let types = {};
let test_fun = {};

var assert = require(\"assert\");

{}

console.log(\"ok\");
", BUILTIN_TYPES, function, compare);

    println!("Full test script: \"\n{}\"", script);
    let res = nodejs_run(&script);

    assert!(res.ends_with(b"ok\n"));
}

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
        &format!("assert.deepEqual(test_fun({}), {});", data, data),
    );
}

#[test]
#[should_panic]
fn test_with_data_eq_test_fail() {
    let data1 = "{\"some\": \"thing\", \"else\": 0}";
    let data2 = "{\"some\": \"thing\", \"else\": 1}";
    test_with_data_eq(
        "function(input) { return input; }",
        &format!("assert.deepEqual(test_fun({}), {});", data1, data2)
    );
}
