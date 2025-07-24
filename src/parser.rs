use crate::node::Node;
use crate::operators::{BinaryOperator, UnaryOperator};
use crate::token::Token;
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(iter: Peekable<Iter<'a, Token>>) -> Self {
        return Parser { iter };
    }

    fn expect(&mut self, t: Token) -> () {
        let next = self.iter.peek().unwrap();
        match *next {
            token if (*token == t) => {
                self.iter.next();
            }
            _ => panic!("Expected {:#?} but got {:#?}", t, next),
        }
    }
    fn E(&mut self) -> Node {
        let next = self.iter.peek().unwrap();
        // println!("E Token: {:#?}", next);
        match *next {
            Token::Term(symbol) => {
                self.iter.next();
                return Node::Leaf(*symbol);
            }
            Token::LeftParen => {
                self.iter.next();
                let inner_expr = self.S();
                self.expect(Token::RightParen);
                return inner_expr;
            }
            _ => return panic!("Unexpected token in E block: {:?}", next),
        }
    }

    fn D(&mut self) -> Node {
        let next = self.iter.peek().unwrap();
        // println!("D Token: {:#?}", next);
        match *next {
            Token::Not => {
                self.iter.next();
                let node = self.E();
                return Node::UnaryExpr {
                    op: UnaryOperator::Not { symbol: '¬' },
                    child: Box::new(node),
                };
            }
            _ => {
                return self.E();
            }
        }
    }
    fn C(&mut self) -> Node {
        let mut node = self.D();
        loop {
            let next = self.iter.peek().unwrap();
            // println!("next: {:#?}", next);
            match *next {
                Token::And => {
                    self.iter.next();
                    node = Node::BinaryExpr {
                        op: BinaryOperator::And { symbol: '∧' },
                        lhs: Box::new(node),
                        rhs: Box::new(self.D()),
                    };
                }
                _ => {
                    break;
                }
            }
        }

        return node;
    }
    fn S(&mut self) -> Node {
        let mut node = self.C();
        loop {
            let next = self.iter.peek().unwrap();
            // println!("next: {:#?}", next);
            match *next {
                Token::Or => {
                    self.iter.next();
                    node = Node::BinaryExpr {
                        op: BinaryOperator::Or { symbol: '∨' },
                        lhs: Box::new(node),
                        rhs: Box::new(self.C()),
                    };
                }
                _ => {
                    break;
                }
            }
        }

        return node;
    }
    pub fn parse(&mut self) -> Node {
        let ast = self.S();
        return ast;
    }
}
