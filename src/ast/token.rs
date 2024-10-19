use std::{f64, fmt};

use crate::{types::Object, Res};

use super::Span;

#[derive(Debug, Clone, PartialOrd, Default)]
pub struct Token {
    pub kind: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenType, span: Span) -> Self {
        Token { kind, span }
    }

    pub fn object(self) -> Res<Object> {
        match self.kind {
            TokenType::True => Ok(Object::Bool(true)),
            TokenType::False => Ok(Object::Bool(false)),
            TokenType::Nil => Ok(Object::Nil),
            TokenType::String(s) => Ok(Object::String(s.into_string())),
            TokenType::Number(num) => match num.parse::<usize>() {
                Ok(v) => Ok(Object::Int(v)),
                Err(_) => match num.parse::<f64>() {
                    Ok(f) => Ok(Object::Float(f)),
                    Err(er) => Err(er.into()),
                },
            },
            _ => todo!(),
        }
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

#[derive(Debug, PartialOrd, Clone, Default)]
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
    #[default]
    Eof,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::OpenParen, Self::OpenParen)
                | (Self::CloseParen, Self::CloseParen)
                | (Self::OpenBrace, Self::OpenBrace)
                | (Self::CloseBrace, Self::CloseBrace)
                | (Self::Comma, Self::Comma)
                | (Self::Dot, Self::Dot)
                | (Self::Minus, Self::Minus)
                | (Self::Plus, Self::Plus)
                | (Self::SemiColon, Self::SemiColon)
                | (Self::Slash, Self::Slash)
                | (Self::Star, Self::Star)
                | (Self::Bang, Self::Bang)
                | (Self::Equal, Self::Equal)
                | (Self::EqualEqual, Self::EqualEqual)
                | (Self::Greater, Self::Greater)
                | (Self::GreaterEqual, Self::GreaterEqual)
                | (Self::Less, Self::Less)
                | (Self::LessEqual, Self::LessEqual)
                | (Self::And, Self::And)
                | (Self::Class, Self::Class)
                | (Self::Else, Self::Else)
                | (Self::False, Self::False)
                | (Self::Fn, Self::Fn)
                | (Self::For, Self::For)
                | (Self::If, Self::If)
                | (Self::Nil, Self::Nil)
                | (Self::Or, Self::Or)
                | (Self::Return, Self::Return)
                | (Self::Super, Self::Super)
                | (Self::This, Self::This)
                | (Self::True, Self::True)
                | (Self::Var, Self::Var)
                | (Self::While, Self::While)
                | (Self::Error, Self::Error)
                | (Self::Eof, Self::Eof)
                | (Self::String(_), Self::String(_))
                | (Self::Ident(_), Self::Ident(_))
                | (Self::Number(_), Self::Number(_))
        )
    }
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
