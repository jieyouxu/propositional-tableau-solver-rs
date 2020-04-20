//! Parser combinators for unary and binary operators.

use super::ParseResult;

use libprop_sat_solver::formula::{BinaryOperator, UnaryOperator};

use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::value;

/// Parses the negation operator.
pub fn negation_operator(input: &str) -> ParseResult<&str, UnaryOperator> {
    value(UnaryOperator::Negation, char('-'))(input)
}

/// Parses the logical AND operator.
pub fn and_operator(input: &str) -> ParseResult<&str, BinaryOperator> {
    value(BinaryOperator::And, char('^'))(input)
}

/// Parses the logical OR operator.
pub fn or_operator(input: &str) -> ParseResult<&str, BinaryOperator> {
    value(BinaryOperator::Or, char('|'))(input)
}

/// Parses the implication operator.
pub fn implication_operator(input: &str) -> ParseResult<&str, BinaryOperator> {
    value(BinaryOperator::Implication, tag("->"))(input)
}

/// Parses the biimplication operator.
pub fn biimplication_operator(input: &str) -> ParseResult<&str, BinaryOperator> {
    value(BinaryOperator::Biimplication, tag("<->"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_negation() {
        check!(("", UnaryOperator::Negation) == negation_operator("-").unwrap());
    }

    #[test]
    fn test_and_operator() {
        check!(("", BinaryOperator::And) == and_operator("^").unwrap());
    }

    #[test]
    fn test_or_operator() {
        check!(("", BinaryOperator::Or) == or_operator("|").unwrap());
    }

    #[test]
    fn test_implication_operator() {
        check!(("", BinaryOperator::Implication) == implication_operator("->").unwrap());
    }

    #[test]
    fn test_biimplication_operator() {
        check!(("", BinaryOperator::Biimplication) == biimplication_operator("<->").unwrap());
    }
}
