// use enum to define different grammar items

use crate::lex::LexToken;

pub enum CfgTerm {
    NonTerm_Expr,
    NonTerm_MultiExpr,
    NonTerm_Term,
    TermNumber(u32),
    TermPlus,
    TermLeftParens,
    TermRightParens,
}

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

/// parsing top level expression
fn parse_expr(tokens: &Vec<LexToken>, pos: usize) -> Result<(ParseNode, usize), String> {
    println!("=> parsing expr node at position {pos}");
    let (multi_expr_node, new_pos) = parse_multi_expr(&tokens, pos)?;
    let mut expr_node = ParseNode::new(CfgTerm::NonTerm_Expr);
    expr_node.add_child_node(multi_expr_node);

    // possible that we have reached EOF
    if (new_pos + 1) >= tokens.len() {
        return Ok((expr_node, new_pos));
    }

    // look for +
    println!("=> looking for + ... {new_pos}");
    let tok = tokens.get(new_pos + 1);
    println!("=> tok: {:?}", tok);
    match tok {
        Some(LexToken::Add(_)) => {
            expr_node.add_child_node(ParseNode::new(CfgTerm::TermPlus));
        }
        _ => {
            return Err(format!("Expected plus sign!"));
        }
    }

    // look for expr
    println!("=> looking for + expr ...");
    let (tail_expr_node, tail_expr_pos) =
        parse_expr(&tokens, new_pos + 2).expect("tail expression failed!");
    expr_node.add_child_node(tail_expr_node);

    Ok((expr_node, tail_expr_pos))
}

/// parsing expression involving multiply (or '*')
fn parse_multi_expr(tokens: &Vec<LexToken>, pos: usize) -> Result<(ParseNode, usize), String> {
    println!("=> parsing multi_expr node at position {pos}");
    // call term
    let (term_node, new_pos) = parse_term(tokens, pos)?;
    let mut me_node = ParseNode::new(CfgTerm::NonTerm_MultiExpr);
    me_node.child_nodes.push(term_node);
    Ok((me_node, new_pos))
}

/// parsing term (either number or sub expr)
fn parse_term(tokens: &Vec<LexToken>, pos: usize) -> Result<(ParseNode, usize), String> {
    println!("=> parsing term node at position {pos} ...");
    let tok = tokens.get(pos);
    match tok {
        Some(LexToken::LeftParen('(')) => {
            let mut term_node = ParseNode::new(CfgTerm::NonTerm_Term);

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
            return Err(format!("Error: invalid term token!"));
        }
    }
}

/// Construct parse tree from the list of lex tokens
pub(crate) fn parse(tokens: Vec<LexToken>) -> Result<(ParseNode, usize), String> {
    // start off with Expr
    return parse_expr(&tokens, 0);
}
