use crate::formula::Formula;
use crate::operators::{BinaryOperator, UnaryOperator};

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
