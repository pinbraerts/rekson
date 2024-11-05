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
        .map(|c| c as char)
        .chain(Some('\0'))
        .filter_map(|c| lexer.process(c))
        .chain(Some(Default::default()))
        .flat_map(|l| parser.parse(l))
        .map(String::from)
        .for_each(|s| writer.write_all(s.as_bytes()).unwrap());
}

fn main() {
    let reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout());
    process_streams(reader, &mut writer, 256);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process(content: &str) -> String {
        let reader = BufReader::new(content.as_bytes());
        let mut writer = BufWriter::new(Vec::new());
        process_streams(reader, &mut writer, 256);
        String::from_utf8(writer.into_inner().unwrap()).unwrap()
    }

    #[test]
    fn empty() {
        assert_eq!("", process(""));
    }

    #[test]
    fn valid() {
        let value = r#"{"a":3,"b": 4}"#;
        assert_eq!(value, process(value));
    }

    #[test]
    fn remove_comma() {
        assert_eq!(r#"{"a":3,"b": 4}"#, process(r#"{,"a":3,"b"= 4,}"#))
    }

    #[test]
    fn remove_colon() {
        assert_eq!(r#"{"a":3,"b": 4}"#, process(r#"{:"a"=3,"b": 4:}"#))
    }

    #[test]
    fn remove_comma_and_colon() {
        assert_eq!("", process(r#":,:=:,,,:,,::="#));
    }

    #[test]
    fn insert_comma() {
        assert_eq!("[],[]", process("[][]"));
        assert_eq!("1, 2", process("1 2"));
    }

    #[test]
    fn fix_string() {
        assert_eq!(
            "\"some\\nmultiline\\nstring\", \"\\\"some\\nescaped\\nstring\"",
            process("'some\nmultiline\nstring', '\"some\\nescaped\\nstring'")
        );
    }

    #[test]
    fn fix_value() {
        assert_eq!(
            "null, null, null, true, false",
            process("nil nul None TruE False")
        );
    }
}
