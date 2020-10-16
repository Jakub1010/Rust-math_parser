pub mod ast;
pub mod parse_error;

use std::str::{self, FromStr};

pub type ParseResult = Result<ast::ExpressionPointer, parse_error::ParseError>;

pub struct Parser {
    input: Vec<u8>,
    position: usize,
}

impl Parser {
    pub fn create(mut input: String) -> Self {
        input.push('$');
        Parser { input: input.bytes().collect(), position: 0 }
    }

    fn skip_whitespace(&mut self) {
        while (self.input[self.position] as char).is_whitespace() {
            self.position += 1;
        }
    }

    fn look_ahead(&mut self) -> char {
        self.skip_whitespace();
        self.input[self.position] as char
    }

    pub fn parse_expression(&mut self) -> ParseResult {
        let e = self.parse_sum()?;
        if self.look_ahead() == '$' {
            return Ok(e);
        }
        Err(parse_error::ParseError { message: "parse_expression".to_string() })
    }

    pub fn parse_sum(&mut self) -> ParseResult {
        let mut e = self.parse_mult()?;
        let mut c = self.look_ahead();
        while c == '+' || c == '-' {
            self.position += 1;
            e = Box::new(ast::BinaryOp::create(c, e, self.parse_mult()?));
            c = self.look_ahead();
        }
        Ok(e)
    }

    pub fn parse_mult(&mut self) -> ParseResult {
        let mut e = self.parse_term()?;
        let mut c = self.look_ahead();
        while c == '*' || c == '/' {
            self.position += 1;
            e = Box::new(ast::BinaryOp::create(c, e, self.parse_term()?));
            c = self.look_ahead();
        }
        Ok(e)
    }

    pub fn parse_term(&mut self) -> ParseResult {
        let c = self.look_ahead();
        if c.is_digit(10) {
            return self.parse_constant();
        }
        else if c == '(' {
            return self.parse_bracket();
        }
        Err(parse_error::ParseError { message: "parse_term".to_string() })
    }

    pub fn parse_constant(&mut self) -> ParseResult {
        let r = self.position;
        while (self.input[self.position] as char).is_digit(10) || (self.input[self.position] as char) == '.' {
            self.position += 1;
        }

        // convert &[u8] to &str
        let number = f64::from_str(str::from_utf8(&self.input[r..self.position]).unwrap());
        if number.is_ok() {
            return Ok(Box::new(ast::Constant::create(number.unwrap())));
        }
        Err(parse_error::ParseError { message: "parse_constant".to_string() })
    }

    pub fn parse_bracket(&mut self) -> ParseResult {
        self.position += 1;
        let e = self.parse_sum()?;
        if self.look_ahead() == ')' {
            self.position += 1;
            return Ok(e);
        }
        Err(parse_error::ParseError { message: "parse_bracket".to_string() })
    }
}