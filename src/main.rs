use std::io::{stdin, stdout, Read, Write};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Paired {
    Parenthesis, // ()
    Bracket,     // []
    Brace,       // {}
    File,        // start, end
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Quote {
    Single,
    Double,
    Reversed,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum LexemType {
    Escape,
    String(Quote),
    Open(Paired),
    Close(Paired),
    Comma,
    Colon,
    Else(char),
}

enum ParseResult<T> {
    Take,
    InsertBefore(T),
    Drop,
    DropBefore,
}

impl From<Quote> for char {
    fn from(val: Quote) -> Self {
        match val {
            Quote::Single => '\'',
            Quote::Double => '"',
            Quote::Reversed => '`',
        }
    }
}

impl From<LexemType> for char {
    fn from(val: LexemType) -> Self {
        match val {
            LexemType::Comma => ',',
            LexemType::Colon => ':',
            LexemType::Escape => '\\',
            LexemType::Else(c) => c,
            LexemType::Open(Paired::Bracket) => '[',
            LexemType::Close(Paired::Bracket) => ']',
            LexemType::Open(Paired::Brace) => '{',
            LexemType::Close(Paired::Brace) => '}',
            LexemType::Open(Paired::Parenthesis) => '(',
            LexemType::Close(Paired::Parenthesis) => ')',
            LexemType::String(quote) => quote.into(),
            LexemType::Open(Paired::File) | LexemType::Close(Paired::File) => '\0',
        }
    }
}

fn lexer(states: &mut Vec<LexemType>, character: char) -> LexemType {
    let state = states
        .last()
        .cloned()
        .unwrap_or(LexemType::Open(Paired::File));
    match state {
        LexemType::String(quote) => {
            if character == quote.into() {
                states.pop();
            } else if character == '\\' {
                states.push(LexemType::Escape);
            }
            return LexemType::Else(character);
        }
        LexemType::Escape => {
            states.pop();
            return state;
        }
        _ => {}
    };
    let state = match character {
        '(' => LexemType::Open(Paired::Parenthesis),
        ')' => LexemType::Close(Paired::Parenthesis),
        '[' => LexemType::Open(Paired::Bracket),
        ']' => LexemType::Close(Paired::Bracket),
        '{' => LexemType::Open(Paired::Brace),
        '}' => LexemType::Close(Paired::Brace),
        ',' => LexemType::Comma,
        ':' => LexemType::Colon,
        '"' => LexemType::String(Quote::Double),
        '\'' => LexemType::String(Quote::Single),
        '`' => LexemType::String(Quote::Reversed),
        _ => LexemType::Else(character),
    };
    match state {
        LexemType::Open(_) | LexemType::String(_) => {
            states.push(state);
        }
        LexemType::Close(_) => {
            states.pop();
        }
        _ => {}
    }
    state
}

fn parse(state: LexemType, lexem: LexemType) -> ParseResult<LexemType> {
    match (state, lexem) {
        (LexemType::Comma, LexemType::Close(_)) => ParseResult::DropBefore,
        (LexemType::Colon, LexemType::Close(_)) => ParseResult::DropBefore,
        (LexemType::Open(_), LexemType::Colon) => ParseResult::Drop,
        (LexemType::Open(_), LexemType::Comma) => ParseResult::Drop,
        (LexemType::Colon, LexemType::Colon) => ParseResult::Drop,
        (LexemType::Comma, LexemType::Comma) => ParseResult::Drop,
        (LexemType::Colon, LexemType::Comma) => ParseResult::DropBefore,
        (LexemType::Comma, LexemType::Colon) => ParseResult::Drop,
        (LexemType::Close(_), LexemType::Open(_)) => ParseResult::InsertBefore(LexemType::Colon),
        _ => ParseResult::Take,
    }
}

fn process(content: &str) -> String {
    let mut result = String::new();
    let mut states = Vec::new();
    let mut state = LexemType::Open(Paired::File);
    for character in content.chars() {
        let mut lexem = lexer(&mut states, character);
        loop {
            match parse(state, lexem) {
                ParseResult::Take => {
                    if state != LexemType::Open(Paired::File) {
                        result.push(state.into());
                    }
                    state = lexem;
                    break;
                }
                ParseResult::Drop => {
                    break;
                }
                ParseResult::DropBefore => {
                    state = lexem;
                    break;
                }
                ParseResult::InsertBefore(insert) => {
                    lexem = insert;
                }
            }
        }
    }
    let mut lexem = LexemType::Close(Paired::File);
    loop {
        match parse(state, lexem) {
            ParseResult::Take => {
                if state != LexemType::Open(Paired::File) && state != LexemType::Close(Paired::File)
                {
                    result.push(state.into());
                }
                break;
            }
            ParseResult::Drop | ParseResult::DropBefore => {
                break;
            }
            ParseResult::InsertBefore(insert) => {
                lexem = insert;
            }
        }
    }
    result
}

fn main() {
    let mut reader = stdin();
    let mut content = String::new();
    if let Err(error) = reader.read_to_string(&mut content) {
        eprintln!("{error}");
        return;
    }
    let processed = process(content.as_str());
    if let Err(error) = stdout().write_all(processed.as_bytes()) {
        eprintln!("{error}");
    }
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
        assert_eq!(r#"{"a":3,"b": 4}"#, process(r#"{,"a":3,"b": 4,}"#))
    }

    #[test]
    fn remove_colon() {
        assert_eq!(r#"{"a":3,"b": 4}"#, process(r#"{:"a":3,"b": 4:}"#))
    }

    #[test]
    fn remove_comma_and_colon() {
        assert_eq!("", process(r#":,:::,,,:,,:::"#));
    }
}
