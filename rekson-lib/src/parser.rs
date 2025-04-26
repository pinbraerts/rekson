use crate::lexer::{fix_lexem, Lexem, Paired};

#[derive(Debug, Default)]
pub struct Token {
    pub lexem: Lexem,
    pub whitespace_before: Vec<u8>,
    pub value: Vec<u8>,
}

impl Token {
    pub fn new(
        lexem: Lexem,
        whitespace_before: impl Into<Vec<u8>>,
        value: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            lexem,
            whitespace_before: whitespace_before.into(),
            value: value.into(),
        }
    }
}

impl From<Lexem> for Token {
    fn from(lexem: Lexem) -> Self {
        Self::new(lexem, Vec::new(), lexem)
    }
}

impl From<Token> for Vec<u8> {
    fn from(value: Token) -> Self {
        let mut result = value.whitespace_before.clone();
        result.extend(fix_lexem(value.lexem, value.value));
        result
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

#[derive(Debug)]
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
    whitespace: Vec<u8>,
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
    if cfg!(debug_assertions) {
        eprint!("{state:?} {lexem:?} ");
    }
    let current_state = match state {
        State::File => match lexem {
            Lexem::Close(Paired::File) => Validate::Take(state),
            Lexem::Open(paired) => Validate::Take(paired.into()),
            _ => Validate::Insert(Lexem::Open(Paired::Brace)),
        },
        State::Array => match lexem {
            Lexem::Comma | Lexem::Colon => Validate::Drop,
            Lexem::String | Lexem::Else => Validate::Push(State::Value),
            Lexem::Open(paired) => Validate::Push(paired.into()),
            Lexem::Close(Paired::Brace) => Validate::Insert(Lexem::Open(Paired::Brace)),
            Lexem::Close(Paired::File) => Validate::Insert(Lexem::Close(Paired::Bracket)),
            Lexem::Close(Paired::Bracket) => Validate::Take(State::Value),
            _ => unreachable!(),
        },
        State::Object => match lexem {
            Lexem::Comma | Lexem::Colon => Validate::Drop,
            Lexem::String | Lexem::Else => Validate::Push(State::Key),
            Lexem::Open(_) => Validate::Drop,
            Lexem::Close(Paired::Brace) => Validate::Take(State::Value),
            Lexem::Close(Paired::File) => Validate::Insert(Lexem::Close(Paired::Brace)),
            Lexem::Close(_) => Validate::Drop,
            _ => unreachable!(),
        },
        State::Value => match lexem {
            Lexem::Comma => Validate::Take(State::Comma),
            Lexem::Colon => Validate::Drop,
            _ => Validate::Insert(Lexem::Comma),
        },
        State::Key => match lexem {
            Lexem::Close(Paired::Bracket) | Lexem::Comma => Validate::Drop,
            Lexem::Colon => Validate::Take(State::Colon),
            Lexem::Open(_) | Lexem::Close(_) | Lexem::Else | Lexem::String => {
                Validate::Insert(Lexem::Colon)
            }
            _ => unreachable!(),
        },
        State::Colon => match lexem {
            Lexem::String | Lexem::Else => Validate::Take(State::Value),
            Lexem::Comma | Lexem::Colon => Validate::Drop,
            Lexem::Open(paired) => Validate::Take(paired.into()),
            Lexem::Close(paired) => Validate::Insert(Lexem::Open(paired)),
            _ => unreachable!(),
        },
        State::Comma => match lexem {
            Lexem::Open(_) | Lexem::String | Lexem::Else => Validate::Pop,
            Lexem::Comma | Lexem::Colon => Validate::Drop,
            Lexem::Close(_) => Validate::DropBefore,
            _ => unreachable!(),
        },
    };
    if cfg!(debug_assertions) {
        eprintln!("{current_state:?}");
    }
    current_state
}

impl Parser {
    pub fn parse(&mut self, lexem: Lexem, value: Vec<u8>) -> Vec<Token> {
        if cfg!(debug_assertions) {
            eprintln!("LEXEM {lexem:?} {value:?}");
        }
        let mut result = Vec::new();
        if let Lexem::Comment(_) = lexem {
            return result;
        }
        if let Lexem::WhiteSpace = lexem {
            self.whitespace.extend(value);
            return result;
        }
        let mut tokens = vec![Token::new(
            lexem,
            std::mem::take(&mut self.whitespace),
            value,
        )];
        while let Some(mut token) = tokens.pop() {
            match validate(self.states.last().cloned().unwrap_or_default(), token.lexem) {
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
                    if let Some(mut t) = std::mem::take(&mut self.delay) {
                        t.whitespace_before.extend(token.whitespace_before);
                        token.whitespace_before = t.whitespace_before;
                    }
                    tokens.push(token);
                    continue;
                }
            };
            if cfg!(debug_assertions) {
                eprintln!("{0:?} ", self.states);
            }
            result.extend(std::mem::replace(&mut self.delay, Some(token)));
        }
        result
    }
}
