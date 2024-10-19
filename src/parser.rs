use crate::{
    ast::{Binary, Expr, Grouping, Literal, Token, TokenType as Ty, Unary},
    error::{Error, RxError},
    Res,
};

pub struct Parser {
    tokens: Vec<Token>,
    offset: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, offset: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(e) => Some(e),
            Err(err) => {
                println!("{err}");
                None
            }
        }
    }

    // main
    fn eof(&self) -> bool {
        self.peek().is_some() && self.peek().map(|x| x.kind == Ty::Eof).unwrap_or_default()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.offset)
    }

    fn prev(&self) -> Token {
        self.tokens.get(self.offset - 1).cloned().unwrap()
    }

    fn advance(&mut self) -> Token {
        if !self.eof() {
            self.offset += 1;
        }

        self.prev()
    }

    // misc
    fn check(&self, ty: &Ty) -> bool {
        if self.eof() {
            return false;
        }
        self.peek().map(|x| x.kind == *ty).unwrap_or_default()
    }

    fn match_ty(&mut self, types: &[Ty]) -> bool {
        for ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn error(&self, token: Token, msg: &str) -> RxError {
        RxError::Parse(Error::new(token, msg))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.eof() {
            if self.prev().kind == Ty::SemiColon
                || matches!(
                    self.peek().unwrap().kind,
                    Ty::Class | Ty::Fn | Ty::Var | Ty::For | Ty::While | Ty::Return
                )
            {
                break;
            }

            self.advance();
        }
    }

    fn consume(&mut self, ty: &Ty, msg: &str) -> Res<Token> {
        if self.check(ty) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek().cloned().unwrap_or_default(), msg))
    }

    // expr
    fn expression(&mut self) -> Res<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Res<Expr> {
        let mut expr = self.comparison()?;

        while self.match_ty(&[Ty::BangEqual, Ty::EqualEqual]) {
            let op = self.prev();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Res<Expr> {
        let mut expr = self.term()?;

        while self.match_ty(&[Ty::Greater, Ty::GreaterEqual, Ty::Less, Ty::LessEqual]) {
            let op = self.prev();
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Res<Expr> {
        let mut expr = self.factor()?;

        while self.match_ty(&[Ty::Minus, Ty::Plus]) {
            let op = self.prev();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Res<Expr> {
        let mut expr = self.unary()?;

        while self.match_ty(&[Ty::Slash, Ty::Star]) {
            let op = self.prev();
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Res<Expr> {
        if self.match_ty(&[Ty::Minus, Ty::Bang]) {
            let op = self.prev();
            let right = self.unary()?;
            Ok(Expr::Unary(Unary::new(op, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Res<Expr> {
        if self.match_ty(&[
            Ty::False,
            Ty::True,
            Ty::Nil,
            Ty::Number(String::new().into_boxed_str()),
            Ty::String(String::new().into_boxed_str()),
        ]) {
            return Ok(Expr::Literal(Literal::new(self.prev().object()?)));
        }

        if self.match_ty(&[Ty::OpenParen]) {
            let expr = self.expression()?;
            self.consume(&Ty::CloseParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Grouping::new(expr)));
        }

        let err = self.error(
            self.peek().cloned().unwrap_or_default(),
            "Expected expression.",
        );
        // self.synchronize();
        Err(err)
    }
}

// impl Drop for Parser {
//     fn drop(&mut self) {
//         println!("{:#?}", self);
//     }
// }
