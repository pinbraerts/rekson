use std::io::{stdin, stdout, BufReader, BufWriter};

fn main() {
    let reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());
    rekson_lib::process_streams(reader, &mut writer, 256);
}
