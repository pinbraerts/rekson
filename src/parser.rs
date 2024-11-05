use crate::lexer::{Lexem, Paired};

pub enum ValidateResult<T> {
    Take,
    InsertBefore(T),
    Drop,
    DropBefore,
}

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

pub fn validate(previous: &Lexem, lexem: &Lexem) -> ValidateResult<Lexem> {
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
        let token = Token::new(lexem, std::mem::take(&mut self.whitespace));
        loop {
            match validate(&self.previous.lexem, &token.lexem) {
                ValidateResult::Take => {
                    result.push(std::mem::replace(&mut self.previous, token));
                    break;
                }
                ValidateResult::DropBefore => {
                    let ws = std::mem::take(&mut self.previous.whitespace_before);
                    self.previous = token;
                    self.previous.whitespace_before.push_str(&ws);
                    break;
                }
                ValidateResult::Drop => {
                    self.whitespace.push_str(&token.whitespace_before);
                    break;
                }
                ValidateResult::InsertBefore(lexem) => {
                    result.push(std::mem::replace(&mut self.previous, lexem.into()));
                }
            }
        }
        result
    }
}
