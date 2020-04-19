//! Abstract syntax tree representation of a well-formed propositional formula.

pub mod operators;
pub mod variable;

// Re-export propositional formula operators and variables.
pub use operators::{BinaryOperator, Operator, UnaryOperator};
pub use variable::Variable;
