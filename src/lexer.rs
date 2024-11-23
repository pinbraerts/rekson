use std::borrow::Cow;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Paired {
    Bracket, // []
    Brace,   // {}
    File,    // start, end
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Lexem {
    String(Vec<u8>),
    Open(Paired),
    Close(Paired),
    Comma,
    Colon,
    Else(Vec<u8>),
    WhiteSpace(Vec<u8>),
}

impl Default for Lexem {
    fn default() -> Self {
        Self::Close(Paired::File)
    }
}

fn fix_str(s: Cow<'_, str>) -> Cow<'_, str> {
    Some('\"')
        .into_iter()
        .chain(
            s.replace('\n', r"\n")
                .replace("\\\"", "\"")
                .replace('"', "\\\"")
                .chars(),
        )
        .chain(Some('\"'))
        .collect()
}

fn is_numeric(s: &str) -> bool {
    let c = match s.chars().next() {
        Some('+') | Some('-') | Some('.') => s.chars().nth(1),
        Some(c) => Some(c),
        None => None,
    };
    c.unwrap_or_default().is_ascii_digit()
}

fn fix_else(s: Cow<'_, str>) -> Cow<'_, str> {
    match s.to_lowercase().as_str() {
        "null" | "nil" | "nul" | "none" => "null".into(),
        "true" => "true".into(),
        "false" => "false".into(),
        &_ => {
            if is_numeric(&s) {
                s
            } else {
                fix_str(s)
            }
        }
    }
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
            Lexem::Open(Paired::File) | Lexem::Close(Paired::File) => "".into(),
            Lexem::Else(s) => fix_else(String::from_utf8_lossy(&s)).into_owned(),
            Lexem::String(s) => fix_str(String::from_utf8_lossy(
                s.get(1..s.len() - 1).unwrap_or_default(),
            ))
            .into_owned(),
            Lexem::WhiteSpace(s) => String::from_utf8_lossy(&s).into_owned(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Lexer {
    state: Option<Lexem>,
}

impl Lexer {
    pub fn process(&mut self, character: u8) -> Option<Lexem> {
        if let Some(Lexem::String(s)) = &mut self.state {
            let first_char = s.iter().next().cloned().unwrap_or_default();
            let last_char = s.iter().last().cloned().unwrap_or_default();
            s.push(character);
            if last_char != b'\\' && character == first_char {
                return std::mem::take(&mut self.state);
            }
            return None;
        }
        let next = match character {
            b'[' => Lexem::Open(Paired::Bracket),
            b']' => Lexem::Close(Paired::Bracket),
            b'(' => Lexem::Open(Paired::Bracket),
            b')' => Lexem::Close(Paired::Bracket),
            b'{' => Lexem::Open(Paired::Brace),
            b'}' => Lexem::Close(Paired::Brace),
            b'\0' => Lexem::Close(Paired::File),
            b',' => Lexem::Comma,
            b':' | b'=' => Lexem::Colon,
            b'"' | b'\'' | b'`' => {
                return std::mem::replace(&mut self.state, Some(Lexem::String(vec![character])));
            }
            _ => {
                if character.is_ascii_whitespace() {
                    if let Some(Lexem::WhiteSpace(s)) = &mut self.state {
                        s.push(character);
                        return None;
                    }
                    Lexem::WhiteSpace(vec![character])
                } else {
                    if let Some(Lexem::Else(s)) = &mut self.state {
                        s.push(character);
                        return None;
                    }
                    Lexem::Else(vec![character])
                }
            }
        };
        std::mem::replace(&mut self.state, Some(next))
    }
}
