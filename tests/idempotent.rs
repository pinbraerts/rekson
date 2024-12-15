use std::{ffi::OsString, path::PathBuf};
mod common;
use common::{get_json, read, run};

fn compare(name: &OsString, input: &PathBuf) {
    println!("comparing {:?}", name);
    let input = read(input).expect("failed to read input");
    let output = run(input.clone()).expect("failed to execute a program");
    assert!(input == output);
}

#[test]
fn idempotent() {
    let input = get_json("tests/output").expect("failed to read input dir");
    assert!(!input.is_empty());
    input.into_iter().for_each(|x| compare(&x.0, &x.1));
}
