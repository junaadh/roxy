use crate::{
    chunks::{Chunk, Opcode},
    value::Value,
};

use super::{Cursor, ParseRule, Precedence, Token, TokenType};

pub struct Parser<'src> {
    pub(super) cursor: Cursor<'src>,
    pub(super) current: Token<'src>,
    pub(super) previous: Token<'src>,

    pub(super) chunk: &'src mut Chunk,

    pub(super) had_error: bool,
    panic_mode: bool,
}

impl<'src> Parser<'src> {
    pub fn new(content: &'src str, chunk: &'src mut Chunk) -> Self {
        Self {
            cursor: Cursor::new(content),
            current: Token::default(),
            previous: Token::default(),

            chunk,

            had_error: false,
            panic_mode: false,
        }
    }

    // pub api
    pub fn compile(mut self) {
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression");

        self.emit_return();

        #[cfg(feature = "trace")]
        {
            if !self.had_error {
                let dis = crate::chunks::Disassembler::new(&self.chunk, None);
                dis.disassemble("code");
            }
        }
    }

    // main logic
    pub(super) fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.cursor.advance();

            if self.current.kind == TokenType::Error("") {
                self.error_at_current(self.current.lexeme());
            } else {
                break;
            }
        }
    }

    // parse expression
    pub(super) fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    pub(super) fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let rule = ParseRule::get_rule(self.previous.kind).prefix;
        if let Some(prefix) = rule {
            prefix(self)
        } else {
            self.error("Expect expression.");
            return;
        }

        while precedence <= ParseRule::get_rule(self.current.kind).precedence {
            self.advance();
            let infix = ParseRule::get_rule(self.previous.kind).infix;
            if let Some(infix_rule) = infix {
                infix_rule(self)
            }
        }
    }

    // misc
    pub(super) fn consume(&mut self, ty: TokenType, msg: &str) {
        if self.current.kind == ty {
            self.advance();
            return;
        }

        self.error_at_current(msg)
    }

    // emitters
    pub(super) fn emit_byte(&mut self, byte: Opcode) {
        self.chunk.write(byte, self.previous.span.2);
    }

    pub(super) fn emit_bytes(&mut self, byte1: Opcode, byte2: Opcode) {
        self.chunk.write(byte1, self.previous.span.2);
        self.chunk.write(byte2, self.previous.span.2);
    }

    pub(super) fn emit_return(&mut self) {
        self.emit_byte(Opcode::Return);
    }

    pub(super) fn emit_constant(&mut self, value: Value) {
        let idx = self.chunk.add_constant(value);
        self.emit_byte(Opcode::Constant(idx));
    }

    // expr
    // error
    pub(super) fn error_at_current(&mut self, msg: &str) {
        self.error_at(self.current, msg);
    }

    pub(super) fn error(&mut self, msg: &str) {
        self.error_at(self.previous, msg);
    }

    pub(super) fn error_at(&mut self, token: Token, msg: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        self.had_error = true;

        print!("[line {}] Error", token.line());
        match token.kind {
            TokenType::Eof => print!(" at end"),
            TokenType::Error(_) => {}
            x => print!(" at '{}'", x.as_str()),
        }
        println!(": {}\n", msg);
    }
}
