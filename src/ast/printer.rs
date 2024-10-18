use crate::types::Object;

use super::{Expr, ExprVisitor};

pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &super::Binary) -> String {
        self.parenthesize(&expr.operator.kind.as_str(), vec![&expr.left, &expr.right])
    }

    fn visit_grouping(&self, expr: &super::Grouping) -> String {
        self.parenthesize("group", vec![&expr.expression])
    }

    fn visit_literal(&self, expr: &super::Literal) -> String {
        match &expr.value {
            Object::Bool(true) => "true".to_owned(),
            Object::Bool(false) => "false".to_owned(),
            Object::Int(s) => format!("{s}"),
            Object::Float(s) => format!("{s}"),
            Object::String(s) => s.clone(),
            Object::Nil => "nil".to_owned(),
        }
    }

    fn visit_unary(&self, expr: &super::Unary) -> String {
        self.parenthesize(&expr.operator.kind.as_str(), vec![&expr.right])
    }
}

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut sb = String::new();

        sb.push('(');
        sb.push_str(name);
        for expr in exprs {
            sb.push_str(&format!(" {}", expr.accept(self)))
        }
        sb.push(')');

        sb
    }

    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
}
