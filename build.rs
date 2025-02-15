use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let input_path = Path::new("tests/input");
    let output_path = Path::new("tests/output");

    let var = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&var);
    let convert_path = out_dir.join("convert.rs");
    let idempotent_path = out_dir.join("idempotent.rs");

    let mut convert = File::create(&convert_path).unwrap();
    let mut idempotent = File::create(&idempotent_path).unwrap();

    for entry in fs::read_dir(input_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let file_name = path.file_name().unwrap();
        let test_name = path.file_stem().unwrap().to_str().unwrap();
        let file_name = file_name.to_str().unwrap();
        let input_file = input_path.join(file_name).to_str().unwrap().to_owned();
        let output_file = output_path.join(file_name).to_str().unwrap().to_owned();
        writeln!(
            convert,
            r#"
            #[test]
            fn test_{}() {{
                compare("{}", "{}", "{}");
            }}
            "#,
            test_name, file_name, input_file, output_file
        )
        .unwrap();
        writeln!(
            idempotent,
            r#"
            #[test]
            fn test_{}() {{
                compare("{}", "{}");
            }}
            "#,
            test_name, file_name, output_file
        )
        .unwrap();
    }
}
