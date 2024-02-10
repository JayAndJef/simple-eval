use std::collections::{HashMap, VecDeque};

use crate::lexer::TokenKind;

#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Leaf(f64),
    Branch {
        left: Box<ASTNode>,
        right: Box<ASTNode>,
        operator: Operator,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Lparen,
    RParen,
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

impl ASTNode {
    pub fn new_branch(left: Box<ASTNode>, right: Box<ASTNode>, operator: Operator) -> Self {
        Self::Branch {
            left,
            right,
            operator,
        }
    }
}

struct ASTBuilder {
    input: VecDeque<TokenKind>,
    output: Vec<ASTNode>,
    operator_aux: Vec<Operator>,
}

impl From<TokenKind> for Operator {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Sub,
            TokenKind::Mul => Self::Mul,
            TokenKind::Div => Self::Div,
            TokenKind::Exp => Self::Exp,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct BadTokenError;

impl ASTBuilder {
    pub fn new(input: VecDeque<TokenKind>) -> Self {
        Self {
            input,
            output: Vec::new(),
            operator_aux: Vec::new(),
        }
    }

    pub fn build_ast(&mut self) -> Result<ASTNode, BadTokenError> {
        while let Some(input_head) = self.input.pop_front() {
            self.step(input_head)?;
        }

        while let Some(_) = self.operator_aux.last() {
            self.build_node()?;
        }

        Ok(self.output.pop().unwrap())
    }

    fn step(&mut self, input_head: TokenKind) -> Result<(), BadTokenError> {
        match input_head {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Mul
            | TokenKind::Div
            | TokenKind::Exp => {
                // dbg!(&self.operator_aux, &self.input, &self.output, input_head);
                let prev_head = self.operator_aux.last();
                match prev_head {
                    Some(op) => {
                        if Operator::from(input_head) as i32 - *op as i32 >= 2 {
                            // higher precedence
                            self.operator_aux.push(input_head.into());
                        } else {
                            while (*self.operator_aux.last().unwrap() as i32)
                                - (Operator::from(input_head) as i32)
                                < 2
                            {
                                // while precedence is lower or equal
                                self.build_node()?;
                            }
                        }
                    }
                    None => self.operator_aux.push(input_head.into()),
                }
            }
            TokenKind::Literal(literalval) => self.output.push(ASTNode::Leaf(literalval)),
            TokenKind::LParen => self.operator_aux.push(Operator::Lparen),
            TokenKind::RParen => {
                // dbg!(&self.operator_aux);

                while self.operator_aux.last().unwrap() != &Operator::Lparen {
                    self.build_node()?;
                }

                self.operator_aux.pop();
                // dbg!(&self.operator_aux, &self.output, &self.input);
            }
        }

        Ok(())
    }

    fn build_node(&mut self) -> Result<(), BadTokenError> {
        let right = self.output.pop();
        let left = self.output.pop();
        if right.is_none() || left.is_none() {
            return Err(BadTokenError);
        }
        self.output.push(ASTNode::new_branch(
            Box::new(left.unwrap()),
            Box::new(right.unwrap()),
            self.operator_aux.pop().unwrap(),
        ));
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        lexer::{TokenKind, TokenKind::*},
        parser::ASTNode,
    };
    use std::collections::VecDeque;

    use super::ASTBuilder;
    use super::Operator;
    use super::Operator::*;

    #[test]
    fn always_passes() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn test_single_expr() {
        let ast = ASTBuilder::new(VecDeque::from([Literal(3.0), Plus, Literal(2.0)]))
            .build_ast()
            .unwrap();
        assert_eq!(
            ast,
            ASTNode::Branch {
                left: Box::new(ASTNode::Leaf(3.0)),
                right: Box::new(ASTNode::Leaf(2.0)),
                operator: Add
            }
        )
    }

    #[test]
    fn test_multiple_expr() {
        let ast = ASTBuilder::new(VecDeque::from([
            Literal(3.0),
            Plus,
            Literal(2.0),
            TokenKind::Mul,
            Literal(4.0),
        ]))
        .build_ast()
        .unwrap();
        assert_eq!(
            ast,
            ASTNode::Branch {
                left: Box::new(ASTNode::Leaf(3.0)),
                right: Box::new(ASTNode::Branch {
                    left: Box::new(ASTNode::Leaf(2.0)),
                    right: Box::new(ASTNode::Leaf(4.0)),
                    operator: Operator::Mul
                }),
                operator: Add
            }
        )
    }

    #[test]
    fn test_parens() {
        let ast = ASTBuilder::new(VecDeque::from([
            Literal(3.0),
            TokenKind::Mul,
            TokenKind::LParen,
            Literal(2.0),
            Plus,
            Literal(4.0),
            TokenKind::RParen,
        ]))
        .build_ast()
        .unwrap();
        assert_eq!(
            ast,
            ASTNode::Branch {
                left: Box::new(ASTNode::Leaf(3.0)),
                right: Box::new(ASTNode::Branch {
                    left: Box::new(ASTNode::Leaf(2.0)),
                    right: Box::new(ASTNode::Leaf(4.0)),
                    operator: Add
                }),
                operator: Operator::Mul
            }

        )
    }
}
