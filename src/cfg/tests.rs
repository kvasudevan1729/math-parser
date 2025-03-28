mod tests {
    use crate::cfg::mathparser::MathParser;
    use crate::cfg::CfgTerm;
    use crate::lex_multi_digit::lexer;

    #[test]
    fn test_parse_add_expr() {
        // This test asserts that the below expression has the correct set of
        // parse nodes, including the non-terminals and terminals.
        let s = "2 + 3";
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        let mut math_parser = MathParser::new(tokens);
        let _ = math_parser.parse();
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
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        let mut math_parser = MathParser::new(tokens);
        let _ = math_parser.parse();
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

    #[test]
    fn test_divide_expr() {
        let s = "3 / 4";
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        let mut math_parser = MathParser::new(tokens);
        let _ = math_parser.parse();
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

                let term_divide_node = div_expr_node
                    .child_nodes
                    .get(1)
                    .expect("Expected a divide term node");
                println!("\nterm_divide_node: {}", term_divide_node);
                assert_eq!(term_divide_node.current_node, CfgTerm::TermDivide);

                let right_div_expr_node = div_expr_node
                    .child_nodes
                    .get(2)
                    .expect("Expected a div expr node");
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

    #[test]
    fn test_parens_and_add_expr() {
        let s = "(2 / 3) + 4";
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        let mut math_parser = MathParser::new(tokens);
        let _ = math_parser.parse();
        match math_parser.parsed_node {
            Some(parsed_node) => {
                println!("parsed node: {}", parsed_node);
                assert_eq!(parsed_node.current_node, CfgTerm::NonTermStartRule);
                let start_child = parsed_node.current_node;
                println!("start_child: {}", start_child);
                assert_eq!(parsed_node.child_nodes.len(), 1);
            }
            _ => {
                println!("Unknown error!");
            }
        }
    }

    #[test]
    fn test_parens_and_div_expr() {
        let s = "(2 / 3) / 4";
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        let mut math_parser = MathParser::new(tokens);
        let _ = math_parser.parse();
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

                let term_expr_node = div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a num node");
                assert_eq!(term_expr_node.current_node, CfgTerm::NonTermTermExpr);

                let left_parens_node = term_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a left parens node");
                assert_eq!(left_parens_node.current_node, CfgTerm::TermLeftParens);

                // fill the ( expr ) => ( 2 / 3) bit here

                let sub_term_expr_node = term_expr_node
                    .child_nodes
                    .get(1)
                    .expect("Expected a sub term expr node");
                assert_eq!(sub_term_expr_node.current_node, CfgTerm::NonTermExpr);

                let sub_term_multi_div_expr_node = sub_term_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a sub term multi div expr node");
                assert_eq!(
                    sub_term_multi_div_expr_node.current_node,
                    CfgTerm::NonTermMultiDivExpr
                );

                let sub_term_div_expr_node = sub_term_multi_div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a sub term div expr node");
                assert_eq!(sub_term_div_expr_node.current_node, CfgTerm::NonTermDivExpr);

                let sub_term_left_num_node = sub_term_div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a sub term left num node");
                assert_eq!(sub_term_left_num_node.current_node, CfgTerm::TermNumber(2));

                let sub_term_div_sym_node = sub_term_div_expr_node
                    .child_nodes
                    .get(1)
                    .expect("Expected a sub term divide symbol node");
                assert_eq!(sub_term_div_sym_node.current_node, CfgTerm::TermDivide);

                let sub_term_right_div_expr_node = sub_term_div_expr_node
                    .child_nodes
                    .get(2)
                    .expect("Expected a sub term right div expr node");
                assert_eq!(
                    sub_term_right_div_expr_node.current_node,
                    CfgTerm::NonTermDivExpr
                );

                let sub_term_right_num_node = sub_term_right_div_expr_node
                    .child_nodes
                    .get(0)
                    .expect("Expected a sub term right num node");
                assert_eq!(sub_term_right_num_node.current_node, CfgTerm::TermNumber(3));

                // sub term ends

                let right_parens_node = term_expr_node
                    .child_nodes
                    .get(2)
                    .expect("Expected a right parens node");
                assert_eq!(right_parens_node.current_node, CfgTerm::TermRightParens);

                // '/' bit
                let div_node = div_expr_node
                    .child_nodes
                    .get(1)
                    .expect("Expected a div symbol terminal node");
                assert_eq!(div_node.current_node, CfgTerm::TermDivide);

                let right_div_expr_node = div_expr_node
                    .child_nodes
                    .get(2)
                    .expect("Expected a div symbol terminal node");
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

    #[test]
    fn test_parens_and_multiply_expr() {
        let s = "(2 / 3) * 4";
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        let mut math_parser = MathParser::new(tokens);
        let _ = math_parser.parse();
        match math_parser.parsed_node {
            Some(parsed_node) => {
                println!("parsed node: {}", parsed_node);
                assert_eq!(parsed_node.current_node, CfgTerm::NonTermStartRule);
                let start_child = parsed_node.current_node;
                println!("start_child: {}", start_child);
                assert_eq!(parsed_node.child_nodes.len(), 1);
            }
            _ => {
                println!("Unknown error!");
            }
        }
    }

    #[test]
    fn test_parens_and_parens_expr() {
        let s = "(2 / 3) / ( 3 / 4)";
        let my_lex = lexer(s).unwrap();
        let tokens = my_lex.get_tokens();
        let mut math_parser = MathParser::new(tokens);
        let _ = math_parser.parse();
        match math_parser.parsed_node {
            Some(parsed_node) => {
                println!("parsed node: {}", parsed_node);
                assert_eq!(parsed_node.current_node, CfgTerm::NonTermStartRule);
                let start_child = parsed_node.current_node;
                println!("start_child: {}", start_child);
                assert_eq!(parsed_node.child_nodes.len(), 1);
            }
            _ => {
                println!("Unknown error!");
            }
        }
    }
}
