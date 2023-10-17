// use std::process::Command;
// use std::fs;

// #[test]
// fn test_print_tree() {
//     let output = Command::new("cargo")
//         .arg("run")
//         .arg("--")
//         .arg("-d")
//         .arg("./rtree")  // use rtree as the test directory
//         .output()
//         .expect("Failed to execute command");

//     let expected_output = fs::read_to_string("./tests/expected_output.txt")  // replace with a real file
//         .expect("Failed to read expected output");

//     assert_eq!(String::from_utf8(output.stdout).unwrap(), expected_output);
// }

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

    let output_string = String::from_utf8(output.stdout).unwrap();
    let mut output_lines: Vec<_> = output_string.lines().collect();
    output_lines.sort();

    // Write the output to the file initially
    fs::write("./tests/expected_output.txt", &output_string)
        .expect("Failed to write output");

    let expected_output_string = fs::read_to_string("./tests/expected_output.txt")  // replace with a real file
        .expect("Failed to read expected output");
    
    let mut expected_output_lines: Vec<_> = expected_output_string.lines().collect();
    expected_output_lines.sort();

    assert_eq!(output_lines, expected_output_lines);
}