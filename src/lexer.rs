#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Paired {
    Parenthesis, // ()
    Bracket,     // []
    Brace,       // {}
    File,        // start, end
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Lexem {
    String(String),
    Open(Paired),
    Close(Paired),
    Comma,
    Colon,
    Else(String),
    WhiteSpace(String),
}

fn fix_str(s: &str) -> String {
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

fn fix_else(s: &str) -> String {
    match s.to_lowercase().as_str() {
        "null" | "nil" | "nul" | "none" => "null".into(),
        "true" => "true".into(),
        "false" => "false".into(),
        &_ => {
            if s.chars().all(|c| c.is_numeric()) {
                s.into()
            } else {
                fix_str(s)
            }
        }
    }
}

impl From<&Lexem> for String {
    fn from(val: &Lexem) -> Self {
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
            Lexem::Else(s) => fix_else(&s),
            Lexem::String(s) => fix_str(s.get(1..s.len() - 1).unwrap_or_default()),
            Lexem::WhiteSpace(s) => s.clone(),
        }
    }
}

pub fn lexer(state: &mut Option<Lexem>, character: char) -> Option<Lexem> {
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
        ':' | '=' => Lexem::Colon,
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
