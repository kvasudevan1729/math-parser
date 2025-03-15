mod cfg;
mod lex;
use std::io::{self, Write};

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
    println!("=> run lexer ...");
    let lex_tokens = lex::lexer(s.trim()).expect("Failed to tokenize the string!");
    println!("lex tokens: {:?}", lex_tokens);
    match cfg::parse(lex_tokens) {
        Ok(parse_tree) => {
            println!("parse tree: {}", parse_tree);
        }
        Err(cfg::ParseError::EndOfTokenError) => {
            println!("End Of Tokens, parsed successfully!")
        }
        _ => {
            println!("Unknown error occurred!")
        }
    }

    Ok(())
}
