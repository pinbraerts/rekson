use std::{ffi::OsString, path::PathBuf};
mod common;
use common::{get_json, read, run};

fn compare(name: &OsString, input: &PathBuf, output: &PathBuf) {
    println!("comparing {:?}", name);
    let input = read(input).expect("failed to read input");
    let expected_output = read(output).expect("failed to read output file");
    let output = run(input).expect("failed to execute a program");
    assert!(expected_output == output);
}

#[test]
fn convert() {
    let input = get_json("tests/input").expect("failed to read input dir");
    assert!(!input.is_empty());
    let output = get_json("tests/output").expect("failed to read output dir");
    assert!(!output.is_empty());
    input
        .into_iter()
        .filter_map(|i| output.get(&i.0).map(|o| (i.0, i.1, o)))
        .for_each(|x| compare(&x.0, &x.1, x.2));
}
