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

fn test_lexer(s: &str) {
    println!("=> parsing math expression: {s}");
    match lex_multi_digit::lexer(s) {
        Ok(my_lexer) => {
            println!("tokens: {:?}", my_lexer.get_tokens());
        }
        Err(e) => {
            println!("Error: {e}");
            panic!("Lexer failed to tokenise the string!");
        }
    }
}

fn main() -> io::Result<()> {
    let mut s = String::new();
    print!("Enter math expression to parse:\n>>");
    io::stdout().flush()?;
    io::stdin().read_line(&mut s)?;
    // test_lexer(&s);

    let mut math_parser = MathParser::new(s);
    match math_parser.set_lexer() {
        Ok(()) => {}
        Err(e) => {
            println!("Error: {e}");
            panic!("Lexer failed to tokenise the string!");
        }
    }
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
