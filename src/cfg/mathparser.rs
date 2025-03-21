use crate::{
    cfg::CfgTerm,
    cfg::ParseError,
    lex::{self, LexToken},
};

use super::ParseNode;

pub(crate) struct MathParser {
    s: String,
    lex_tokens: Vec<LexToken>,
    pub(crate) parsed_node: Option<ParseNode>,
}

impl MathParser {
    pub(crate) fn new(s: String) -> Self {
        MathParser {
            s: s,
            lex_tokens: vec![],
            parsed_node: None,
        }
    }

    /// check if we have reached EOF by inspecting the current position with the
    /// number of tokens
    fn peek(&self, pos: usize) -> Option<&LexToken> {
        if (pos + 1) >= self.lex_tokens.len() {
            return None;
        }
        let tok = self.lex_tokens.get(pos + 1);
        println!("=> [peek] tok: {:?} pos: [{}]", tok, pos + 1);
        return tok;
    }

    /// parsing term (either number or sub expr)
    fn parse_term(
        &mut self,
        pos: usize,
        node_depth: usize,
    ) -> Result<(ParseNode, usize), ParseError> {
        println!("=> [{node_depth}]parsing term node at position {pos} ...");
        let tok = self.lex_tokens.get(pos);
        match tok {
            Some(LexToken::LeftParen('(')) => {
                let mut term_node = ParseNode::new(CfgTerm::NonTermTermExpr, node_depth);

                println!("(");
                let left_parens_node = ParseNode::new(CfgTerm::TermLeftParens, node_depth + 1);
                term_node.child_nodes.push(left_parens_node);
                let (expr_node, expr_pos) = self.parse_expr(pos + 1, node_depth + 1)?;
                term_node.child_nodes.push(expr_node);
                println!("term_node: {}, expr_pos: {}", term_node, expr_pos);

                // close parens
                // let close_parens_tok = tokens.get(expr_pos + 1).expect("Expected close parens");
                let close_parens_tok = self.peek(expr_pos);
                println!("close_parens_tok: {:?}", close_parens_tok);
                assert_eq!(*(close_parens_tok.unwrap()), LexToken::RightParen(')'));

                let right_parens_node = ParseNode::new(CfgTerm::TermRightParens, node_depth + 1);
                term_node.child_nodes.push(right_parens_node);
                println!("term_node: {}, expr_pos+2: {}", term_node, expr_pos + 1);

                return Ok((term_node, expr_pos + 1));
            }
            Some(LexToken::Num(n)) => {
                println!("term num: {}", *n);
                let pt_node = ParseNode::new(CfgTerm::TermNumber(*n), node_depth);
                println!("term num node: {pt_node}");
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

    // parsing the divide expression
    fn parse_div_expr(
        &mut self,
        pos: usize,
        node_depth: usize,
    ) -> Result<(ParseNode, usize), ParseError> {
        println!("=> [{node_depth}][div_expr] parsing div_expr node at position {pos}");
        // call term
        let (term_node, new_pos) = self.parse_term(pos, node_depth + 1)?;
        let mut dive_node = ParseNode::new(CfgTerm::NonTermDivExpr, node_depth);
        dive_node.child_nodes.push(term_node);
        println!("dive_node, after adding term: {dive_node}");

        // term / div_expr - look for multiply '/' symbol -
        // term - look for '+'/- symbol, this is essentially follow-on sets
        let tok = self.peek(new_pos);
        match tok {
            Some(LexToken::Div(_)) => {
                println!("=> [div_expr] found / ");
                dive_node.add_child_node(ParseNode::new(CfgTerm::TermDivide, node_depth + 1));
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
            Some(LexToken::RightParen(_)) => {
                println!("=> [div_expr] right parens");
                return Ok((dive_node, new_pos));
            }
            None => {
                println!("=> [div_expr] End of token stream!");
                return Ok((dive_node, new_pos));
            }
            _ => {
                return Err(ParseError::InvalidTokenError(String::from(
                    "Expected * sign!",
                )));
            }
        }

        // look for div_expr
        println!("=>[pos] looking for term ...");
        let (tail_dive_node, tail_dive_pos) = self.parse_div_expr(new_pos + 2, node_depth + 1)?;
        dive_node.add_child_node(tail_dive_node);

        Ok((dive_node, tail_dive_pos))
    }

    /// parsing expression involving multiply (or '*')
    fn parse_multi_div_expr(
        &mut self,
        pos: usize,
        node_depth: usize,
    ) -> Result<(ParseNode, usize), ParseError> {
        println!("=> [{node_depth}][multi_div_expr] parsing node at position {pos}");
        // call div_expr
        let (div_expr_node, new_pos) = self.parse_div_expr(pos, node_depth + 1)?;
        let mut mde_node = ParseNode::new(CfgTerm::NonTermMultiDivExpr, node_depth);
        mde_node.child_nodes.push(div_expr_node);
        println!("mde_node, after adding div_expr_node: {mde_node}");

        // div_expr * multi_div_expr => look for multiply '*' symbol -
        // div_expr => look for '+'/- symbol, this is essentially follow-on sets
        println!("=> [multi_div_expr] at position: {new_pos}");
        let tok = self.peek(new_pos);
        match tok {
            Some(LexToken::Multi(_)) => {
                mde_node.add_child_node(ParseNode::new(CfgTerm::TermMultiply, node_depth + 1));
            }
            Some(LexToken::Add(_)) | Some(LexToken::Subtract(_)) => {
                // return as this is end of div_expr, and is to be parsed by
                // tail end of expr, so pass the current position (new_pos),
                // which will then be parsed by the parent function
                return Ok((mde_node, new_pos));
            }
            Some(LexToken::Num(n)) => {
                // multi_div_expr _ <> expr
                println!("=> [multi_div_expr] n: {}", *n);
                let pt_node = ParseNode::new(CfgTerm::TermNumber(*n), node_depth);
                mde_node.add_child_node(pt_node);
                return Ok((mde_node, new_pos + 1));
            }
            Some(LexToken::RightParen(_)) => {
                println!("=> [multi_div_expr] right parens");
                return Ok((mde_node, new_pos));
            }
            None => {
                println!("=> [multi_div_expr] End of token stream!");
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
        let (tail_mde_node, tail_mde_pos) =
            self.parse_multi_div_expr(new_pos + 2, node_depth + 1)?;
        mde_node.add_child_node(tail_mde_node);

        Ok((mde_node, tail_mde_pos))
    }

    /// parsing top level expression
    fn parse_expr(
        &mut self,
        pos: usize,
        node_depth: usize,
    ) -> Result<(ParseNode, usize), ParseError> {
        println!("=> [{node_depth}][parse_expr] parsing at position {pos}");
        let mut expr_node = ParseNode::new(crate::cfg::CfgTerm::NonTermExpr, node_depth);
        let (multi_expr_node, new_pos) = self.parse_multi_div_expr(pos, node_depth + 1)?;
        expr_node.add_child_node(multi_expr_node);
        println!("expr_node, after adding multi_expr_node: {expr_node}");

        // look for +/-
        println!("=> [parse_expr] at position {new_pos}");
        let tok = self.peek(new_pos);
        match tok {
            Some(LexToken::Add(_)) => {
                expr_node.add_child_node(ParseNode::new(CfgTerm::TermPlus, node_depth + 1));
            }
            Some(LexToken::Subtract(_)) => {
                expr_node.add_child_node(ParseNode::new(CfgTerm::TermMinus, node_depth + 1));
            }
            // Some(LexToken::Num(n)) => {
            //     // multi_div_expr _ <> expr
            //     println!("=> ***** [parse_expr] n: {}", *n);
            //     println!("=> parsing tail expr ...");
            //     let pt_node = ParseNode::new(CfgTerm::TermNumber(*n), node_depth);
            //     expr_node.add_child_node(pt_node);
            //     let (tail_expr_node, tail_expr_pos) =
            //         self.parse_expr(new_pos + 1, node_depth + 1)?;
            //     return Ok((tail_expr_node, tail_expr_pos));
            // }
            Some(LexToken::RightParen(_)) => {
                println!("=> [expr] right parens");
                return Ok((expr_node, new_pos));
            }
            None => {
                println!("=> [parse_expr] End of token stream!");
                return Ok((expr_node, new_pos));
            }
            _ => {
                return Err(ParseError::InvalidTokenError(String::from(
                    "Expected +/- sign!",
                )));
            }
        }

        // look for expr
        println!("=> parsing tail expr ...");
        let (tail_expr_node, tail_expr_pos) = self.parse_expr(new_pos + 2, node_depth + 1)?;
        expr_node.add_child_node(tail_expr_node);

        Ok((expr_node, tail_expr_pos))
    }

    /// respresents the start_rule in the grammar
    fn start_rule(&mut self) -> Result<(), ParseError> {
        let start_depth = 0;
        let mut start_node = ParseNode::new(crate::cfg::CfgTerm::NonTermStartRule, start_depth);
        let (expr_node, _) = self.parse_expr(0, start_depth + 1)?;
        start_node.add_child_node(expr_node);
        self.parsed_node = Some(start_node);
        Ok(())
    }

    pub(crate) fn start(&mut self) -> Result<(), ParseError> {
        self.lex_tokens = lex::lexer(self.s.trim()).expect("Failed to tokenize the string!");
        self.start_rule()
    }
}
