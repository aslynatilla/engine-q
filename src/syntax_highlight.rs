use std::sync::Arc;

use crate::{Block, Expr, Expression, ParserState, ParserWorkingSet, Statement};

fn syntax_highlight(parser_state: Arc<ParserState>, input: &[u8]) {
    let mut working_set = ParserWorkingSet::new(Some(parser_state));

    let (block, _) = working_set.parse_source(input, false);

    // for stmt in &block.stmts {
    //     match stmt {
    //         Statement::Expression(expr) => {

    //         }
    //     }
    // }
    // No merge at the end because this parse is speculative
}

fn highlight_expression(expression: &Expression) {
    // match &expression.expr {
    //     Expr::BinaryOp()
    // }
}