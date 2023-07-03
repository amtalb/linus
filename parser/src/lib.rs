use lexer::Token;
use std::cell::Cell;

#[derive(Debug)]
pub enum Expr {
    Assignment {
        name: String,
        type_decl: String,
        expr: Box<Expr>,
    },
    Literal {
        token: Token,
    },
    FunctionCall {
        operator: Token,
        operand: Vec<Expr>,
    },
    Operator {
        token: Token,
    },
    Variable {
        name: Token,
    },
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    idx: Cell<usize>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Parser {
        Parser {
            tokens: tokens,
            idx: Cell::new(0),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut exprs: Vec<Expr> = vec![];
        let mut errs: Vec<&'static str> = vec![];

        while let Some(tok) = self.peek() {
            if tok == &Token::Newline {
                self.advance();
            } else {
                match self.declaration() {
                    Ok(expr) => exprs.push(expr),
                    Err(err) => {
                        errs.push(err);
                        //self.synchronize();
                    }
                }
            }
        }

        if !errs.is_empty() {
            Err(errs.join("\n"))
        } else {
            Ok(exprs)
        }
    }

    fn declaration(&self) -> Result<Expr, &'static str> {
        match self.peek() {
            Some(&Token::Def) => {
                // advance past def
                self.advance();
                // match on variable name
                match self.advance() {
                    Some(Token::Symbol(token)) => {
                        // match on type delimiter ':', type, and assignment symbol '->'
                        match (self.advance(), self.advance(), self.peek()) {
                            (
                                Some(&Token::TypeDelim),
                                Some(&Token::TypeDecl(ref type_declaration)),
                                Some(&Token::Assign)
                            ) => {
                                // advance past assignment symbol
                                self.advance();
                                // match on expression following assignment symbol
                                match (self.expression()?, self.peek()) {
                                    (e, Some(&Token::Indent))
                                    | (e, Some(&Token::LeftParen))
                                    | (e, Some(&Token::Appl)) 
                                    | (e, Some(&Token::Newline)) 
                                    | (e, Some(&Token::EOF)) => {
                                        self.advance();
                                        Ok(Expr::Assignment {
                                            name: token.clone(),
                                            type_decl: type_declaration.clone(),
                                            expr: Box::new(e),
                                        })
                                    }
                                    _ => Err("Error in global variable declaration: no expression following variable declaration."),
                                }
                            },
                            _ => Err("Error in global variable declaration: invalid syntax after \"def\""),
                        }
                    },
                    _ => Err("Invalid variable name."),
                }
            },
            _ => self.special_expression(),
        }
    }

    fn special_expression(&self) -> Result<Expr, &'static str> {
        match self.peek() {
            // Some(&Token::Let) => {
            //     self.advance();
            //     self.let_special_expr()
            // }
            // Some(&Token::If) => {
            //     self.advance();
            //     self.if_special_expr()
            // }
            // Some(&Token::Do) => {
            //     self.advance();
            //     self.do_special_expr()
            // }
            // Some(&Token::Loop) => {
            //     self.advance();
            //     self.loop_special_expr()
            // }
            // Some(&Token::Try) => {
            //     self.advance();
            //     self.try_special_expr()
            // }
            // Some(&Token::Catch) => {
            //     self.advance();
            //     self.catch_special_expr()
            // }
            _ => self.expression(),
        }
    }

    fn expression(&self) -> Result<Expr, &'static str> {
        self.function_call()
    }

    fn function_call(&self) -> Result<Expr, &'static str> {
        let mut expr = self.primary()?;
        
        loop {
            match self.peek() {
                Some(Token::Symbol(_))
                | Some(Token::Str(_))
                | Some(Token::Num(_))
                | Some(Token::True)
                | Some(Token::False)
                | Some(Token::None) 
                | Some(Token::Appl) 
                | Some(Token::Indent) 
                | Some(Token::LeftParen) => {
                    let mut operands: Vec<Expr> = Vec::new();
                    loop {
                        match self.peek() {
                            Some(Token::Appl) | Some(Token::LeftParen) | Some(Token::Indent) => {
                                self.advance();
                                operands.push(self.expression()?)
                            }
                            Some(Token::Subtract) 
                            | Some(Token::Add) 
                            | Some(Token::Multiply) 
                            | Some(Token::Divide) 
                            | Some(Token::GreaterThan) 
                            | Some(Token::LessThan) 
                            | Some(Token::GreaterThanOrEqual) 
                            | Some(Token::LessThanOrEqual)
                            | Some(Token::Equal)
                            | Some(Token::And) 
                            | Some(Token::Or)
                            | Some(Token::Not) => {
                                operands.push(self.expression()?)
                            }
                            Some(Token::RightParen)
                            | Some(Token::Dedent)
                            | Some(Token::EOF)
                            | Some(Token::Newline) => {
                                break;
                            }
                            _ => operands.push(self.primary()?),
                        }
                    }
                    let operator = match expr {
                        Expr::Variable{ name } => name,
                        Expr::Operator{ token } => token,
                        _ => return Err("Invalid function name"),
                    };
                    expr = Expr::FunctionCall {
                        operator: operator,
                        operand: operands,
                    }
                }
                Some(Token::RightParen) | Some(Token::Newline) | Some(Token::Dedent) => {self.advance(); break}
                _ => break,
            }
        }
        Ok(expr)
    }

    fn primary(&self) -> Result<Expr, &'static str> {
        if let Some(t) = self.advance() {
            match t {
                Token::Num(_)
                | Token::Str(_)
                | Token::EOF
                | Token::True
                | Token::False
                | Token::None => Ok(Expr::Literal { token: t.clone() }),
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
                | Token::Or
                | Token::Not => Ok(Expr::Operator { token: t.clone() }),
                Token::Symbol(_) => Ok(Expr::Variable { name: t.clone() }),
                Token::Appl => Err("Cannot pass an application symbol ($) there."),
                _ => Err("Problem parsing primary."),
            }
        } else {
            Err("Problem advancing parser.")
        }
    }

    fn advance(&self) -> Option<&Token> {
        let previous_index = self.idx.get();
        if previous_index >= self.tokens.len() {
            None
        } else {
            self.idx.set(previous_index + 1);
            Some(&self.tokens[previous_index])
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.idx.get() >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.idx.get()])
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Expr>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn ast_pretty_print(expr: &Expr) {
    match expr {
        Expr::Assignment { name, type_decl, expr } => {
            print!("( ");
            print!("def ");
            print!("{}: {} ", name, type_decl);
            ast_pretty_print(expr);
            print!(") ");
        }
        Expr::Literal { token } => print_literal(token),
        Expr::FunctionCall { operator, operand } => {
            print!("( ");
            print_literal(operator);
            for op in operand {
                ast_pretty_print(op);
            }
            print!(") ");
        }
        Expr::Variable { name } => print_literal(name),
        Expr::Operator { token } => print_literal(token),
    }
}

fn print_literal(token: &Token) {
    match token {
        Token::Str(string) => {
            print!("{} ", &string);
        }
        Token::Num(num) => {
            print!("{} ", num);
        }
        Token::True => {
            print!("true ");
        }
        Token::False => {
            print!("false ");
        }
        Token::None => {
            print!("none ");
        }
        Token::Add => {
            print!("+ ");
        }
        Token::Subtract => {
            print!("- ");
        }
        Token::Multiply => {
            print!("* ");
        }
        Token::Divide => {
            print!("/ ");
        }
        Token::GreaterThan => {
            print!("> ");
        }
        Token::LessThan => {
            print!("< ");
        }
        Token::GreaterThanOrEqual => {
            print!(">= ");
        }
        Token::LessThanOrEqual => {
            print!("<= ");
        }
        Token::Equal => {
            print!("= ");
        }
        Token::And => {
            print!("and ");
        }
        Token::Or => {
            print!("or ");
        }
        Token::Not => {
            print!("not ");
        }
        Token::Symbol(sym) => {
            print!("{} ", &sym)
        }
        Token::EOF => (),
        _ => panic!("Shouldn't be printing a literal here!"),
    }
}