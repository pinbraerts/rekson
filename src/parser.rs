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

#[derive(Debug, Default, Clone)]
enum State {
    #[default]
    File, // needs value or morph to object
    Array,  // needs value
    Value,  // needs comma
    Object, // needs key
    Key,    // needs colon
    Colon,
    Comma,
}

type States = Vec<State>;

enum Validate {
    Take(State),
    Push(State),
    Pop,
    Insert(Lexem),
    Drop,
    DropBefore,
}

#[derive(Default, Debug)]
pub struct Parser {
    whitespace: String,
    states: States,
    delay: Option<Token>,
}

impl From<Paired> for State {
    fn from(value: Paired) -> Self {
        match value {
            Paired::Brace => State::Object,
            Paired::Bracket => State::Array,
            Paired::File => State::File,
        }
    }
}

fn validate(state: State, lexem: Lexem) -> Validate {
    match state {
        State::File => match lexem {
            Lexem::Close(Paired::File) => Validate::Drop,
            Lexem::Open(paired) => Validate::Take(paired.into()),
            _ => Validate::Insert(Lexem::Open(Paired::Brace)),
        },
        State::Array => match lexem {
            Lexem::Comma | Lexem::Colon => Validate::Drop,
            Lexem::String(_) | Lexem::Else(_) => Validate::Push(State::Value),
            Lexem::Open(paired) => Validate::Push(paired.into()),
            Lexem::Close(Paired::Bracket) => Validate::Take(State::Value),
            Lexem::Close(_) => Validate::Insert(Lexem::Close(Paired::Bracket)),
            Lexem::WhiteSpace(_) => unreachable!(),
        },
        State::Object => match lexem {
            Lexem::Comma | Lexem::Colon => Validate::Drop,
            Lexem::String(_) | Lexem::Else(_) => Validate::Push(State::Key),
            Lexem::Open(paired) => Validate::Push(paired.into()),
            Lexem::Close(Paired::Brace) => Validate::Take(State::Value),
            Lexem::Close(_) => Validate::Insert(Lexem::Close(Paired::Brace)),
            Lexem::WhiteSpace(_) => unreachable!(),
        },
        State::Value => match lexem {
            Lexem::Comma => Validate::Take(State::Comma),
            Lexem::Colon => Validate::Drop,
            Lexem::Close(_) => Validate::Pop,
            Lexem::Open(_) | Lexem::Else(_) | Lexem::String(_) => Validate::Insert(Lexem::Comma),
            Lexem::WhiteSpace(_) => unreachable!(),
        },
        State::Key => match lexem {
            Lexem::Comma => Validate::Drop,
            Lexem::Colon => Validate::Take(State::Colon),
            Lexem::Open(_) | Lexem::Close(_) | Lexem::Else(_) | Lexem::String(_) => {
                Validate::Insert(Lexem::Colon)
            }
            Lexem::WhiteSpace(_) => unreachable!(),
        },
        State::Colon => match lexem {
            Lexem::String(_) | Lexem::Else(_) => Validate::Take(State::Value),
            Lexem::Comma | Lexem::Colon => Validate::Drop,
            Lexem::Open(paired) => Validate::Take(paired.into()),
            Lexem::Close(paired) => Validate::Insert(Lexem::Open(paired)),
            Lexem::WhiteSpace(_) => unreachable!(),
        },
        State::Comma => match lexem {
            Lexem::String(_) | Lexem::Else(_) => Validate::Pop,
            Lexem::Comma | Lexem::Colon => Validate::Drop,
            Lexem::Open(paired) => Validate::Take(paired.into()),
            Lexem::Close(_) => Validate::DropBefore,
            Lexem::WhiteSpace(_) => unreachable!(),
        },
    }
}

impl Parser {
    pub fn parse(&mut self, lexem: Lexem) -> Vec<Token> {
        let mut result = Vec::new();
        let last = matches!(lexem, Lexem::Close(Paired::File));
        if let Lexem::WhiteSpace(s) = lexem {
            self.whitespace.push_str(s.as_str());
            return result;
        }
        let mut tokens = vec![Token::new(lexem, std::mem::take(&mut self.whitespace))];
        while let Some(token) = tokens.pop() {
            match validate(
                self.states.last().cloned().unwrap_or_default(),
                token.lexem.clone(),
            ) {
                Validate::Push(state) => {
                    self.states.push(state);
                }
                Validate::Take(state) => {
                    self.states.pop();
                    self.states.push(state);
                }
                Validate::Pop => {
                    self.states.pop();
                    tokens.push(token);
                    continue;
                }
                Validate::Insert(insert) => {
                    tokens.push(token);
                    tokens.push(insert.into());
                    continue;
                }
                Validate::Drop => {
                    continue;
                }
                Validate::DropBefore => {
                    self.states.pop();
                    if let Some(t) = std::mem::take(&mut self.delay) {
                        self.whitespace = t.whitespace_before + &self.whitespace;
                    }
                    continue;
                }
            };
            result.push(token);
        }
        if let Some(token) = std::mem::take(&mut self.delay) {
            result.insert(0, token);
        }
        if !last {
            self.delay = result.pop();
        }
        result
    }
}
