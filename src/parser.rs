use crate::lexer::{Lexem, Paired};

#[derive(Debug)]
pub struct Token {
    pub lexem: Lexem,
    pub whitespace_before: String,
}

impl Token {
    pub fn new(lexem: Lexem, whitespace_before: impl Into<String>) -> Self {
        Self {
            lexem,
            whitespace_before: whitespace_before.into(),
        }
    }
}

impl From<Lexem> for Token {
    fn from(value: Lexem) -> Self {
        Self::new(value, String::new())
    }
}

impl Default for Token {
    fn default() -> Self {
        Lexem::Open(Paired::File).into()
    }
}

impl From<Token> for String {
    fn from(value: Token) -> Self {
        value.whitespace_before.clone() + &Into::<String>::into(value.lexem)
    }
}

pub enum Validate {
    Take,
    Insert(Token),
    Drop,
    DropBefore,
}

pub fn validate(previous: &Lexem, lexem: &Lexem) -> Validate {
    match (previous, lexem) {
        (Lexem::Comma, Lexem::Close(_)) => Validate::DropBefore,
        (Lexem::Colon, Lexem::Close(_)) => Validate::DropBefore,
        (Lexem::Open(_), Lexem::Colon) => Validate::Drop,
        (Lexem::Open(_), Lexem::Comma) => Validate::Drop,
        (Lexem::Colon, Lexem::Colon) => Validate::Drop,
        (Lexem::Comma, Lexem::Comma) => Validate::Drop,
        (Lexem::Colon, Lexem::Comma) => Validate::DropBefore,
        (Lexem::Comma, Lexem::Colon) => Validate::Drop,
        (Lexem::Close(_), Lexem::Close(_)) => Validate::Take,
        (Lexem::Close(_), Lexem::Comma) => Validate::Take,
        (Lexem::Close(_), Lexem::Colon) => Validate::Drop,
        (Lexem::Close(_), _) => Validate::Insert(Lexem::Comma.into()),
        (Lexem::Colon, Lexem::Open(_)) => Validate::Take,
        (Lexem::Comma, Lexem::Open(_)) => Validate::Take,
        (Lexem::Open(_), Lexem::Open(_)) => Validate::Take,
        (_, Lexem::Open(_)) => Validate::Insert(Lexem::Comma.into()),
        (Lexem::String(_), Lexem::String(_)) => Validate::Insert(Lexem::Comma.into()),
        (Lexem::Else(_), Lexem::String(_)) => Validate::Insert(Lexem::Comma.into()),
        (Lexem::String(_), Lexem::Else(_)) => Validate::Insert(Lexem::Comma.into()),
        (Lexem::Else(_), Lexem::Else(_)) => Validate::Insert(Lexem::Comma.into()),
        _ => Validate::Take,
    }
}

#[derive(Default, Debug)]
pub struct Parser {
    whitespace: String,
    previous: Token,
}

impl Parser {
    pub fn parse(&mut self, lexem: Lexem) -> Vec<Token> {
        let mut result = Vec::new();
        if let Lexem::WhiteSpace(s) = lexem {
            self.whitespace.push_str(s.as_str());
            return result;
        }
        let mut tokens = vec![Token::new(lexem, std::mem::take(&mut self.whitespace))];
        while let Some(token) = tokens.pop() {
            match validate(&self.previous.lexem, &token.lexem) {
                Validate::Take => {
                    result.push(std::mem::replace(&mut self.previous, token));
                }
                Validate::DropBefore => {
                    let ws = std::mem::take(&mut self.previous.whitespace_before);
                    self.previous = token;
                    self.previous.whitespace_before.push_str(&ws);
                }
                Validate::Drop => {
                    self.whitespace.push_str(&token.whitespace_before);
                }
                Validate::Insert(insert) => {
                    tokens.push(token);
                    tokens.push(insert);
                }
            }
        }
        result
    }
}
