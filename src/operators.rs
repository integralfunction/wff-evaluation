#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    And { symbol: char },
    Or { symbol: char },
}
impl BinaryOperator {
    pub fn eval(&self, x: bool, y: bool) -> bool {
        match self {
            Self::And { symbol: _ } => x && y,
            Self::Or { symbol: _ } => x || y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Not { symbol: char },
}
impl UnaryOperator {
    pub fn eval(&self, x: bool) -> bool {
        match self {
            Self::Not { symbol: _ } => !x,
        }
    }
}

// #[derive(Debug)]
// pub enum Operator {
//     Unary(UnaryOperator),
//     Binary(BinaryOperator),
// }
