pub mod chunks;
pub mod lexer;
pub mod parser;

use std::io::{stdin, stdout, BufReader, BufWriter, Write};

use chunks::ChunkReader;
use lexer::Lexer;
use parser::Parser;

fn process_streams<In, Out>(reader: BufReader<In>, writer: &mut BufWriter<Out>, chunk_size: usize)
where
    In: std::io::Read,
    Out: std::io::Write,
{
    let mut lexer = Lexer::default();
    let mut parser = Parser::default();
    ChunkReader::new(reader, chunk_size)
        .flatten()
        .chain(Some(0))
        .filter_map(|c| lexer.process(c))
        .chain([Default::default(), Default::default()])
        .flat_map(|l| parser.parse(l))
        .map(String::from)
        .for_each(|s| writer.write_all(s.as_bytes()).unwrap());
}

fn main() {
    let reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());
    process_streams(reader, &mut writer, 256);
}
