use std::io::{self, Write};

use operator_precedence_parser::Expression;

fn main() {
    loop {
        print!(">> ");
        if let Err(err) = io::stdout().flush() {
            eprintln!("Error: {}", err);
        };

        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", err);
        };

        if input.trim() == "exit" {
            break;
        }

        let expr = match Expression::from_str(&input) {
            Ok(expr) => expr,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };

        match expr.eval() {
            Ok(result) => println!("{}", result),
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        }
    }
}
