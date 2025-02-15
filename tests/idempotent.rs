use std::{path::PathBuf, str::FromStr};
mod common;
use common::{read, run};

fn compare(name: &str, input: &str) {
    println!("comparing {:?}", name);
    let input = PathBuf::from_str(input).unwrap();
    let input = read(&input).expect("failed to read input");
    let output = run(input.clone()).expect("failed to execute a program");
    assert!(input == output);
}

include!(concat!(env!("OUT_DIR"), "/idempotent.rs"));
