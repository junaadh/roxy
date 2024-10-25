use crate::{chunks::Opcode, object::ObjRef, value::Value};

use super::{Parser, Precedence, TokenType};

pub type ParseFn<'vm> = fn(&mut Parser<'vm>);

#[derive(Clone, Copy, Default)]
pub struct ParseRule<'parse> {
    pub prefix: Option<ParseFn<'parse>>,
    pub infix: Option<ParseFn<'parse>>,
    pub precedence: Precedence,
}

impl<'parse> ParseRule<'parse> {
    pub fn get_rule(kind: TokenType) -> ParseRule {
        match kind {
            TokenType::OpenParen => ParseRule {
                prefix: Some(grouping),
                ..Default::default()
            },
            TokenType::Minus => ParseRule {
                prefix: Some(unary),
                infix: Some(binary),
                precedence: Precedence::Term,
            },
            TokenType::Plus => ParseRule {
                infix: Some(binary),
                precedence: Precedence::Term,
                ..Default::default()
            },
            TokenType::Slash => ParseRule {
                infix: Some(binary),
                precedence: Precedence::Factor,
                ..Default::default()
            },
            TokenType::Star => ParseRule {
                infix: Some(binary),
                precedence: Precedence::Factor,
                ..Default::default()
            },
            TokenType::Bang => ParseRule {
                prefix: Some(unary),
                ..Default::default()
            },
            TokenType::BangEqual | TokenType::EqualEqual => ParseRule {
                infix: Some(binary),
                precedence: Precedence::Equality,
                ..Default::default()
            },
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => ParseRule {
                infix: Some(binary),
                precedence: Precedence::Comparison,
                ..Default::default()
            },
            TokenType::String(_) => ParseRule {
                prefix: Some(string),
                ..Default::default()
            },
            TokenType::Number(_) => ParseRule {
                prefix: Some(number),
                ..Default::default()
            },
            TokenType::Nil | TokenType::True | TokenType::False => ParseRule {
                prefix: Some(literal),
                ..Default::default()
            },
            _ => ParseRule {
                ..Default::default()
            },
        }
    }
}

fn grouping(parser: &mut Parser<'_>) {
    parser.expression();
    parser.consume(TokenType::CloseParen, "Expect ')' after expression.");
}

fn binary(parser: &mut Parser<'_>) {
    let op = parser.previous.kind;

    let rule = ParseRule::get_rule(op);
    parser.parse_precedence(rule.precedence.next());

    match op {
        TokenType::BangEqual => parser.emit_bytes(Opcode::Equal, Opcode::Not),
        TokenType::EqualEqual => parser.emit_byte(Opcode::Equal),
        TokenType::Greater => parser.emit_byte(Opcode::Greater),
        TokenType::GreaterEqual => parser.emit_bytes(Opcode::Less, Opcode::Not),
        TokenType::Less => parser.emit_byte(Opcode::Less),
        TokenType::LessEqual => parser.emit_bytes(Opcode::Greater, Opcode::Not),
        TokenType::Plus => parser.emit_byte(Opcode::Add),
        TokenType::Minus => parser.emit_byte(Opcode::Subtract),
        TokenType::Star => parser.emit_byte(Opcode::Multiply),
        TokenType::Slash => parser.emit_byte(Opcode::Divide),
        _ => (),
    }
}

fn unary(parser: &mut Parser<'_>) {
    let operator = parser.previous.kind;

    parser.parse_precedence(Precedence::Unary);

    match operator {
        TokenType::Minus => parser.emit_byte(Opcode::Negate),
        TokenType::Bang => parser.emit_byte(Opcode::Not),
        _ => (),
    }
}

fn number(parser: &mut Parser<'_>) {
    let value = parser
        .previous
        .object()
        .map_err(|x| parser.error(&x.to_string()))
        .unwrap();
    parser.emit_constant(value);
}

fn literal(parser: &mut Parser<'_>) {
    match parser.previous.kind {
        TokenType::Nil => parser.emit_byte(Opcode::Nil),
        TokenType::True => parser.emit_byte(Opcode::True),
        TokenType::False => parser.emit_byte(Opcode::False),
        _ => (),
    }
}

fn string(parser: &mut Parser<'_>) {
    let string = parser.previous;
    if let TokenType::String(v) = string.kind {
        let s = Value::String(ObjRef::new(Box::into_raw(Box::new(v.to_string()))));
        parser.emit_constant(s);
    }
}
