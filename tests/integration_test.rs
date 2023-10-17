// tests/integration_test.rs
use std::process::Command;
use std::fs;

#[test]
fn test_print_tree() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-d")
        .arg("./rtree")  // use rtree as the test directory
        .output()
        .expect("Failed to execute command");

    let expected_output = fs::read_to_string("./tests/expected_output.txt")  // replace with a real file
        .expect("Failed to read expected output");

    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected_output);
}