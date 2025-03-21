mod cfg;
mod lex;
use std::io::{self, Write};

use cfg::mathparser::MathParser;

// grammar rules
// start_rule: expr
// expr: multi_div_expr + expr | multi_div_expr '-' expr
// multi_div_expr: div_expr * multi_div_expr | div_expr
// div_expr: term / div_expr | term
// term: NUMBER | ( expr )

fn main() -> io::Result<()> {
    let mut s = String::new();
    print!("Enter math expression to parse:\n>>");
    io::stdout().flush()?;
    io::stdin().read_line(&mut s)?;
    println!("=> parsing expression: {s}");
    let mut math_parser = MathParser::new(String::from(s));
    let _ = math_parser.start();
    match math_parser.parsed_node {
        Some(parse_node) => {
            println!("\nparse node:\n\n{}", parse_node);
        }
        _ => {
            println!("Unknown error occurred!")
        }
    }

    Ok(())
}
