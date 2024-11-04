use std::io::{stdin, stdout, Read, Write};

use lexer::Lexer;
use parser::Parser;
pub mod lexer;
pub mod parser;

fn process(content: &str) -> String {
    let mut lexer = Lexer::default();
    let mut parser = Parser::default();
    content
        .chars()
        .chain(Some('\0'))
        .filter_map(|c| lexer.process(c))
        .chain(Some(Default::default()))
        .flat_map(|l| parser.parse(l))
        .flatten()
        .map(Into::<String>::into)
        .collect()
}

fn main() {
    let mut content = String::new();
    stdin().read_to_string(&mut content).unwrap();
    let processed = process(content.as_str());
    stdout().write_all(processed.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

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
