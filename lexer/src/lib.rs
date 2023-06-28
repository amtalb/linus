use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Literals
    Symbol(String),
    Str(String),
    Num(f64),
    True,
    False,
    None,
    // Collections
    Seq,
    Hash,
    Group,
    Choice,
    // Operators
    Add,
    Subtract,
    Divide,
    Multiply,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    And,
    Or,
    Not,
    // Types
    TypeDecl(String),
    TypeDelim,
    // Variables/Functions
    Def,
    Assign,
    AnonFn,
    // Special Expressions
    Do,
    Let,
    If,
    Loop,
    // Blocks
    Indent,
    Dedent,
    LeftParen,
    RightParen,
    Appl,
    Newline,
    // Exception handling
    Try,
    Catch,
    Finally,
    Throw,
    // EOF
    EOF,
}

pub struct Lexer<'a> {
    stream: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    indented: bool,
}

impl Lexer<'_> {
    fn new(input: &String) -> Lexer {
        Lexer {
            stream: input.chars().peekable(),
            tokens: vec![],
            indented: false,
        }
    }

    fn split_tokens(&mut self) -> Result<(), &'static str> {
        while let Some(c) = self.stream.next() {
            match c {
                '\n' if self.stream.peek() == Some(&' ') || self.stream.peek() == Some(&'\t') => {
                    self.indented = true;
                    self.tokens.push(Token::Indent);
                }
                '\n' if self.stream.peek() != Some(&' ')
                    && self.stream.peek() != Some(&'\t')
                    && self.indented =>
                {
                    self.indented = false;
                    self.tokens.push(Token::Dedent);
                }
                '\n' => {
                    self.tokens.push(Token::Newline);
                }
                ' ' | '\t' | '\r' => {}
                '"' => {
                    let mut str_lexeme = String::new();
                    str_lexeme.push(c);
                    while let Some(&x) = self.stream.peek() {
                        match self.stream.next() {
                            Some(nxt) => str_lexeme.push(nxt),
                            None => break,
                        };
                        if x == '"' {
                            break;
                        }
                    }
                    self.tokens.push(Token::Str(str_lexeme));
                }
                '#' => {
                    while let Some(x) = self.stream.next() {
                        if x == '\n' {
                            break;
                        }
                    }
                }
                ':' => {
                    self.tokens.push(Token::TypeDelim);
                }
                '(' => {
                    self.tokens.push(Token::LeftParen);
                }
                ')' => {
                    self.tokens.push(Token::RightParen);
                }
                '$' => {
                    self.tokens.push(Token::Appl);
                }
                '\\' => {
                    self.tokens.push(Token::AnonFn);
                }
                '-' if self.stream.peek() == Some(&'>') => {
                    self.tokens.push(Token::Assign);
                    self.stream.next();
                }
                '+' => {
                    self.tokens.push(Token::Add);
                }
                '-' => {
                    self.tokens.push(Token::Subtract);
                }
                '/' => {
                    self.tokens.push(Token::Divide);
                }
                '*' => {
                    self.tokens.push(Token::Multiply);
                }
                '>' if self.stream.peek() == Some(&'=') => {
                    self.tokens.push(Token::GreaterThanOrEqual);
                }
                '<' if self.stream.peek() == Some(&'=') => {
                    self.tokens.push(Token::LessThanOrEqual);
                }
                '>' => {
                    self.tokens.push(Token::GreaterThan);
                }
                '<' => {
                    self.tokens.push(Token::LessThan);
                }
                '=' => {
                    self.tokens.push(Token::Equal);
                }
                '0'..='9' => {
                    let mut num_lexeme = String::new();
                    num_lexeme.push(c);

                    while let Some(&x) = self.stream.peek() {
                        if !x.is_digit(10) && x != '.' {
                            break;
                        }
                        match self.stream.next() {
                            Some(nxt) => num_lexeme.push(nxt),
                            None => break,
                        };
                    }
                    self.tokens
                        .push(Token::Num(num_lexeme.parse::<f64>().unwrap()));
                }
                _ => {
                    let mut lexeme = String::new();
                    lexeme.push(c);

                    while let Some(&x) = self.stream.peek() {
                        if x.is_whitespace() || x == '#' || x == ':' || x == ')' {
                            break;
                        }
                        match self.stream.next() {
                            Some(nxt) => lexeme.push(nxt),
                            None => break,
                        };
                    }
                    match lexeme.as_str() {
                        "true" => self.tokens.push(Token::True),
                        "false" => self.tokens.push(Token::False),
                        "none" => self.tokens.push(Token::None),
                        "and" => self.tokens.push(Token::And),
                        "or" => self.tokens.push(Token::Or),
                        "not" => self.tokens.push(Token::Not),
                        "def" => self.tokens.push(Token::Def),
                        "let" => self.tokens.push(Token::Let),
                        "try" => self.tokens.push(Token::Try),
                        "catch" => self.tokens.push(Token::Catch),
                        "finally" => self.tokens.push(Token::Finally),
                        "throw" => self.tokens.push(Token::Throw),
                        "loop" => self.tokens.push(Token::Loop),
                        "do" => self.tokens.push(Token::Do),
                        "num" | "str" | "_" | "bool" => self.tokens.push(Token::TypeDecl(lexeme)),
                        _ => self.tokens.push(Token::Symbol(lexeme)),
                    }
                }
            }
        }
        self.tokens.push(Token::EOF);
        Ok(())
    }
}

pub fn lex(source: String) -> Result<Vec<Token>, &'static str> {
    let mut lexer = Lexer::new(&source);
    lexer.split_tokens()?;
    Ok(lexer.tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_assignment() -> Result<(), String> {
        let result = lex("def first_num: num -> 1".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_num_assignment"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_num_assignment"),
            &Token::Symbol("first_num".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_num_assignment"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_num_assignment"),
            &Token::TypeDecl("num".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_num_assignment"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_num_assignment"),
            &Token::Num(1.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );

        Ok(())
    }

    #[test]
    fn test_str_assignment() -> Result<(), String> {
        let result = lex("def test_string: str -> \"this is a test\"".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_str_assignment"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_str_assignment"),
            &Token::Symbol("test_string".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_str_assignment"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_str_assignment"),
            &Token::TypeDecl("str".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_str_assignment"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_str_assignment"),
            &Token::Str("\"this is a test\"".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_bool_assignment() -> Result<(), String> {
        let result = lex("def is_bool: bool -> true".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_bool_assignment"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_bool_assignment"),
            &Token::Symbol("is_bool".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_bool_assignment"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_bool_assignment"),
            &Token::TypeDecl("bool".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_bool_assignment"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_bool_assignment"),
            &Token::True
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_none_assignment() -> Result<(), String> {
        let result = lex("def is_none: _ ->none".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_none_assignment"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_none_assignment"),
            &Token::Symbol("is_none".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_none_assignment"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_none_assignment"),
            &Token::TypeDecl("_".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_none_assignment"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_none_assignment"),
            &Token::None
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );

        Ok(())
    }

    #[test]
    fn test_func_assignment() -> Result<(), String> {
        let result = lex("def no_args_func: _ -> print \"Hello, world!\"".to_string())?;

        println!("{:?}", result);

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_none_assignment"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::Symbol("no_args_func".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::TypeDecl("_".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::Symbol("print".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::Str("\"Hello, world!\"".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_func_call() -> Result<(), String> {
        let result = lex("print test_string".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_call"),
            &Token::Symbol("print".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_call"),
            &Token::Symbol("test_string".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_multiline_func_call() -> Result<(), String> {
        let result = lex("def sum: num\n    x: num\n    y: num ->\n    + x y".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::Symbol("sum".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::TypeDecl("num".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::Indent
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::Symbol("x".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::TypeDecl("num".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Indent
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::Symbol("y".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::TypeDecl("num".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Indent
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::Symbol("+".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::Symbol("x".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::Symbol("y".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );

        Ok(())
    }

    #[test]
    fn test_comment1() -> Result<(), String> {
        let result = lex("# this is a comment".to_string())?;

        // test token types and lexemes
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_comment2() -> Result<(), String> {
        let result = lex("# this # is # also # a comment".to_string())?;

        // test token types and lexemes
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_comment3() -> Result<(), String> {
        let result = lex("# this \"should also be a comment\"".to_string())?;

        // test token types and lexemes
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_func_call"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_inline_comment() -> Result<(), String> {
        let result = lex("symbol sym#comment".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_inline_comment"),
            &Token::Symbol("symbol".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_inline_comment"),
            &Token::Symbol("sym".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_string_with_comment_symbol() -> Result<(), String> {
        let result = lex("\"this is # not a comment\"".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_string_with_comment_symbol"),
            &Token::Str("\"this is # not a comment\"".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_long_function_def() -> Result<(), String> {
        let result = lex("def x: num -> + 1 / 2 * 3 - 4 5".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Symbol("x".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::TypeDecl("num".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Symbol("+".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Num(1.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Symbol("/".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Num(2.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Symbol("*".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Num(3.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Symbol("-".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Num(4.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_long_function_def"),
            &Token::Num(5.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );

        Ok(())
    }

    #[test]
    fn test_multiline_func_def() -> Result<(), String> {
        let result = lex(
            "def x: num -> sum 1\n    divide 2\n        multiply 3\n            subtract 4 5"
                .to_string(),
        )?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Symbol("x".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::TypeDecl("num".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Symbol("sum".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Num(1.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Indent
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Symbol("divide".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Num(2.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Indent
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Symbol("multiply".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Num(3.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Indent
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Symbol("subtract".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Num(4.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_multiline_function_def"),
            &Token::Num(5.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );
        Ok(())
    }

    #[test]
    fn test_two_func_decls() -> Result<(), String> {
        let result = lex(
            "def func_1: _ ->\n    print \"Hello, world!\"\ndef func_2: num ->\n    + 2 2"
                .to_string(),
        )?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("func_1".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::TypeDecl("_".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Indent
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("print".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Str("\"Hello, world!\"".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Dedent
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("def".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("func_2".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::TypeDelim
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::TypeDecl("num".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Assign
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Indent
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("+".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(2.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(2.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_func_assignment"),
            &Token::EOF
        );

        Ok(())
    }

    #[test]
    fn test_appl() -> Result<(), String> {
        let result = lex("plus 1 2 $ divide 3 4".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("plus".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(1.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(2.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Appl
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("divide".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(3.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(4.0)
        );

        Ok(())
    }

    #[test]
    fn test_paren_expr() -> Result<(), String> {
        let result = lex("plus 1 2 (divide 3 4)".to_string())?;

        // test token types
        let mut result_iter = result.iter();
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("plus".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(1.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(2.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::LeftParen
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Symbol("divide".to_string())
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(3.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::Num(4.0)
        );
        assert_eq!(
            result_iter
                .next()
                .expect("Error reading test: test_two_func_decls"),
            &Token::RightParen
        );

        Ok(())
    }
}
