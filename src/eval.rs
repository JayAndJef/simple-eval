use std::ops::{Add, Div, Mul, Sub};

use crate::parser::{ASTNode, Operator};

pub fn evaluate(node: ASTNode) -> f64 {
    match node {
        ASTNode::Leaf(val) => val,
        ASTNode::Branch { left, right, operator } => {
            let bin_op = match operator {
                Operator::Add => f64::add,
                Operator::Sub => f64::sub,
                Operator::Mul => f64::mul,
                Operator::Div => f64::div,
                Operator::Exp => f64::powf,
                _ => unreachable!(),
            };

            bin_op(evaluate(*left), evaluate(*right))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{eval::evaluate, parser::{Operator, Operator::*, ASTNode}};

    #[test]
    fn always_passes() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(
            evaluate(ASTNode::Branch {
                left: Box::new(ASTNode::Leaf(3.0)),
                right: Box::new(ASTNode::Branch {
                    left: Box::new(ASTNode::Leaf(2.0)),
                    right: Box::new(ASTNode::Leaf(4.0)),
                    operator: Add
                }),
                operator: Operator::Mul
            }),
            18.0
        )
    }
}

