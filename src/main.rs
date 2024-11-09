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
        assert_eq!(r#"{"a":3}"#, process(r#"{,"a":3,}"#))
    }

    #[test]
    fn insert_comma() {
        assert_eq!("[1, 2]", process("[1 2]"));
    }

    #[test]
    fn correct_quotes() {
        assert_eq!(r#"{"key":"value"}"#, process(r#"{`key`:'value'}"#))
    }

    #[test]
    fn multiline_string() {
        assert_eq!(
            r#"["some\nmultiline\nescaped"]"#,
            process("[\"some\nmultiline\\nescaped\"]")
        );
    }

    #[test]
    fn fix_value() {
        assert_eq!(
            r#"[null, null, null, true, false, "unknown"]"#,
            process("[nil nul None TruE False unknown]")
        );
    }

    #[test]
    fn auto_quote() {
        assert_eq!(r#"{"a":3}"#, process(r#"{a:3}"#));
    }

    #[test]
    fn correct_colon() {
        assert_eq!(r#"{"a":3}"#, process(r#"{"a"=3}"#));
    }

    #[test]
    fn insert_colon() {
        assert_eq!(r#"{"b":3}"#, process(r#"{"b"3}"#))
    }

    #[test]
    fn fix_brackets() {
        assert_eq!("{[{[{}]}]}", process("{[{[{]]"));
    }

    #[test]
    fn replace_parenthesis() {
        assert_eq!("[[],[]]", process("((),())"));
    }

    #[test]
    fn preserve_whitespaces() {
        assert_eq!(
            " [  1,   2,\n\t  3     ]\n\t\t\r\n",
            process(" [  1,   2,\n\t  3     ]\n\t\t\r\n")
        );
    }
}
