use crate::formula::Formula;
use crate::operators::{BinaryOperator, UnaryOperator};
use std::fmt;

#[derive(Debug)]
pub enum Node {
    Leaf(char),
    UnaryExpr {
        op: UnaryOperator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: BinaryOperator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Node::Leaf(symbol) => write!(f, "{}", symbol),
            Node::UnaryExpr { ref op, ref child } => {
                write!(f, "({}", op.symbol())?;
                if let Err(s) = child.fmt(f) {
                    return Err(s);
                }
                write!(f, ")")?;
                return Ok(());
            }
            Node::BinaryExpr {
                ref op,
                ref lhs,
                ref rhs,
            } => {
                write!(f, "(")?;
                if let Err(s) = lhs.fmt(f) {
                    return Err(s);
                };
                write!(f, "{}", op.symbol())?;
                if let Err(s) = rhs.fmt(f) {
                    return Err(s);
                };
                write!(f, ")")?;
                return Ok(());
            }
        }
    }
}

impl Node {
    pub fn eval(&self, f: &Formula) -> bool {
        match *self {
            Node::Leaf(symbol) => f.term_from_symbol(symbol).unwrap().value,
            Node::UnaryExpr { ref op, ref child } => {
                return op.eval(child.eval(f));
            }
            Node::BinaryExpr {
                ref op,
                ref lhs,
                ref rhs,
            } => return op.eval(lhs.eval(f), rhs.eval(f)),
        }
    }
}
