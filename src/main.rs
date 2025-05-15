use std::{
    collections::HashMap,
    io::{self, Write},
};

use operator_precedence_parser::Expression;

fn main() {
    let mut map = HashMap::<char, f64>::new();
    loop {
        print!(">> ");
        if let Err(err) = io::stdout().flush() {
            eprintln!("Error: {}", err);
        };

        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", err);
        };

        let input = input.trim();

        if input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let expr = match Expression::from_str(input) {
            Ok(expr) => expr,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };

        if expr.is_assignment() {
            if let Err(err) = expr.assign(&mut map) {
                eprintln!("Error: {}", err);
            };
            continue;
        }

        match expr.eval(&map) {
            Ok(result) => println!("{}", result),
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        }
    }
}
