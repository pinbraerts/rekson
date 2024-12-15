use std::{
    collections::HashMap,
    ffi::OsString,
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
    process::Stdio,
    thread,
};

pub fn get_json(dirname: impl Into<PathBuf>) -> io::Result<HashMap<OsString, PathBuf>> {
    let mut project = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    println!("{:?}", project);
    project.push(dirname.into());
    println!("{:?}", project);
    Ok(std::fs::read_dir(project)?
        .filter_map(|f| f.ok())
        .map(|f| (f.file_name(), f.path().to_owned()))
        .collect())
}

pub fn read(path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut content = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut content)?;
    Ok(content)
}

pub fn run(input: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut rekson = std::process::Command::new("./target/release/rekson")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = rekson.stdin.take().unwrap();
    thread::spawn(move || {
        stdin.write_all(&input).unwrap();
    });
    Ok(rekson.wait_with_output()?.stdout)
}
