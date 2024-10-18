use std::fmt;

use super::Span;

#[derive(Debug, Clone, PartialOrd)]
pub struct Token {
    pub kind: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenType, span: Span) -> Self {
        Token { kind, span }
    }
}

#[macro_export]
macro_rules! token {
    ($kind:ident, $span: expr) => {
        $crate::ast::Token::new($crate::ast::TokenType::$kind, $span)
    };

    ($kind:ident, $str: expr, $span: expr) => {
        $crate::ast::Token::new(
            $crate::ast::TokenType::$kind($str.to_string().into_boxed_str()),
            $span,
        )
    };
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenType::Eof => write!(f, "Token ( {} line: {} )", self.kind, self.span.2),
            _ => write!(f, "Token ( {} span: {} )", self.kind, self.span),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Ident(Box<str>),
    String(Box<str>),
    Number(Box<str>),

    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

macro_rules! as_str {
    ($self:expr, $f:expr, [$($ty:ident$(($data:tt))? $(=> $printer:expr)?),* $(,)?]) => {
        match $self {
            $(
                Self::$ty$(($data))? => write!($f, "kind: {}, val: '{}'", stringify!($ty),$($printer)? $($data)?),
            )*
        }
    };

    (s $self:expr, [$($ty:ident$(($data:tt))? $(=> $printer:expr)?),* $(,)?]) => {
        match $self {
            $(
                Self::$ty$(($data))? => format!("{}", $($printer)? $($data)?),
            )*
        }
    };
}

impl TokenType {
    pub fn as_str(&self) -> String {
        as_str!(s self, [
                OpenParen => "(",
                CloseParen => ")",
                OpenBrace => "{",
                CloseBrace => "}",
                Comma => ",",
                Dot => ".",
                Minus => "-",
                Plus => "+",
                SemiColon => ";",
                Slash => "/",
                Star => "*",
                Bang => "!",
                BangEqual => "!=",
                Equal => "=",
                EqualEqual => "==",
                Greater => ">",
                GreaterEqual => ">=",
                Less => "<",
                LessEqual => "<=",
                Ident(data),
                String(data),
                Number(data),
                And => "and",
                Class => "class",
                Else => "else",
                False => "false",
                Fn => "fn",
                For => "for",
                If => "if",
                Nil => "nil",
                Or => "or",
                Return => "return",
                Super => "super",
                This => "this",
                True => "true",
                Var => "var",
                While => "while",
                Error => "Error",
                Eof => "<<EOF>>",
        ])
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        as_str!(
            self, f,
            [
                OpenParen => "(",
                CloseParen => ")",
                OpenBrace => "{",
                CloseBrace => "}",
                Comma => ",",
                Dot => ".",
                Minus => "-",
                Plus => "+",
                SemiColon => ";",
                Slash => "/",
                Star => "*",
                Bang => "!",
                BangEqual => "!=",
                Equal => "=",
                EqualEqual => "==",
                Greater => ">",
                GreaterEqual => ">=",
                Less => "<",
                LessEqual => "<=",
                Ident(data),
                String(data),
                Number(data),
                And => "and",
                Class => "class",
                Else => "else",
                False => "false",
                Fn => "fn",
                For => "for",
                If => "if",
                Nil => "nil",
                Or => "or",
                Return => "return",
                Super => "super",
                This => "this",
                True => "true",
                Var => "var",
                While => "while",
                Error => "Error",
                Eof => "<<EOF>>",
            ]
        )
    }
}
