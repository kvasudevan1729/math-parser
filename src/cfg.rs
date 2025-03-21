// Create a parse tree from the math expression
pub mod mathparser;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CfgTerm {
    NonTermStartRule,
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
            Self::NonTermStartRule => {
                write!(f, "NonTermStartRule::")
            }
            Self::NonTermExpr => {
                write!(f, "NonTermExpr::")
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
    node_depth: usize,
}

impl ParseNode {
    pub fn new(current_node: CfgTerm, node_depth: usize) -> ParseNode {
        ParseNode {
            current_node,
            child_nodes: Vec::new(),
            node_depth,
        }
    }

    pub fn add_child_node(&mut self, child_node: ParseNode) {
        self.child_nodes.push(child_node);
    }
}

impl fmt::Display for ParseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut space: String = " ".to_owned();
        let mut s: String = " ".to_owned();
        for (_, n) in self.child_nodes.iter().enumerate() {
            s.push_str(&format!("\n{}", n.to_string()));
        }
        for _ in 0..self.node_depth {
            space.push_str(" ");
        }
        write!(
            f,
            "{space}+[{}](child count={}){s}",
            self.current_node,
            self.child_nodes.len()
        )
    }
}

#[derive(Debug)]
pub(crate) enum ParseError {
    InvalidTokenError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidTokenError(s) => write!(f, "Invalid token found: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cfg::mathparser::MathParser;
    use crate::cfg::CfgTerm;

    #[test]
    fn test_parse_add_expr() {
        // This test asserts that the below expression has the correct set of
        // parse nodes, including the non-terminals and terminals.
        let s = "2 + 3";
        let mut math_parser = MathParser::new(String::from(s));
        let _ = math_parser.start();
        match math_parser.parsed_node {
            Some(parsed_node) => {
                println!("parsed node: {}", parsed_node);
                assert_eq!(parsed_node.current_node, CfgTerm::NonTermStartRule);
                let start_child = parsed_node.current_node;
                println!("start_child: {}", start_child);
                assert_eq!(parsed_node.child_nodes.len(), 1);
                let left_child_expr_node = parsed_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a expr node");
                println!("\nleft_child_expr_node: {}", left_child_expr_node);
                assert_eq!(left_child_expr_node.current_node, CfgTerm::NonTermExpr);

                let multi_div_expr_node = left_child_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a multi div expr node");
                println!("\nmulti_div_expr_node: {}", multi_div_expr_node);
                assert_eq!(
                    multi_div_expr_node.current_node,
                    CfgTerm::NonTermMultiDivExpr
                );

                let div_expr_node = multi_div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a div expr node");
                println!("\ndiv_expr_node: {}", div_expr_node);
                assert_eq!(div_expr_node.current_node, CfgTerm::NonTermDivExpr);

                let num_node = div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a num node");
                assert_eq!(num_node.current_node, CfgTerm::TermNumber(2));

                // '+' bit
                let plus_child_node = left_child_expr_node
                    .child_nodes
                    .get(1)
                    .expect("Expected a terminal + node");
                assert_eq!(plus_child_node.current_node, CfgTerm::TermPlus);

                // right side of plus, expr -> multi_dev_expr '+' expr
                let right_child_node = left_child_expr_node
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

                let right_child_div_node = right_child_me_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a div expr node");
                assert_eq!(right_child_div_node.current_node, CfgTerm::NonTermDivExpr);

                let right_child_num_node = right_child_div_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a terminal number node");
                assert_eq!(right_child_num_node.current_node, CfgTerm::TermNumber(3));
            }
            _ => {
                println!("Unknown error!");
            }
        }
    }

    #[test]
    fn test_multiply_expr() {
        let s = "3 * 4";
        let mut math_parser = MathParser::new(String::from(s));
        let _ = math_parser.start();
        match math_parser.parsed_node {
            Some(parsed_node) => {
                println!("parsed node: {}", parsed_node);
                assert_eq!(parsed_node.current_node, CfgTerm::NonTermStartRule);
                let start_child = parsed_node.current_node;
                println!("start_child: {}", start_child);
                assert_eq!(parsed_node.child_nodes.len(), 1);

                let left_child_expr_node = parsed_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a expr node");
                println!("\nleft_child_expr_node: {}", left_child_expr_node);
                assert_eq!(left_child_expr_node.current_node, CfgTerm::NonTermExpr);

                let multi_div_expr_node = left_child_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a multi div expr node");
                println!("\nmulti_div_expr_node: {}", multi_div_expr_node);
                assert_eq!(
                    multi_div_expr_node.current_node,
                    CfgTerm::NonTermMultiDivExpr
                );

                let div_expr_node = multi_div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a div expr node");
                println!("\ndiv_expr_node: {}", div_expr_node);
                assert_eq!(div_expr_node.current_node, CfgTerm::NonTermDivExpr);

                let num_node = div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a num node");
                assert_eq!(num_node.current_node, CfgTerm::TermNumber(3));

                let term_multiply_node = multi_div_expr_node
                    .child_nodes
                    .get(1)
                    .expect("Expected a multiply term node");
                println!("\nterm_multiply_node: {}", term_multiply_node);
                assert_eq!(term_multiply_node.current_node, CfgTerm::TermMultiply);

                let right_multi_div_expr_node = multi_div_expr_node
                    .child_nodes
                    .get(2)
                    .expect("Expected a multi div expr node");
                println!("\nright_multi_div_expr_node: {}", right_multi_div_expr_node);
                assert_eq!(
                    right_multi_div_expr_node.current_node,
                    CfgTerm::NonTermMultiDivExpr
                );

                let right_div_expr_node = right_multi_div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a div expr node");
                println!("\nright_div_expr_node: {}", right_div_expr_node);
                assert_eq!(right_div_expr_node.current_node, CfgTerm::NonTermDivExpr);

                let right_num_node = right_div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a num node");
                assert_eq!(right_num_node.current_node, CfgTerm::TermNumber(4));
            }
            _ => {
                println!("Unknown error!");
            }
        }
    }
}
