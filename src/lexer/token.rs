#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum Token {
    None,
    EndOfFile,

    Function,
    StartScope,
    EndScope,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Pub,
    Import,
    Use,
    DoubleColon,
    Enum,
    Struct,
    Unsafe,
    Reference,
    SemiColon,
    Return,
    Dot,
    Underscore,
    Colon,
    Equals,
    Compare,
    Comma,
    Mutable,
    Variable,
    Give,
    If,
    Else,
    Plus,
    Minus,
    Slash,
    Asterisk,
    Loop,
    While,
    Boolean(bool),
    String(String),
    Integer(usize),
    Identifier(String),
}
impl Token {
    pub fn len(&self) -> usize {
        match self {
            Token::None => 0,

            Token::Pub => 3,
            Token::Dot => 1,
            Token::Struct => 6,
            Token::While => 5,
            Token::Loop => 4,
            Token::Give => 4,
            Token::Unsafe => 6,
            Token::Function => 2,
            Token::StartScope => 1,
            Token::EndScope => 1,
            Token::OpenParen => 1,
            Token::CloseParen => 1,
            Token::OpenBracket => 1,
            Token::CloseBracket => 1,
            Token::Import => 6,
            Token::Use => 3,
            Token::DoubleColon => 2,
            Token::Reference => 1,
            Token::SemiColon => 1,
            Token::Return => 6,
            Token::Underscore => 1,
            Token::Colon => 1,
            Token::Equals => 1,
            Token::Compare => 2,
            Token::Comma => 1,
            Token::Mutable => 3,
            Token::Variable => 3,
            Token::If => 2,
            Token::Else => 4,
            Token::Enum => 4,
            Token::Boolean(bool) => match bool {
                true => 4,
                false => 5,
            },
            Token::Plus | Token::Minus | Token::Asterisk | &Token::Slash => 1,
            Token::String(string) => string.len() + 2,
            Token::Integer(int) => format!("{}", int).len(),
            Token::Identifier(name) => name.len(),
            Token::EndOfFile => 0,
        }
    }
}


#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}
impl TokenInfo {
    pub fn new(token: Token, line: usize, column: usize) -> Self {
        Self {
            column: column - token.len(),
            token,
            line,
        }
    }
    pub fn default() -> Self {
        Self {
            token: Token::None,
            line: 0,
            column: 0,
        }
    }
}
