use std::io::{stdin, stdout, Read, Write};

use lexer::{lexer, Lexem};
use parser::{validate, Token, ValidateResult};
pub mod lexer;
pub mod parser;

fn process(content: &str) -> String {
    let mut state = None;
    let mut states: Vec<Token> = Vec::new();
    let mut whitespace = String::new();
    for character in content.chars().chain(Some('\0')) {
        let lexem = lexer(&mut state, character);
        if lexem.is_none() {
            continue;
        }
        let lexem = lexem.unwrap();
        if let Lexem::WhiteSpace(s) = lexem {
            whitespace.push_str(s.as_str());
            continue;
        }
        let mut token = Token::new(lexem, std::mem::take(&mut whitespace));
        loop {
            let previous = states.pop().unwrap_or_default();
            match validate(&previous.lexem, &token.lexem) {
                ValidateResult::Take => {
                    states.push(previous);
                    states.push(token);
                    break;
                }
                ValidateResult::DropBefore => {
                    token
                        .whitespace_before
                        .push_str(&previous.whitespace_before);
                    states.push(token);
                    break;
                }
                ValidateResult::Drop => {
                    whitespace.push_str(&token.whitespace_before);
                    states.push(previous);
                    break;
                }
                ValidateResult::InsertBefore(lexem) => {
                    states.push(previous);
                    states.push(lexem.into());
                }
            }
        }
    }
    states.iter().map(Into::<String>::into).collect()
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
