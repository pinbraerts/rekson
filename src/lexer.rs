#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Paired {
    Bracket, // []
    Brace,   // {}
    File,    // start, end
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Lexem {
    String,
    Open(Paired),
    Close(Paired),
    Comma,
    Colon,
    Else,
    WhiteSpace,
}

fn joinable(lexem: Lexem) -> bool {
    matches!(lexem, Lexem::Else | Lexem::String | Lexem::WhiteSpace)
}

impl Default for Lexem {
    fn default() -> Self {
        Self::Close(Paired::File)
    }
}

impl From<u8> for Lexem {
    fn from(character: u8) -> Self {
        match character {
            b'[' => Lexem::Open(Paired::Bracket),
            b']' => Lexem::Close(Paired::Bracket),
            b'(' => Lexem::Open(Paired::Bracket),
            b')' => Lexem::Close(Paired::Bracket),
            b'{' => Lexem::Open(Paired::Brace),
            b'}' => Lexem::Close(Paired::Brace),
            b'\0' => Lexem::Close(Paired::File),
            b',' => Lexem::Comma,
            b':' | b'=' => Lexem::Colon,
            b'\'' | b'"' | b'`' => Lexem::String,
            _ => {
                if character.is_ascii_whitespace() {
                    Lexem::WhiteSpace
                } else {
                    Lexem::Else
                }
            }
        }
    }
}

impl From<Lexem> for Option<u8> {
    fn from(val: Lexem) -> Self {
        Some(match val {
            Lexem::String => b'"',
            Lexem::Open(Paired::Bracket) => b'[',
            Lexem::Close(Paired::Bracket) => b']',
            Lexem::Open(Paired::Brace) => b'{',
            Lexem::Close(Paired::Brace) => b'}',
            Lexem::Comma => b',',
            Lexem::Colon => b':',
            _ => {
                return None;
            }
        })
    }
}

impl From<Lexem> for Vec<u8> {
    fn from(val: Lexem) -> Self {
        Into::<Option<u8>>::into(val).into_iter().collect()
    }
}

fn fix_str(s: &[u8]) -> Vec<u8> {
    let mut result = vec![b'"'];
    let mut escaped = false;
    for c in s.iter() {
        if *c == b'\\' {
            escaped = !escaped;
            continue;
        }
        match c {
            b'"' => {
                escaped = true;
            }
            b'\n' => {
                if !escaped {
                    result.push(b'\\')
                }
                result.push(b'n');
                continue;
            }
            b'\r' => {
                if !escaped {
                    result.push(b'\\')
                }
                result.push(b'r');
                continue;
            }
            _ => {}
        };
        if escaped {
            result.push(b'\\');
        }
        result.push(*c);
        escaped = false;
    }
    result.push(b'"');
    result
}

fn is_numeric(s: &[u8]) -> bool {
    let c = match s.first() {
        Some(b'+') | Some(b'-') | Some(b'.') => s.get(1),
        Some(c) => Some(c),
        None => None,
    };
    c.cloned().unwrap_or_default().is_ascii_digit()
}

fn fix_else(s: Vec<u8>) -> Vec<u8> {
    match s.to_ascii_lowercase().as_slice() {
        b"null" | b"nil" | b"nul" | b"none" => b"null".to_vec(),
        b"true" => b"true".to_vec(),
        b"false" => b"false".to_vec(),
        &_ => {
            if is_numeric(&s) {
                s
            } else {
                fix_str(&s)
            }
        }
    }
}

pub fn fix_lexem(lexem: Lexem, value: Vec<u8>) -> Vec<u8> {
    match lexem {
        Lexem::String => fix_str(if value.len() > 2 {
            value.get(1..value.len() - 1).expect("value.len() > 2")
        } else {
            &[]
        }),
        Lexem::Else => fix_else(value),
        lexem => lexem.into(),
    }
}

#[derive(Debug, Default)]
pub struct Lexer {
    state: Vec<u8>,
    lexem: Option<Lexem>,
}

impl Lexer {
    pub fn process(&mut self, character: u8) -> Option<(Lexem, Vec<u8>)> {
        if cfg!(debug_assertions) {
            eprintln!("{character} {}", character as char);
        }
        if let Some(Lexem::String) = self.lexem {
            let first_char = self.state.first().cloned().unwrap_or_default();
            let last_char = self.state.last().cloned().unwrap_or_default();
            self.state.push(character);
            if last_char != b'\\' && character == first_char {
                self.lexem = None;
                return Some((Lexem::String, std::mem::take(&mut self.state)));
            }
            return None;
        }
        let next = Lexem::from(character);
        let Some(lexem) = self.lexem else {
            self.state.push(character);
            self.lexem = Some(next);
            return None;
        };
        if joinable(lexem) && lexem == next {
            self.state.push(character);
            return None;
        }
        self.lexem = Some(next);
        Some((lexem, std::mem::replace(&mut self.state, vec![character])))
    }
}
