#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    And { symbol: char },
    Or { symbol: char },
    If { symbol: char },
    Iff { symbol: char },
}
impl BinaryOperator {
    pub fn symbol(&self) -> char {
        match *self {
            Self::And { symbol } => symbol,
            Self::Or { symbol } => symbol,
            Self::If { symbol } => symbol,
            Self::Iff { symbol } => symbol,
        }
    }
    pub fn eval(&self, x: bool, y: bool) -> bool {
        match self {
            Self::And { symbol: _ } => x && y,
            Self::Or { symbol: _ } => x || y,
            Self::If { symbol: _ } => !x || y,
            Self::Iff { symbol: _ } => (!x || y) && (!y || x),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    Not { symbol: char },
}
impl UnaryOperator {
    pub fn symbol(&self) -> char {
        match *self {
            Self::Not { symbol } => symbol,
        }
    }
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
