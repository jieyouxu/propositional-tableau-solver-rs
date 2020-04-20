//! Abstract syntax tree representation of a well-formed propositional formula.

pub mod operators;
pub mod propositional_formula;
pub mod variable;

// Re-export propositional formula operators and variables.
pub use operators::{BinaryOperator, Operator, UnaryOperator};
pub use propositional_formula::PropositionalFormula;
pub use variable::Variable;
