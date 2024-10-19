use crate::types::Object;

use super::Token;

macro_rules! define_ast {
    ($($name: ident : $trait: ident [$($field:ident: $value: ty),*]),* $(,)?) => {
        pub trait ExprVisitor<R> {
            $(
                fn $trait(&self, expr: &$name) -> R;
            )*
        }

        #[derive(Debug, Clone)]
        pub enum Expr {
            $(
                $name(Box<$name>),
            )*
        }

        impl Expr {
            pub fn accept<R>(&self, visitor: &dyn ExprVisitor<R>) -> R {
                match self {
                    $(
                        Self::$name(expr) => visitor.$trait(expr),
                    )*
                }
            }
        }

        $(
            #[derive(Debug, Clone)]
            pub struct $name {
                $(
                    pub $field: $value,
                )*
            }

            impl $name {
                pub fn new($( $field: $value, )*) -> Box<Self> {
                    Box::new(Self{
                        $( $field, )*
                    })
                }
            }
        )*
    };
}

define_ast!(
    Binary : visit_binary [ left: Expr, operator: Token, right: Expr ],
    Grouping : visit_grouping [ expression: Expr ],
    Literal : visit_literal [ value: Object ],
    Unary : visit_unary [ operator: Token, right: Expr ]
);
