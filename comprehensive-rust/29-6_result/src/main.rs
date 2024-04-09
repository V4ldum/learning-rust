use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
struct TokenizerError(String);

impl Display for TokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message: {}", self.0)
    }
}

impl Error for TokenizerError {}

#[derive(Debug)]
struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message: {}", self.0)
    }
}

impl Error for ParserError {}

/// An arithmetic operator.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

/// A token in the expression language.
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

/// An expression in the expression language.
#[derive(Debug, PartialEq)]
enum Expression {
    /// A reference to a variable.
    Var(String),
    /// A literal number.
    Number(u32),
    /// A binary operation.
    Operation(Box<Expression>, Op, Box<Expression>),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Result<Token, TokenizerError>> {
        let c = self.0.next()?;
        match c {
            '0'..='9' => {
                let mut num = String::from(c);
                while let Some(c @ '0'..='9') = self.0.peek() {
                    num.push(*c);
                    self.0.next();
                }
                Some(Ok(Token::Number(num)))
            }
            'a'..='z' => {
                let mut ident = String::from(c);
                while let Some(c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
                    ident.push(*c);
                    self.0.next();
                }
                Some(Ok(Token::Identifier(ident)))
            }
            '+' => Some(Ok(Token::Operator(Op::Add))),
            '-' => Some(Ok(Token::Operator(Op::Sub))),
            _ => Some(Err(TokenizerError(format!("Unexpected character {c}")))),
        }
    }
}

fn parse(input: &str) -> Result<Expression, Box<dyn Error>> {
    let mut tokens = tokenize(input);

    fn parse_expr(tokens: &mut Tokenizer) -> Result<Expression, Box<dyn Error>> {
        let tok = tokens.next();
        let tok = match tok {
            Some(Ok(tok)) => tok,
            Some(Err(err)) => return Err(Box::new(err)),
            _ => return Err(Box::new(ParserError("Unexpected end of input".into()))),
        };

        let expr = match tok {
            Token::Number(num) => {
                let v = num.parse().expect("Invalid 32-bit integer'");
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => return Err(Box::new(ParserError(format!("Unexpected token {tok:?}")))),
        };
        // Look ahead to parse a binary operation if present.
        match tokens.next() {
            None => Ok(expr),
            Some(Ok(Token::Operator(op))) => Ok(Expression::Operation(
                Box::new(expr),
                op,
                Box::new(parse_expr(tokens)?),
            )),
            Some(Ok(tok)) => Err(Box::new(ParserError(format!("Unexpected token {tok:?}")))),
            Some(Err(err)) => Err(Box::new(err)),
        }
    }

    parse_expr(&mut tokens)
}

fn main() {
    match parse("10+foo+20-30") {
        Ok(expr) => println!("{expr:?}"),
        Err(err) => eprintln!("{}", err)
    }
}