use nu_protocol::ast::{Block, Expr, Expression, Pipeline, Statement};
use nu_protocol::{engine::StateWorkingSet, Span};

#[derive(Debug)]
pub enum FlatShape {
    Garbage,
    Bool,
    Int,
    Float,
    InternalCall,
    External,
    Literal,
    Operator,
    Signature,
    String,
    Variable,
}

pub fn flatten_block(working_set: &StateWorkingSet, block: &Block) -> Vec<(Span, FlatShape)> {
    let mut output = vec![];
    for stmt in &block.stmts {
        output.extend(flatten_statement(working_set, stmt));
    }
    output
}

pub fn flatten_statement(
    working_set: &StateWorkingSet,
    stmt: &Statement,
) -> Vec<(Span, FlatShape)> {
    match stmt {
        Statement::Pipeline(pipeline) => flatten_pipeline(working_set, pipeline),
        _ => vec![],
    }
}

pub fn flatten_expression(
    working_set: &StateWorkingSet,
    expr: &Expression,
) -> Vec<(Span, FlatShape)> {
    match &expr.expr {
        Expr::BinaryOp(lhs, op, rhs) => {
            let mut output = vec![];
            output.extend(flatten_expression(working_set, lhs));
            output.extend(flatten_expression(working_set, op));
            output.extend(flatten_expression(working_set, rhs));
            output
        }
        Expr::Block(block_id) => flatten_block(working_set, working_set.get_block(*block_id)),
        Expr::Call(call) => {
            let mut output = vec![(call.head, FlatShape::InternalCall)];
            for positional in &call.positional {
                output.extend(flatten_expression(working_set, positional));
            }
            output
        }
        Expr::ExternalCall(..) => {
            vec![(expr.span, FlatShape::External)]
        }
        Expr::Garbage => {
            vec![(expr.span, FlatShape::Garbage)]
        }
        Expr::Int(_) => {
            vec![(expr.span, FlatShape::Int)]
        }
        Expr::Float(_) => {
            vec![(expr.span, FlatShape::Float)]
        }
        Expr::Bool(_) => {
            vec![(expr.span, FlatShape::Bool)]
        }

        Expr::List(list) => {
            let mut output = vec![];
            for l in list {
                output.extend(flatten_expression(working_set, l));
            }
            output
        }
        Expr::Keyword(_, span, expr) => {
            let mut output = vec![(*span, FlatShape::Operator)];
            output.extend(flatten_expression(working_set, expr));
            output
        }
        Expr::Operator(_) => {
            vec![(expr.span, FlatShape::Operator)]
        }
        Expr::Signature(_) => {
            vec![(expr.span, FlatShape::Signature)]
        }
        Expr::String(_) => {
            vec![(expr.span, FlatShape::String)]
        }
        Expr::Subexpression(block_id) => {
            flatten_block(working_set, working_set.get_block(*block_id))
        }
        Expr::Table(headers, cells) => {
            let mut output = vec![];
            for e in headers {
                output.extend(flatten_expression(working_set, e));
            }
            for row in cells {
                for expr in row {
                    output.extend(flatten_expression(working_set, expr));
                }
            }
            output
        }
        Expr::Var(_) => {
            vec![(expr.span, FlatShape::Variable)]
        }
    }
}

pub fn flatten_pipeline(
    working_set: &StateWorkingSet,
    pipeline: &Pipeline,
) -> Vec<(Span, FlatShape)> {
    let mut output = vec![];
    for expr in &pipeline.expressions {
        output.extend(flatten_expression(working_set, expr))
    }
    output
}