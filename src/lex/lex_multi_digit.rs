use super::simple::LexToken;
use std::error;
use std::num::ParseIntError;

#[derive(Debug)]
pub(crate) struct Lexer {
    s: String,
    input_chars: Vec<char>,
    tokens: Vec<LexToken>,
}

impl Lexer {
    pub(crate) fn new(s: &str) -> Self {
        Lexer {
            s: s.to_string(),
            input_chars: s.chars().collect(),
            tokens: vec![],
        }
    }

    pub(crate) fn get_tokens(&self) -> &[LexToken] {
        return self.tokens.as_slice();
    }

    // Parse the sequence of digits starting from `pos` and return a lex token.
    fn get_number(&mut self, pos: usize) -> Result<(LexToken, usize), ParseIntError> {
        println!("=> get number from position {pos}");
        let mut curr_pos = pos;
        let mut num_vec: Vec<char> = vec![];
        loop {
            if let Some(c) = self.input_chars.get(curr_pos) {
                match *c {
                    '0'..='9' => {
                        num_vec.push(*c);
                        curr_pos += 1;
                    }
                    _ => {
                        // not a number character
                        println!("* end of a number sequence, pos: {curr_pos} *");
                        break;
                    }
                }
            } else {
                // no input characters left, possibly end of input?
                println!("* no input character left, pos: {curr_pos} *");
                break;
            }

            if (curr_pos + 1) > self.input_chars.len() {
                println!("* eos: {curr_pos}");
                break;
            }
        }

        let num_vec_s: Vec<String> = num_vec.iter().map(|c| c.to_string()).collect();
        let num_s: String = num_vec_s.join("");
        let num: u32 = num_s.parse::<u32>()?;

        return Ok((LexToken::Num(num), curr_pos));
    }

    pub(crate) fn tokenise(&mut self) -> Result<(), Box<dyn error::Error>> {
        let mut next_pos = 0;
        println!("=> tokenising string {} from pos: {next_pos}", self.s);
        loop {
            if let Some(c) = self.input_chars.get(next_pos) {
                match *c {
                    '0'..='9' => {
                        let (n, pos) = self.get_number(next_pos)?;
                        println!("n: {n}");
                        self.tokens.push(n);
                        // get_number fn already has moved the pointer
                        next_pos = pos;
                        println!("next_pos: {next_pos}");
                    }
                    '+' => {
                        self.tokens.push(LexToken::Add('+'));
                        next_pos += 1;
                    }
                    '-' => {
                        self.tokens.push(LexToken::Subtract('-'));
                        next_pos += 1;
                    }
                    '*' => {
                        self.tokens.push(LexToken::Multi('*'));
                        next_pos += 1;
                    }
                    '/' => {
                        self.tokens.push(LexToken::Div('/'));
                        next_pos += 1;
                    }
                    '(' => {
                        self.tokens.push(LexToken::LeftParen('('));
                        next_pos += 1;
                    }
                    ')' => {
                        self.tokens.push(LexToken::RightParen(')'));
                        next_pos += 1;
                    }
                    c if c.is_whitespace() => {
                        println!("whitespace -- ignore");
                        next_pos += 1;
                    }
                    '\n' | '\r' => {
                        println!("newline");
                        self.tokens.push(LexToken::Newline);
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            if next_pos >= self.input_chars.len() {
                break;
            }
        }

        Ok(())
    }
}

pub(crate) fn lexer(s: &str) -> Result<Lexer, Box<dyn error::Error>> {
    let mut my_lexer = Lexer::new(s);
    let _ = my_lexer.tokenise()?;

    return Ok(my_lexer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_add_expr() {
        let s = "(12+34)";
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        assert_eq!(tokens[0], LexToken::LeftParen('('));
        assert_eq!(tokens[1], LexToken::Num(12));
        assert_eq!(tokens[2], LexToken::Add('+'));
        assert_eq!(tokens[3], LexToken::Num(34));
        assert_eq!(tokens[4], LexToken::RightParen(')'));
    }

    #[test]
    fn test_lexer_subtract_expr() {
        let s = "(12 -345)";
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        assert_eq!(tokens[0], LexToken::LeftParen('('));
        assert_eq!(tokens[1], LexToken::Num(12));
        assert_eq!(tokens[2], LexToken::Subtract('-'));
        assert_eq!(tokens[3], LexToken::Num(345));
        assert_eq!(tokens[4], LexToken::RightParen(')'));
    }

    #[test]
    fn test_lexer_multi_op_expr() {
        let s = "12 -345* (555 / 678) ";
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        assert_eq!(tokens[0], LexToken::Num(12));
        assert_eq!(tokens[1], LexToken::Subtract('-'));
        assert_eq!(tokens[2], LexToken::Num(345));
        assert_eq!(tokens[3], LexToken::Multi('*'));
        assert_eq!(tokens[4], LexToken::LeftParen('('));
        assert_eq!(tokens[5], LexToken::Num(555));
        assert_eq!(tokens[6], LexToken::Div('/'));
        assert_eq!(tokens[7], LexToken::Num(678));
        assert_eq!(tokens[8], LexToken::RightParen(')'));
    }
}
