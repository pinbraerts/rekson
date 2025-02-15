use std::{path::PathBuf, str::FromStr};
mod common;
use common::{read, run};

fn compare(name: &str, input: &str, output: &str) {
    println!("comparing {:?}", name);
    let input = PathBuf::from_str(input).unwrap();
    let output = PathBuf::from_str(output).unwrap();
    let input = read(&input).expect("failed to read input");
    let expected_output = read(&output).expect("failed to read output file");
    let output = run(input).expect("failed to execute a program");
    assert!(expected_output == output);
}

include!(concat!(env!("OUT_DIR"), "/convert.rs"));
