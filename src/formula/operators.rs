//! Unary and binary operators for propositional formulas.

/// There are two types of operators in propositional formulas:
/// 1. `UnaryOperator`: arity 1.
/// 2. `BinaryOperator`: arity 2.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Operator {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

/// There is only one basic unary operator, the negation (logical NOT) operator.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum UnaryOperator {
    Negation,
}

/// There are four basic binary operators:
/// 1. Logical AND.
/// 2. Logical OR.
/// 3. Implication.
/// 4. Biimplication.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum BinaryOperator {
    And,
    Or,
    Implication,
    Biimplication,
}
