use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
    process::Stdio,
    thread,
};

pub fn read(path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut content = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut content)?;
    Ok(content)
}

pub fn run(input: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut project = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    project.push("target/release/rekson");
    let mut rekson = std::process::Command::new(project)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut stdin = rekson.stdin.take().unwrap();
    thread::spawn(move || {
        stdin.write_all(&input).unwrap();
    });
    Ok(rekson.wait_with_output()?.stdout)
}
