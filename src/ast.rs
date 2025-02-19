use crate::expr::Expr;
use crate::tokens::Literal;

struct AstPrinter {}

impl AstPrinter {
    pub fn print(e: Expr) -> String {
        match e {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let formatted_left = AstPrinter::print(*left).to_owned();
                let formatted_right = AstPrinter::print(*right).to_owned();
                let operator_lexeme = operator.lexeme.to_owned();
                format!("({operator_lexeme} {formatted_left} {formatted_right})")
            }
            Expr::Unary { operator, right } => {
                let operator_lexeme = operator.lexeme.to_owned();
                let formatted_expr = AstPrinter::print(*right).to_owned();
                format!("({operator_lexeme} {formatted_expr})")
            }
            Expr::Grouping { expr } => {
                let formatted_expr = AstPrinter::print(*expr).to_owned();
                format!("(group {formatted_expr})")
            }
            Expr::Literal { value } => value.to_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::Expr,
        tokens::{Literal, Token, TokenType},
    };

    use super::AstPrinter;

    #[test]
    fn test_print() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: String::from("-"),
                    line: 1,
                    position_start: 0,
                    position_end: 0,
                    literal: None,
                },
                right: Box::new(Expr::Literal {
                    value: Literal::Number { value: 123.0 },
                }),
            }),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: String::from("*"),
                line: 1,
                position_start: 0,
                position_end: 0,
                literal: None,
            },
            right: {
                Box::new(Expr::Grouping {
                    expr: Box::new(Expr::Literal {
                        value: Literal::Number { value: 45.67 },
                    }),
                })
            },
        };

        let res = AstPrinter::print(expr);
        let expected = "(* (- 123) (group 45.67))";
        assert_eq!(res, expected);
    }
}
