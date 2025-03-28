use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LexToken {
    Num(u32),
    Add(char),
    Subtract(char),
    Div(char),
    Multi(char),
    LeftParen(char),
    RightParen(char),
    Newline,
}

impl fmt::Display for LexToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LexToken::Num(n) => {
                write!(f, "{}", n)
            }
            LexToken::Add(c) | LexToken::Subtract(c) | LexToken::Div(c) | LexToken::Multi(c) => {
                write!(f, " {} ", c)
            }
            LexToken::LeftParen(c) | LexToken::RightParen(c) => {
                write!(f, "{}", c)
            }
            LexToken::Newline => {
                write!(f, "{}", "NL")
            }
        }
    }
}

/// Handles single digit only, converts a char to a single digit
fn get_number_from_char(c: char) -> u32 {
    let n = c.to_string().parse::<u32>().expect("Expected a digit!");
    return n;
}

/// Takes an input string, parses and returns a result containing
/// a vector of lex tokens. On error, returns an error message
pub(crate) fn lexer(s: &str) -> Result<Vec<LexToken>, String> {
    let mut tokens: Vec<LexToken> = Vec::new();

    let mut tok_list = s.chars().peekable();
    while let Some(&c) = tok_list.peek() {
        match c {
            '0'..='9' => {
                tok_list.next();
                let n = get_number_from_char(c);
                tokens.push(LexToken::Num(n));
                println!("number: {}", c);
            }
            '+' => {
                println!("plus: {}", c);
                tokens.push(LexToken::Add(c));
                tok_list.next();
            }
            '-' => {
                println!("minus: {}", c);
                tokens.push(LexToken::Subtract(c));
                tok_list.next();
            }
            '/' => {
                println!("div: {}", c);
                tokens.push(LexToken::Div(c));
                tok_list.next();
            }
            '*' => {
                println!("multi: {}", c);
                tokens.push(LexToken::Multi(c));
                tok_list.next();
            }
            '(' => {
                println!("left bracket: {}", c);
                tokens.push(LexToken::LeftParen(c));
                tok_list.next();
            }
            ')' => {
                println!("right bracket: {}", c);
                tokens.push(LexToken::RightParen(c));
                tok_list.next();
            }
            ' ' => {
                // ignore blank spaces
                tok_list.next();
            }
            _ => {
                println!("=> invalid character found: {}", c);
                return Err(format!("Invalid character found: {}", c));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_div_expr() {
        let s = "(2/3)";
        let tokens = lexer(s).unwrap();
        assert_eq!(tokens[0], LexToken::LeftParen('('));
        assert_eq!(tokens[1], LexToken::Num(2));
        assert_eq!(tokens[2], LexToken::Div('/'));
        assert_eq!(tokens[3], LexToken::Num(3));
        assert_eq!(tokens[4], LexToken::RightParen(')'));
    }

    #[test]
    fn test_lexer_add_expr() {
        let s = "(2+3)";
        let tokens = lexer(s).unwrap();
        assert_eq!(tokens[0], LexToken::LeftParen('('));
        assert_eq!(tokens[1], LexToken::Num(2));
        assert_eq!(tokens[2], LexToken::Add('+'));
        assert_eq!(tokens[3], LexToken::Num(3));
        assert_eq!(tokens[4], LexToken::RightParen(')'));
    }

    #[test]
    fn test_lexer_subtraction_expr() {
        let s = "(2-3)";
        let tokens = lexer(s).unwrap();
        assert_eq!(tokens[0], LexToken::LeftParen('('));
        assert_eq!(tokens[1], LexToken::Num(2));
        assert_eq!(tokens[2], LexToken::Subtract('-'));
        assert_eq!(tokens[3], LexToken::Num(3));
        assert_eq!(tokens[4], LexToken::RightParen(')'));
    }

    #[test]
    fn test_lexer_div_multi_add_subtraction_expr() {
        let s = "(2/3)*4+5-6";
        let tokens = lexer(s).unwrap();
        assert_eq!(tokens[0], LexToken::LeftParen('('));
        assert_eq!(tokens[1], LexToken::Num(2));
        assert_eq!(tokens[2], LexToken::Div('/'));
        assert_eq!(tokens[3], LexToken::Num(3));
        assert_eq!(tokens[4], LexToken::RightParen(')'));
        assert_eq!(tokens[5], LexToken::Multi('*'));
        assert_eq!(tokens[6], LexToken::Num(4));
        assert_eq!(tokens[7], LexToken::Add('+'));
        assert_eq!(tokens[8], LexToken::Num(5));
        assert_eq!(tokens[9], LexToken::Subtract('-'));
        assert_eq!(tokens[10], LexToken::Num(6));
    }
}
