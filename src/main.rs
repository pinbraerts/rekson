use std::io::{stdin, stdout, Read, Write};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Paired {
    Parenthesis, // ()
    Bracket,     // []
    Brace,       // {}
    File,        // start, end
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Lexem {
    String(String),
    Open(Paired),
    Close(Paired),
    Comma,
    Colon,
    Else(String),
    WhiteSpace(String),
}

enum ValidateResult<T> {
    Take,
    InsertBefore(T),
    Drop,
    DropBefore,
}

fn fix_str(s: &str) -> String {
    Some('\"')
        .into_iter()
        .chain(s.escape_default())
        .chain(Some('\"'))
        .collect()
}

impl From<Lexem> for String {
    fn from(val: Lexem) -> Self {
        match val {
            Lexem::Comma => ",".into(),
            Lexem::Colon => ":".into(),
            Lexem::Open(Paired::Bracket) => "[".into(),
            Lexem::Close(Paired::Bracket) => "]".into(),
            Lexem::Open(Paired::Brace) => "{".into(),
            Lexem::Close(Paired::Brace) => "}".into(),
            Lexem::Open(Paired::Parenthesis) => "(".into(),
            Lexem::Close(Paired::Parenthesis) => ")".into(),
            Lexem::Open(Paired::File) | Lexem::Close(Paired::File) => "".into(),
            Lexem::Else(s) => s,
            Lexem::String(s) => fix_str(s.get(1..s.len() - 1).unwrap_or_default()),
            Lexem::WhiteSpace(s) => s,
        }
    }
}

fn lexer(state: &mut Option<Lexem>, character: char) -> Option<Lexem> {
    if let Some(Lexem::String(s)) = state {
        let first_char = s.chars().next().unwrap_or_default();
        let last_char = s.chars().last().unwrap_or_default();
        s.push(character);
        if last_char != '\\' && character == first_char {
            return std::mem::take(state);
        }
        return None;
    }
    let next = match character {
        '(' => Lexem::Open(Paired::Parenthesis),
        ')' => Lexem::Close(Paired::Parenthesis),
        '[' => Lexem::Open(Paired::Bracket),
        ']' => Lexem::Close(Paired::Bracket),
        '{' => Lexem::Open(Paired::Brace),
        '}' => Lexem::Close(Paired::Brace),
        '\0' => Lexem::Close(Paired::File),
        ',' => Lexem::Comma,
        ':' => Lexem::Colon,
        '"' | '\'' | '`' => {
            return std::mem::replace(state, Some(Lexem::String(character.into())));
        }
        _ => {
            if character.is_whitespace() {
                if let Some(Lexem::WhiteSpace(s)) = state {
                    s.push(character);
                    return None;
                }
                Lexem::WhiteSpace(character.into())
            } else {
                if let Some(Lexem::Else(s)) = state {
                    s.push(character);
                    return None;
                }
                Lexem::Else(character.into())
            }
        }
    };
    std::mem::replace(state, Some(next))
}

struct Token {
    lexem: Lexem,
    whitespace_before: String,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            lexem: Lexem::Open(Paired::File),
            whitespace_before: String::new(),
        }
    }
}

impl From<Lexem> for Token {
    fn from(value: Lexem) -> Self {
        Self {
            lexem: value,
            whitespace_before: String::new(),
        }
    }
}

fn validate(previous: &Lexem, lexem: &Lexem) -> ValidateResult<Lexem> {
    match (previous, lexem) {
        (Lexem::Comma, Lexem::Close(_)) => ValidateResult::DropBefore,
        (Lexem::Colon, Lexem::Close(_)) => ValidateResult::DropBefore,
        (Lexem::Open(_), Lexem::Colon) => ValidateResult::Drop,
        (Lexem::Open(_), Lexem::Comma) => ValidateResult::Drop,
        (Lexem::Colon, Lexem::Colon) => ValidateResult::Drop,
        (Lexem::Comma, Lexem::Comma) => ValidateResult::Drop,
        (Lexem::Colon, Lexem::Comma) => ValidateResult::DropBefore,
        (Lexem::Comma, Lexem::Colon) => ValidateResult::Drop,
        (Lexem::Close(_), Lexem::Close(_)) => ValidateResult::Take,
        (Lexem::Close(_), Lexem::Comma) => ValidateResult::Take,
        (Lexem::Close(_), Lexem::Colon) => ValidateResult::Drop,
        (Lexem::Close(_), _) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::Colon, Lexem::Open(_)) => ValidateResult::Take,
        (Lexem::Comma, Lexem::Open(_)) => ValidateResult::Take,
        (Lexem::Open(_), Lexem::Open(_)) => ValidateResult::Take,
        (_, Lexem::Open(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::String(_), Lexem::String(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::Else(_), Lexem::String(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::String(_), Lexem::Else(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        (Lexem::Else(_), Lexem::Else(_)) => ValidateResult::InsertBefore(Lexem::Comma),
        _ => ValidateResult::Take,
    }
}

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
        let mut token = Token {
            lexem,
            whitespace_before: std::mem::take(&mut whitespace),
        };
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
    let mut result = String::new();
    for token in states {
        result.push_str(&token.whitespace_before);
        let string: String = token.lexem.into();
        result.push_str(string.as_str());
    }
    result
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

    #[test]
    fn insert_comma() {
        assert_eq!("[],[]", process("[][]"));
        assert_eq!("1, 2", process("1 2"));
    }

    #[test]
    fn fix_string() {
        assert_eq!(
            "\"some\\nmultiline\\nstring\"",
            process("'some\nmultiline\nstring'")
        );
    }
}
