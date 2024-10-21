use std::fmt;

use crate::{
    error::{Compile, RxError},
    value::Value,
    Res,
};

use super::Span;

#[derive(Debug, Clone, Copy, PartialOrd, Default)]
pub struct Token<'t> {
    pub kind: TokenType<'t>,
    pub span: Span,
}

impl<'t> Token<'t> {
    pub fn new(kind: TokenType<'t>, span: Span) -> Self {
        Token { kind, span }
    }

    pub fn object(self) -> Res<Value> {
        match self.kind {
            TokenType::True => Ok(Value::Bool(true)),
            TokenType::False => Ok(Value::Bool(false)),
            // TokenType::Nil => Ok(Value::Nil),
            // TokenType::String(s) => Ok(Value::String(s.into_string())),
            TokenType::Number(num) => match num.parse::<i64>() {
                Ok(v) => Ok(Value::Int(v)),
                Err(_) => match num.parse::<f64>() {
                    Ok(f) => Ok(Value::Float(f)),
                    Err(er) => Err(RxError::new(Compile::new(&er.to_string()))),
                },
            },
            _ => todo!(),
        }
    }

    pub fn line(&self) -> usize {
        self.span.2
    }

    pub fn lexeme(&self) -> &'t str {
        self.kind.as_str()
    }
}

#[macro_export]
macro_rules! token {
    ($kind:ident, $span: expr) => {
        $crate::compiler::Token::new($crate::compiler::TokenType::$kind, $span)
    };

    ($kind:ident, $str: expr, $span: expr) => {
        $crate::compiler::Token::new(
            $crate::compiler::TokenType::$kind($str.to_string().into_boxed_str()),
            $span,
        )
    };
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            TokenType::Eof => write!(f, "Token ( {} line: {} )", self.kind, self.span.2),
            _ => write!(f, "Token ( {} span: {} )", self.kind, self.span),
        }
    }
}

#[derive(Debug, PartialOrd, Clone, Copy, Default)]
pub enum TokenType<'str> {
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

    Ident(&'str str),
    String(&'str str),
    Number(&'str str),

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

    Error(&'str str),
    #[default]
    Eof,
}

impl PartialEq for TokenType<'_> {
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
                | (Self::Error(_), Self::Error(_))
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
                Self::$ty$(($data))? => $($printer)? $($data)?,
            )*
        }
    };
}

impl<'str> TokenType<'str> {
    pub fn as_str(&self) -> &'str str {
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
                Error(data),
                Eof => "<<EOF>>",
        ])
    }
}

impl fmt::Display for TokenType<'_> {
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
                Error(data),
                Eof => "<<EOF>>",
            ]
        )
    }
}
