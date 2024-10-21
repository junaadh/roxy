use crate::chunks::Opcode;

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
            TokenType::Number(_) => ParseRule {
                prefix: Some(number),
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
        TokenType::Bang => todo!(),
        _ => (),
    }
}

fn number(parser: &mut Parser<'_>) {
    let value = parser
        .previous
        .object()
        .map_err(|x| parser.error(&x.to_string()))
        .unwrap();
    let idx = parser.chunk.add_constant(value);
    parser.emit_byte(Opcode::Constant(idx));
}