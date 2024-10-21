use std::str::Chars;

use super::{Span, Token, TokenType};

#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    source: &'a str,
    chars: Chars<'a>,
    start: usize,
    line: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Cursor {
        let chars = source.chars();
        Self {
            source,
            chars,
            start: 0,
            line: 1,
        }
    }

    // pub fn tokenize(&'a mut self) -> impl Iterator<Item = Token<'_>> + 'a {
    //     std::iter::from_fn(move || {
    //         let token = self.advance();
    //         if token.kind != TokenType::Eof {
    //             Some(token)
    //         } else {
    //             None
    //         }
    //     })
    // }

    pub fn advance(&mut self) -> Token<'a> {
        let start = self.start;
        let end = self.pos();
        let line = self.line;
        let kind = self.next();
        Token::new(kind, Span::new(start, end, line))
    }

    // Misc
    fn bump(&mut self) -> char {
        self.chars
            .next()
            .inspect(|x| {
                if *x == '\n' {
                    self.line += 1;
                }
            })
            .unwrap_or('\0')
    }

    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    fn pos(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn reset_ptr(&mut self) {
        self.start = self.pos();
    }

    fn content(&self) -> &'a str {
        let end = self.pos();
        &self.source[self.start..end]
    }

    fn eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn bump_while<F>(&mut self, cond: F)
    where
        F: Fn(char) -> bool,
    {
        while !self.eof() && cond(self.peek()) {
            self.bump();
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.eof() {
            return false;
        }

        if self.peek() != expected {
            false
        } else {
            self.bump();
            true
        }
    }

    // kind
    fn next(&mut self) -> TokenType<'a> {
        self.skip_whitespace();
        self.reset_ptr();

        let char = self.bump();

        match char {
            '(' => TokenType::OpenParen,
            ')' => TokenType::CloseParen,
            '{' => TokenType::OpenBrace,
            '}' => TokenType::CloseBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::SemiColon,
            '/' => TokenType::Slash,
            '*' => TokenType::Star,

            '!' => {
                if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }

            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.ident(),

            '\0' => TokenType::Eof,
            _ => TokenType::Error("Unexpected character."),
        }
    }

    // specific handlers
    fn skip_whitespace(&mut self) {
        self.bump_while(|x| matches!(x, ' ' | '\r' | '\n' | '\t'))
    }

    fn string(&mut self) -> TokenType<'a> {
        self.bump_while(|x| x != '"');

        if self.eof() {
            return TokenType::Error("Unterminated string");
        }

        self.bump();

        let end = self.pos() - 1;
        let content = &self.source[self.start + 1..end];

        TokenType::String(content)
    }

    fn number(&mut self) -> TokenType<'a> {
        self.bump_while(|x| x.is_ascii_digit());

        if self.match_char('.') {
            self.bump_while(|x| x.is_ascii_digit());
        }

        TokenType::Number(self.content())
    }

    fn ident(&mut self) -> TokenType<'a> {
        self.bump_while(|x| x.is_ascii_alphabetic());
        let symbol = self.content();

        match symbol {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fn" => TokenType::Fn,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Ident(symbol),
        }
    }
}
