use lexer::Token;
use parser::Expr;

#[derive(Debug)]
pub enum Type {
    Num(f64),
    Str(String),
    Bool(bool),
    None,
    Function(String),
}

pub fn interpret(exprs: &[Expr]) {
    for expr in exprs {
        match evaluate_expression(expr) {
            Ok(Type::Num(n)) => println!("{}", n),
            Ok(Type::Str(str)) => println!("{}", str),
            Ok(Type::Bool(bool)) => println!("{}", bool),
            Ok(Type::None) => (),
            Err(err) => panic!("{}", err),
            _ => println!("error"),
        }
    }
}

fn evaluate_expression(expression: &Expr) -> Result<Type, &'static str> {
    match expression {
        Expr::Literal { token } => match token {
            Token::Str(string) => Ok(Type::Str(string.clone())),
            Token::Num(num) => Ok(Type::Num(*num)),
            Token::True => Ok(Type::Bool(true)),
            Token::False => Ok(Type::Bool(false)),
            Token::None => Ok(Type::None),
            Token::Symbol(sym) => Ok(Type::Str(sym.clone())),
            Token::EOF => Ok(Type::None),
            _ => Err("Not a literal"),
        },
        Expr::FunctionCall { operator, operand } => match operator {
            Token::Add
            | Token::Subtract
            | Token::Multiply
            | Token::Divide
            | Token::GreaterThan
            | Token::LessThan
            | Token::GreaterThanOrEqual
            | Token::LessThanOrEqual
            | Token::Equal
            | Token::And
            | Token::Or => operand
                .iter()
                .map(|operand| evaluate_expression(operand))
                .reduce(|a, b| match (a, b) {
                    (Ok(Type::Num(a)), Ok(Type::Num(b))) => match operator {
                        Token::Add => Ok(Type::Num(a + b)),
                        Token::Subtract => Ok(Type::Num(a - b)),
                        Token::Multiply => Ok(Type::Num(a * b)),
                        Token::Divide => Ok(Type::Num(a / b)),
                        Token::GreaterThan => Ok(Type::Bool(a > b)),
                        Token::LessThan => Ok(Type::Bool(a < b)),
                        Token::GreaterThanOrEqual => Ok(Type::Bool(a >= b)),
                        Token::LessThanOrEqual => Ok(Type::Bool(a <= b)),
                        Token::Equal => Ok(Type::Bool(a == b)),
                        _ => Err("Unexpected operator"),
                    },
                    (Ok(Type::Bool(a)), Ok(Type::Bool(b))) => match operator {
                        Token::And => Ok(Type::Bool(a && b)),
                        Token::Or => Ok(Type::Bool(a || b)),
                        Token::Equal => Ok(Type::Bool(a == b)),
                        _ => Err("Unexpected operator"),
                    },
                    (Ok(Type::Bool(_)), Ok(Type::Num(_))) => Err("Cannot compare Bool and Num"),
                    (Ok(Type::Num(_)), Ok(Type::Bool(_))) => Err("Cannot compare Num and Bool"),
                    _ => Err("Runtime Error: something wrong with operands!"),
                })
                .unwrap(),
            Token::Not => {
                match &operand
                    .iter()
                    .map(|operand| evaluate_expression(operand))
                    .collect::<Vec<_>>()[0]
                {
                    Ok(Type::Bool(a)) => Ok(Type::Bool(!a)),
                    Ok(Type::None) => Ok(Type::Bool(true)),
                    Ok(Type::Num(_)) => Err("Cannot apply function 'not' to type num"),
                    Ok(Type::Str(_)) => Err("Cannot apply function 'not' to type str"),
                    _ => Err("Not enough arguments to function 'not'"),
                }
            }
            // .reduce(|a, _| match a {
            //     Ok(Type::Bool(a)) => match operator {
            //         Token::Not => Ok(Type::Bool(!a)),
            //         _ => Err("Unexpected operator"),
            //     },
            //     _ => Err("Can only apply 'not' to a bool"),
            // })
            // .unwrap(),
            _ => Err("Function does not exist"),
        },
        _ => Err("Invalid expression"),
    }
}
