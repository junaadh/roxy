use super::Expr;

macro_rules! define_ast {
    ($($name: ident : $trait: ident [$($field:ident: $value: ty),*]),* $(,)?) => {
        pub trait StmtVisitor<R> {
            $(
                fn $trait(&self, expr: &$name) -> R;
            )*
        }

        #[derive(Debug, Clone)]
        pub enum Stmt {
            $(
                $name(Box<$name>),
            )*
        }

        impl Stmt {
            pub fn accept<R>(&self, visitor: &dyn StmtVisitor<R>) -> R {
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
    Expression : visit_expression [ expression: Expr ],
    Function   : visit_function   [ expression: Expr ],
);
