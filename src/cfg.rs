// use enum to define different grammar items

use core::fmt;

use crate::lex::LexToken;

#[derive(Debug, PartialEq)]
pub enum CfgTerm {
    NonTermExpr,
    NonTermMultiExpr,
    NonTermTermExpr,
    TermNumber(u32),
    TermPlus,
    TermMinus,
    TermLeftParens,
    TermRightParens,
}

impl fmt::Display for CfgTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonTermExpr => {
                write!(f, "NonTerm::")
            }
            Self::NonTermMultiExpr => {
                write!(f, "NonTermMulti::")
            }
            Self::NonTermTermExpr => {
                write!(f, "NonTermTerm::")
            }
            Self::TermPlus => {
                write!(f, "Term('+')")
            }
            Self::TermMinus => {
                write!(f, "Term('-')")
            }
            Self::TermNumber(n) => {
                write!(f, "Term({})", *n)
            }
            Self::TermLeftParens => {
                write!(f, "Term('(')")
            }
            Self::TermRightParens => {
                write!(f, "Term(')')")
            }
        }
    }
}

#[derive(PartialEq)]
pub struct ParseNode {
    current_node: CfgTerm,
    child_nodes: Vec<ParseNode>,
}

impl ParseNode {
    pub fn new(current_node: CfgTerm) -> ParseNode {
        ParseNode {
            current_node,
            child_nodes: Vec::new(),
        }
    }

    pub fn add_child_node(&mut self, child_node: ParseNode) {
        self.child_nodes.push(child_node);
    }
}

impl fmt::Display for ParseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}](child nodes: {})",
            self.current_node,
            self.child_nodes.len()
        )
    }
}

#[derive(Debug)]
pub(crate) enum ParseError {
    EndOfTokenError,
    InvalidTokenError(String),
    UnknownError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EndOfTokenError => write!(f, "Reached end of token stream!"),
            ParseError::InvalidTokenError(s) => write!(f, "Invalid token found: {}", s),
            ParseError::UnknownError => write!(f, "Unknown error during parsing!"),
        }
    }
}

/// check if we have reached EOF by inspecting the current position with the
/// number of tokens
fn get_next_token(
    tokens: &Vec<LexToken>,
    pos: usize,
) -> Result<(Option<&LexToken>, usize), ParseError> {
    if (pos + 1) >= tokens.len() {
        return Err(ParseError::EndOfTokenError);
    }
    let tok = tokens.get(pos + 1);
    println!("=> tok: {:?}", tok);
    return Ok((tok, pos + 1));
}

/// parsing top level expression
fn parse_expr(tokens: &Vec<LexToken>, pos: usize) -> Result<(ParseNode, usize), ParseError> {
    println!("=> parsing expr node at position {pos}");
    let (multi_expr_node, new_pos) = parse_multi_expr(&tokens, pos)?;
    let mut expr_node = ParseNode::new(CfgTerm::NonTermExpr);
    expr_node.add_child_node(multi_expr_node);

    // look for +
    println!("=> looking for +/- ... {new_pos}");
    // let tok = tokens.get(new_pos + 1);
    let (tok, next_pos) = get_next_token(&tokens, new_pos)?;
    println!("=> tok: {:?}", tok);
    match tok {
        Some(LexToken::Add(_)) => {
            expr_node.add_child_node(ParseNode::new(CfgTerm::TermPlus));
        }
        Some(LexToken::Subtract(_)) => {
            expr_node.add_child_node(ParseNode::new(CfgTerm::TermMinus));
        }
        _ => {
            return Err(ParseError::InvalidTokenError(String::from(
                "Expected +/- sign!",
            )));
        }
    }

    // look for expr
    println!("=> looking for + expr ...");
    let (tail_expr_node, tail_expr_pos) = parse_expr(&tokens, next_pos + 1)?;
    expr_node.add_child_node(tail_expr_node);

    Ok((expr_node, tail_expr_pos))
}

/// parsing expression involving multiply (or '*')
fn parse_multi_expr(tokens: &Vec<LexToken>, pos: usize) -> Result<(ParseNode, usize), ParseError> {
    println!("=> parsing multi_expr node at position {pos}");
    // call term
    let (term_node, new_pos) = parse_term(tokens, pos)?;
    let mut me_node = ParseNode::new(CfgTerm::NonTermMultiExpr);
    me_node.child_nodes.push(term_node);
    Ok((me_node, new_pos))
}

/// parsing term (either number or sub expr)
fn parse_term(tokens: &Vec<LexToken>, pos: usize) -> Result<(ParseNode, usize), ParseError> {
    println!("=> parsing term node at position {pos} ...");
    let tok = tokens.get(pos);
    match tok {
        Some(LexToken::LeftParen('(')) => {
            let mut term_node = ParseNode::new(CfgTerm::NonTermTermExpr);

            println!("(");
            let left_parens_node = ParseNode::new(CfgTerm::TermLeftParens);
            term_node.child_nodes.push(left_parens_node);
            let (expr_node, expr_pos) =
                parse_expr(&tokens, pos + 1).expect("Term expression failed!");
            term_node.child_nodes.push(expr_node);

            // close parens
            let close_parens_tok = tokens.get(expr_pos + 1).expect("Expected close parens");
            assert_eq!(*close_parens_tok, LexToken::RightParen(')'));

            let right_parens_node = ParseNode::new(CfgTerm::TermRightParens);
            term_node.child_nodes.push(right_parens_node);

            return Ok((term_node, expr_pos + 1));
        }
        Some(LexToken::Num(n)) => {
            println!("{}", *n);
            let pt_node = ParseNode::new(CfgTerm::TermNumber(*n));
            return Ok((pt_node, pos));
        }
        _ => {
            return Err(ParseError::InvalidTokenError(format!(
                "Error: invalid term token: {}",
                tok.unwrap()
            )));
        }
    }
}

/// Construct parse tree from the list of lex tokens
pub(crate) fn parse(tokens: Vec<LexToken>) -> Result<ParseNode, ParseError> {
    // start off with Exprs (start rule)
    match parse_expr(&tokens, 0) {
        Ok((parsed_node, _)) => {
            return Ok(parsed_node);
        }
        Err(ParseError::EndOfTokenError) => Err(ParseError::EndOfTokenError),
        _ => Err(ParseError::UnknownError),
    }
}

#[cfg(test)]
mod tests {
    use crate::cfg;
    use crate::cfg::{CfgTerm, ParseError};
    use crate::lex;

    #[test]
    fn test_parse_add_expr() {
        // This test asserts that the below expression has the correct set of
        // parse nodes, including the non-terminals and terminals.
        let s = "2 + 3";
        let lex_tokens = lex::lexer(s).expect("Failed to tokenize the string!");
        match cfg::parse(lex_tokens) {
            Ok(parsed_node) => {
                println!("parsed node: {}", parsed_node);
                assert_eq!(parsed_node.current_node, CfgTerm::NonTermExpr);
                assert_eq!(parsed_node.child_nodes.len(), 3);
                let left_child_node = parsed_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a terminal number node");
                assert_eq!(left_child_node.current_node, CfgTerm::NonTermMultiExpr);
                let left_child_num_node = left_child_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a terminal number node");
                assert_eq!(left_child_num_node.current_node, CfgTerm::TermNumber(2));

                // '+' bit
                let plus_child_node = parsed_node
                    .child_nodes
                    .get(1)
                    .expect("Expected a terminal + node");
                assert_eq!(plus_child_node.current_node, CfgTerm::TermPlus);

                // right side of plus, TermExpr -> TermMultiExpr -> TermNumber
                let right_child_node = parsed_node
                    .child_nodes
                    .get(2)
                    .expect("Expected a terminal number node");
                assert_eq!(right_child_node.current_node, CfgTerm::NonTermExpr);
                let right_child_me_node = right_child_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a Non terminal multi_expr node");
                assert_eq!(right_child_me_node.current_node, CfgTerm::NonTermMultiExpr);
                let right_child_num_node = right_child_me_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a terminal number node");
                assert_eq!(right_child_num_node.current_node, CfgTerm::TermNumber(3));
            }
            Err(ParseError::EndOfTokenError) => {
                // end of token list
            }
            _ => {
                println!("Unknown error!");
            }
        }
    }
}
