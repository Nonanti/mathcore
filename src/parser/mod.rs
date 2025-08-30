use crate::types::{BinaryOp, Expr, MathError, UnaryOp};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0},
    combinator::{map, opt, recognize},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use num_complex::Complex64;

// parser struct for math expressions
pub struct Parser;

impl Parser {
    /// Parse math expression string
    pub fn parse(input: &str) -> Result<Expr, MathError> {
        let input = input.trim();
        match expression(input) {
            Ok((remaining, expr)) => {
                if remaining.trim().is_empty() {
                    Ok(expr)
                } else {
                    Err(MathError::ParseError(format!(
                        "Unexpected input: '{}'",
                        remaining
                    )))
                }
            }
            Err(e) => Err(MathError::ParseError(format!("Parse error: {:?}", e))), // TODO: better error messages
        }
    }
}

fn expression(input: &str) -> IResult<&str, Expr> {
    additive(input)
}

fn additive(input: &str) -> IResult<&str, Expr> {
    let (input, initial) = multiplicative(input)?;

    let (input, operations) = many0(tuple((
        delimited(multispace0, alt((char('+'), char('-'))), multispace0),
        multiplicative,
    )))(input)?;

    Ok((
        input,
        operations.into_iter().fold(initial, |acc, (op, expr)| {
            let op = match op {
                '+' => BinaryOp::Add,
                '-' => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            Expr::Binary {
                op,
                left: Box::new(acc),
                right: Box::new(expr),
            }
        }),
    ))
}

fn multiplicative(input: &str) -> IResult<&str, Expr> {
    let (input, initial) = power(input)?;

    let (input, operations) = many0(tuple((
        delimited(
            multispace0,
            alt((char('*'), char('/'), char('%'))),
            multispace0,
        ),
        power,
    )))(input)?;

    Ok((
        input,
        operations.into_iter().fold(initial, |acc, (op, expr)| {
            let op = match op {
                '*' => BinaryOp::Multiply,
                '/' => BinaryOp::Divide,
                '%' => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            Expr::Binary {
                op,
                left: Box::new(acc),
                right: Box::new(expr),
            }
        }),
    ))
}

fn power(input: &str) -> IResult<&str, Expr> {
    let (input, base) = postfix(input)?;

    if let Ok((input, _)) = delimited::<_, _, _, _, nom::error::Error<&str>, _, _, _>(
        multispace0,
        char('^'),
        multispace0,
    )(input)
    {
        let (input, exponent) = power(input)?;
        Ok((
            input,
            Expr::Binary {
                op: BinaryOp::Power,
                left: Box::new(base),
                right: Box::new(exponent),
            },
        ))
    } else {
        Ok((input, base))
    }
}

fn postfix(input: &str) -> IResult<&str, Expr> {
    let (input, expr) = primary(input)?;
    let (input, _) = multispace0(input)?;

    if let Ok((input, _)) = char::<_, nom::error::Error<&str>>('!')(input) {
        Ok((
            input,
            Expr::Unary {
                op: UnaryOp::Factorial,
                expr: Box::new(expr),
            },
        ))
    } else {
        Ok((input, expr))
    }
}

fn primary(input: &str) -> IResult<&str, Expr> {
    let (input, _) = multispace0(input)?;

    alt((
        complex_number,
        number,
        function_call,
        symbol,
        parenthesized,
        absolute_value,
        unary_minus,
    ))(input)
}

fn parenthesized(input: &str) -> IResult<&str, Expr> {
    delimited(
        char('('),
        delimited(multispace0, expression, multispace0),
        char(')'),
    )(input)
}

fn absolute_value(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            char('|'),
            delimited(multispace0, expression, multispace0),
            char('|'),
        ),
        |expr| Expr::Unary {
            op: UnaryOp::Abs,
            expr: Box::new(expr),
        },
    )(input)
}

fn unary_minus(input: &str) -> IResult<&str, Expr> {
    map(preceded(char('-'), primary), |expr| Expr::Unary {
        op: UnaryOp::Negate,
        expr: Box::new(expr),
    })(input)
}

fn number(input: &str) -> IResult<&str, Expr> {
    map(
        recognize(tuple((
            opt(char('-')),
            digit1,
            opt(pair(char('.'), digit1)),
            opt(tuple((
                alt((char('e'), char('E'))),
                opt(alt((char('+'), char('-')))),
                digit1,
            ))),
        ))),
        |s: &str| Expr::Number(s.parse().unwrap()),
    )(input)
}

fn complex_number(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((number, alt((char('+'), char('-'))), number, char('i'))),
        |(real, sign, imag, _)| {
            if let (Expr::Number(r), Expr::Number(i)) = (real, imag) {
                let i = if sign == '-' { -i } else { i };
                Expr::Complex(Complex64::new(r, i))
            } else {
                unreachable!()
            }
        },
    )(input)
}

fn symbol(input: &str) -> IResult<&str, Expr> {
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s: &str| Expr::Symbol(s.to_string()),
    )(input)
}

fn function_call(input: &str) -> IResult<&str, Expr> {
    let (input, name) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)?;

    let (input, args) = delimited(
        char('('),
        separated_list0(delimited(multispace0, char(','), multispace0), expression),
        char(')'),
    )(input)?;

    Ok((
        input,
        Expr::Function {
            name: name.to_string(),
            args,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let expr = Parser::parse("42").unwrap();
        assert!(matches!(expr, Expr::Number(n) if n == 42.0));

        let expr = Parser::parse("3.14").unwrap();
        assert!(matches!(expr, Expr::Number(n) if (n - 3.14).abs() < 0.001));

        let expr = Parser::parse("-2.5e-3").unwrap();
        assert!(matches!(expr, Expr::Number(n) if (n + 0.0025).abs() < 0.00001));
    }

    #[test]
    fn test_parse_symbol() {
        let expr = Parser::parse("x").unwrap();
        assert!(matches!(expr, Expr::Symbol(s) if s == "x"));

        let expr = Parser::parse("var_123").unwrap();
        assert!(matches!(expr, Expr::Symbol(s) if s == "var_123"));
    }

    #[test]
    fn test_parse_binary_ops() {
        let expr = Parser::parse("2 + 3").unwrap();
        assert!(matches!(
            expr,
            Expr::Binary {
                op: BinaryOp::Add,
                ..
            }
        ));

        let expr = Parser::parse("5 * x").unwrap();
        assert!(matches!(
            expr,
            Expr::Binary {
                op: BinaryOp::Multiply,
                ..
            }
        ));

        let expr = Parser::parse("x^2").unwrap();
        assert!(matches!(
            expr,
            Expr::Binary {
                op: BinaryOp::Power,
                ..
            }
        ));
    }

    #[test]
    fn test_parse_function() {
        let expr = Parser::parse("sin(x)").unwrap();
        if let Expr::Function { name, args } = expr {
            assert_eq!(name, "sin");
            assert_eq!(args.len(), 1);
        } else {
            panic!("Expected function");
        }

        let expr = Parser::parse("max(a, b, c)").unwrap();
        if let Expr::Function { name, args } = expr {
            assert_eq!(name, "max");
            assert_eq!(args.len(), 3);
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_parse_complex() {
        let expr = Parser::parse("3+4i").unwrap();
        if let Expr::Complex(c) = expr {
            assert_eq!(c.re, 3.0);
            assert_eq!(c.im, 4.0);
        } else {
            panic!("Expected complex number");
        }
    }

    #[test]
    fn test_precedence() {
        let expr = Parser::parse("2 + 3 * 4").unwrap();
        println!("{}", expr);

        let expr = Parser::parse("2 * 3 ^ 4").unwrap();
        println!("{}", expr);
    }
}
