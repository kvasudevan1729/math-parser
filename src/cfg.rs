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
mod tests;
