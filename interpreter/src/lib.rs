use environment::Environment;
use environment::Type;
use lexer::Token;
use parser::Expr;

pub struct Interpreter {
    environment: environment::Environment,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, exprs: &[Expr]) {
        for expr in exprs {
            match self.evaluate_expression(expr) {
                Ok(Type::Num(n)) => println!("{}", n),
                Ok(Type::Str(str)) => println!("{}", str),
                Ok(Type::Bool(bool)) => println!("{}", bool),
                Ok(Type::None) => (),
                Err(err) => panic!("{}", err),
                _ => println!("error"),
            }
        }
    }

    fn evaluate_expression(&mut self, expression: &Expr) -> Result<Type, &'static str> {
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
                    .map(|operand| self.evaluate_expression(operand))
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
                        .map(|operand| self.evaluate_expression(operand))
                        .collect::<Vec<_>>()[0]
                    {
                        Ok(Type::Bool(a)) => Ok(Type::Bool(!a)),
                        Ok(Type::None) => Ok(Type::Bool(true)),
                        Ok(Type::Num(_)) => Err("Cannot apply function 'not' to type num"),
                        Ok(Type::Str(_)) => Err("Cannot apply function 'not' to type str"),
                        _ => Err("Not enough arguments to function 'not'"),
                    }
                }
                _ => Err("Function does not exist"),
            },
            Expr::Assignment {
                name,
                type_decl,
                expr,
            } => {
                let val: Type;
                match self.evaluate_expression(expr) {
                    Ok(x) => val = x,
                    _ => return Err("Problem in assignment"),
                }
                self.environment.define(name.to_string(), val);
                Ok(Type::None)
            }
            Expr::Variable { name } => match name {
                Token::Symbol(name) => match self.environment.retrieve(name) {
                    Some(t) => Ok(t.clone()),
                    None => Err("Variable name not found"),
                },
                _ => Err("Invalid variable name"),
            },
            _ => Err("Invalid expression"),
        }
    }
}

pub fn interpret(exprs: &[Expr]) {
    let mut interpreter = Interpreter::new();
    interpreter.interpret(exprs)
}
