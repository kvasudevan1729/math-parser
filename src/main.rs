mod cfg;
mod lex;
use std::io::{self, Write};

use cfg::mathparser::MathParser;
use lex::lex_multi_digit;

// grammar rules
// start_rule: expr
// expr: multi_div_expr + expr | multi_div_expr '-' expr | multi_div_expr
// multi_div_expr: div_expr * multi_div_expr | div_expr
// div_expr: term / div_expr | term
// term: NUMBER | ( expr )

fn main() -> io::Result<()> {
    let mut s = String::new();
    print!("Enter math expression to parse:\n>>");
    io::stdout().flush()?;
    io::stdin().read_line(&mut s)?;

    let _lexer =
        lex_multi_digit::lexer(s.as_str()).expect("Failed to create a lexer with input {s}");
    let lex_tokens = _lexer.get_tokens();
    println!("lex tokens: {:?}", lex_tokens);
    let mut math_parser = MathParser::new(lex_tokens);
    let _ = math_parser.parse();
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
