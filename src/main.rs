mod parser;
use std::io::{self, Write};

fn main() {
    print!("Enter expression: ");
    io::stdout().flush().unwrap();

    let mut s = String::from("");
    io::stdin().read_line(&mut s).unwrap();

    let mut parser = parser::Parser::create(s.to_string());

    let try_exp = parser.parse_expression();

    if let Ok(exp) = try_exp {
        println!("sum: {}", exp.eval());
    }
    else if let Err(e) = try_exp {
        println!("{}", e);
    }
}
