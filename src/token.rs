#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    RightParen,
    LeftParen,
    Term(char),
    Not,
    And,
    Or,
    // If,
    // Iff,
    End,
}
