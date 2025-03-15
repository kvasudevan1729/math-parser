// Create a parse tree from the math expression

use core::fmt;

use crate::lex::LexToken;

#[derive(Debug, PartialEq)]
pub enum CfgTerm {
    NonTermExpr,
    NonTermMultiDivExpr,
    NonTermDivExpr,
    NonTermTermExpr,
    TermNumber(u32),
    TermDivide,
    TermMultiply,
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
            Self::NonTermMultiDivExpr => {
                write!(f, "NonTermMultiDiv::")
            }
            Self::NonTermDivExpr => {
                write!(f, "NonTermDiv::")
            }
            Self::NonTermTermExpr => {
                write!(f, "NonTermTerm::")
            }
            Self::TermDivide => {
                write!(f, "Term('/')")
            }
            Self::TermMultiply => {
                write!(f, "Term('*')")
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
    UnknownParseError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EndOfTokenError => write!(f, "Reached end of token stream!"),
            ParseError::InvalidTokenError(s) => write!(f, "Invalid token found: {}", s),
            ParseError::UnknownParseError => write!(f, "Unknown error during parsing!"),
        }
    }
}

/// check if we have reached EOF by inspecting the current position with the
/// number of tokens
fn peek(tokens: &Vec<LexToken>, pos: usize) -> Result<Option<&LexToken>, ParseError> {
    if (pos + 1) >= tokens.len() {
        return Err(ParseError::EndOfTokenError);
    }
    let tok = tokens.get(pos + 1);
    println!("=> [peek] tok: {:?} pos: [{}]", tok, pos + 1);
    return Ok(tok);
}

/// parsing top level expression
fn parse_expr(tokens: &Vec<LexToken>, pos: usize) -> Result<(ParseNode, usize), ParseError> {
    println!("=> [parse_expr] parsing at position {pos}");
    let (multi_expr_node, new_pos) = parse_multi_div_expr(&tokens, pos)?;
    let mut expr_node = ParseNode::new(CfgTerm::NonTermExpr);
    expr_node.add_child_node(multi_expr_node);

    // look for +/-
    println!("=> [parse_expr] at position {new_pos}");
    let tok = peek(&tokens, new_pos)?;
    println!("=> tok: {:?}", tok);
    match tok {
        Some(LexToken::Add(_)) => {
            expr_node.add_child_node(ParseNode::new(CfgTerm::TermPlus));
        }
        Some(LexToken::Subtract(_)) => {
            expr_node.add_child_node(ParseNode::new(CfgTerm::TermMinus));
        }
        Some(LexToken::Num(n)) => {
            // multi_div_expr _ <> expr
            println!("=> [parse_expr] n: {}", *n);
            println!("=> parsing tail expr ...");
            let pt_node = ParseNode::new(CfgTerm::TermNumber(*n));
            expr_node.add_child_node(pt_node);
            let (tail_expr_node, tail_expr_pos) = parse_expr(&tokens, new_pos + 1)?;
            expr_node.add_child_node(tail_expr_node);
            return Ok((expr_node, tail_expr_pos));
        }
        _ => {
            return Err(ParseError::InvalidTokenError(String::from(
                "Expected +/- sign!",
            )));
        }
    }

    // look for expr
    println!("=> parsing tail expr ...");
    let (tail_expr_node, tail_expr_pos) = parse_expr(&tokens, new_pos + 2)?;
    expr_node.add_child_node(tail_expr_node);

    Ok((expr_node, tail_expr_pos))
}

/// parsing expression involving multiply (or '*')
fn parse_multi_div_expr(
    tokens: &Vec<LexToken>,
    pos: usize,
) -> Result<(ParseNode, usize), ParseError> {
    println!("=> [multi_div_expr] parsing node at position {pos}");
    // call div_expr
    let (div_expr_node, new_pos) = parse_div_expr(tokens, pos)?;
    let mut mde_node = ParseNode::new(CfgTerm::NonTermMultiDivExpr);
    mde_node.child_nodes.push(div_expr_node);

    // div_expr * multi_div_expr => look for multiply '*' symbol -
    // div_expr => look for '+'/- symbol, this is essentially follow-on sets
    println!("=> [multi_div_expr] at position: {new_pos}");
    let tok = peek(&tokens, new_pos)?;
    println!("=> [multi_div_expr] tok: {:?}: ", tok);
    match tok {
        Some(LexToken::Multi(_)) => {
            mde_node.add_child_node(ParseNode::new(CfgTerm::TermMultiply));
        }
        Some(LexToken::Add(_)) | Some(LexToken::Subtract(_)) => {
            // return as this is end of div_expr, and is to be parsed by
            // tail end of expr, so pass the current pos, next_pos, which will
            // then be parsed by the parent function
            return Ok((mde_node, new_pos + 1));
        }
        Some(LexToken::Num(n)) => {
            // multi_div_expr _ <> expr
            println!("=> [multi_div_expr] n: {}", *n);
            let pt_node = ParseNode::new(CfgTerm::TermNumber(*n));
            mde_node.add_child_node(pt_node);
            return Ok((mde_node, new_pos + 1));
        }
        _ => {
            return Err(ParseError::InvalidTokenError(String::from(
                "Expected * sign!",
            )));
        }
    }

    // look for expr
    println!("=> looking for multi_div_expr ...");
    let (tail_mde_node, tail_mde_pos) = parse_multi_div_expr(&tokens, new_pos + 2)?;
    mde_node.add_child_node(tail_mde_node);

    Ok((mde_node, tail_mde_pos))
}

// parsing the divide expression
fn parse_div_expr(tokens: &Vec<LexToken>, pos: usize) -> Result<(ParseNode, usize), ParseError> {
    println!("=> [div_expr] parsing div_expr node at position {pos}");
    // call term
    let (term_node, new_pos) = parse_term(tokens, pos)?;
    let mut dive_node = ParseNode::new(CfgTerm::NonTermDivExpr);
    dive_node.child_nodes.push(term_node);

    // term / div_expr - look for multiply '/' symbol -
    // term - look for '+'/- symbol, this is essentially follow-on sets
    let tok = peek(&tokens, new_pos)?;
    println!("=> [div_expr] tok: {:?}", tok);
    match tok {
        Some(LexToken::Div(_)) => {
            println!("=> [div_expr] found / ");
            dive_node.add_child_node(ParseNode::new(CfgTerm::TermDivide));
        }
        Some(LexToken::Add(_)) | Some(LexToken::Subtract(_)) => {
            // PEEK but don't consume
            // return as this is end of div_expr, and is to be parsed by
            // tail end of (expr: multi_div_expr <> _ expr)
            println!("=> [div_expr] found + or - , next_pos: {}", new_pos + 1);
            return Ok((dive_node, new_pos));
        }
        Some(LexToken::Multi(_)) => {
            // PEEK but don't consume
            // return as this is end of div_expr, and is to be parsed by
            // tail end of (multi_div_expr: div_expr <> * multi_div_expr)
            println!("=> [div_expr] found * , next_pos: {}", new_pos + 1);
            return Ok((dive_node, new_pos));
        }
        _ => {
            return Err(ParseError::InvalidTokenError(String::from(
                "Expected * sign!",
            )));
        }
    }

    // look for div_expr
    println!("=> looking for term ...");
    let (tail_dive_node, tail_dive_pos) = parse_div_expr(&tokens, new_pos + 2)?;
    dive_node.add_child_node(tail_dive_node);

    Ok((dive_node, tail_dive_pos))
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
            let (expr_node, expr_pos) = parse_expr(&tokens, pos + 1)?;
            term_node.child_nodes.push(expr_node);

            // close parens
            // let close_parens_tok = tokens.get(expr_pos + 1).expect("Expected close parens");
            let close_parens_tok = peek(&tokens, expr_pos + 1)?;
            assert_eq!(*(close_parens_tok.unwrap()), LexToken::RightParen(')'));

            let right_parens_node = ParseNode::new(CfgTerm::TermRightParens);
            term_node.child_nodes.push(right_parens_node);

            return Ok((term_node, expr_pos + 2));
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
        _ => Err(ParseError::UnknownParseError),
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
        let p_node = cfg::parse(lex_tokens);
        match p_node {
            Ok(parsed_node) => {
                println!("parsed node: {}", parsed_node);
                assert_eq!(parsed_node.current_node, CfgTerm::NonTermExpr);
                assert_eq!(parsed_node.child_nodes.len(), 3);
                let left_child_node = parsed_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a terminal number node");
                assert_eq!(left_child_node.current_node, CfgTerm::NonTermMultiDivExpr);
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
                assert_eq!(
                    right_child_me_node.current_node,
                    CfgTerm::NonTermMultiDivExpr
                );
                let right_child_num_node = right_child_me_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a terminal number node");
                assert_eq!(right_child_num_node.current_node, CfgTerm::TermNumber(3));
            }
            Err(ParseError::EndOfTokenError) => {
                // end of token list
                println!("** EndOfTokenError! **");
            }
            _ => {
                println!("Unknown error!");
            }
        }
    }
}
