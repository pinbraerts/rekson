pub mod chunks;
pub mod lexer;
pub mod parser;

use std::io::{BufReader, BufWriter, Write};

pub fn process_streams<In, Out>(
    reader: BufReader<In>,
    writer: &mut BufWriter<Out>,
    chunk_size: usize,
) where
    In: std::io::Read,
    Out: std::io::Write,
{
    let mut lexer = lexer::Lexer::default();
    let mut parser = parser::Parser::default();
    chunks::ChunkReader::new(reader, chunk_size)
        .flatten()
        .chain(Some(0))
        .filter_map(|c| lexer.process(c))
        .chain([Default::default(), Default::default()])
        .flat_map(|(l, v)| parser.parse(l, v))
        .map(Vec::<u8>::from)
        .for_each(|s| writer.write_all(&s).unwrap());
}

pub fn process_bytes(input: &[u8]) -> Vec<u8> {
    // create reader from string
    let reader = BufReader::new(input);
    let mut output = Vec::new();
    let mut writer = BufWriter::new(&mut output);
    let chunk_size = 256;
    process_streams(reader, &mut writer, chunk_size);
    drop(writer);
    output
}
