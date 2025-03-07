mod cfg;
mod lex;

// grammar rules
// start_rule: expr
// expr: multi_expr + expr
// multi_expr: term
// term: NUMBER | ( expr )

fn main() {
    let s = "2 + 3";
    println!("=> run lexer ...");
    let lex_tokens = lex::lexer(s).expect("Failed to tokenize the string!");
    println!("lex tokens: {:?}", lex_tokens);
    cfg::parse(lex_tokens);
}
