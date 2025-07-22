use crate::operators::{BinaryOperator, UnaryOperator};
use crate::{Node, Token};
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(iter: Peekable<Iter<'a, Token>>) -> Self {
        return Parser { iter };
    }

    fn expect(&mut self, t: Token) -> Result<(), String> {
        let next = self.iter.peek().unwrap();
        match *next {
            t => {
                self.iter.next();
                return Ok(());
            }
            _ => Err(format!("Expected {:#?} but got {:#?}", t, next)),
        }
    }
    fn E(&mut self) -> Result<Node, String> {
        let next = self.iter.peek().unwrap();
        // println!("E Token: {:#?}", next);
        match *next {
            Token::Term(symbol) => {
                self.iter.next();
                return Ok(Node::Leaf(*symbol));
            }
            Token::LeftParen => {
                self.iter.next();
                let inner_expr = self.S();

                if let Err(s) = self.expect(Token::RightParen) {
                    return Err(s);
                }
                return inner_expr;
            }
            _ => return Err(format!("Unexpected token in E block: {:?}", next)),
        }
    }

    fn D(&mut self) -> Result<Node, String> {
        let next = self.iter.peek().unwrap();
        // println!("D Token: {:#?}", next);
        match *next {
            Token::Not => {
                self.iter.next();
                // let node = self.E()?;
                match self.E() {
                    Ok(node) => {
                        return Ok(Node::UnaryExpr {
                            op: UnaryOperator::Not { symbol: '¬' },
                            child: Box::new(node),
                        });
                    }
                    Err(s) => return Err(s),
                }
                // let node = self.E()?;
                // return Ok(Node::UnaryExpr {
                //     op: UnaryOperator::Not { symbol: '¬' },
                //     child: Box::new(node),
                // });
            }
            _ => {
                return self.E();
            }
        }
    }
    fn C(&mut self) -> Result<Node, String> {
        let mut node = self.D()?;
        // match self.D() {
        //     Ok(mut node) => {}
        //     Err(s) => return Err(s),
        // }
        loop {
            let next = self.iter.peek().unwrap();
            // println!("next: {:#?}", next);
            match *next {
                Token::And => {
                    self.iter.next();
                    node = Node::BinaryExpr {
                        op: BinaryOperator::And { symbol: '∧' },
                        lhs: Box::new(node),
                        rhs: Box::new(self.D()?),
                    };
                }
                Token::End => {
                    break;
                }
                _ => {
                    return Err(String::from("Invalid Formula"));
                }
            }
        }

        return Ok(node);
    }
    fn S(&mut self) -> Result<Node, String> {
        let mut node = self.C()?;
        loop {
            let next = self.iter.peek().unwrap();
            // println!("next: {:#?}", next);
            match *next {
                Token::Or => {
                    self.iter.next();
                    node = Node::BinaryExpr {
                        op: BinaryOperator::Or { symbol: '∨' },
                        lhs: Box::new(node),
                        rhs: Box::new(self.C()?),
                    };
                }
                _ => {
                    break;
                }
            }
        }

        return Ok(node);
    }
    pub fn parse(&mut self) -> Result<Node, String> {
        let ast = self.S();
        return ast;
    }
}
