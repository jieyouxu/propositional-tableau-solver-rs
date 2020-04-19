//! Parser combinators for unary and binary operators.

use super::ParseResult;
use crate::formula::{BinaryOperator, UnaryOperator};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::value;

/// Parses a unary operator.
pub fn unary_operator(input: &str) -> ParseResult<&str, UnaryOperator> {
    let negation = value(UnaryOperator::Negation, char('-'));
    negation(input)
}

/// Parses a binary operator.
pub fn binary_operator(input: &str) -> ParseResult<&str, BinaryOperator> {
    let and = value(BinaryOperator::And, char('^'));
    let or = value(BinaryOperator::Or, char('|'));
    let implication = value(BinaryOperator::Implication, tag("->"));
    let biimplication = value(BinaryOperator::Biimplication, tag("<->"));

    alt((and, or, implication, biimplication))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_unary_operator() {
        check!(("", UnaryOperator::Negation) == unary_operator("-").unwrap());
    }

    #[test]
    fn test_binary_operators() {
        check!(("", BinaryOperator::And) == binary_operator("^").unwrap());
        check!(("", BinaryOperator::Or) == binary_operator("|").unwrap());
        check!(("", BinaryOperator::Implication) == binary_operator("->").unwrap());
        check!(("", BinaryOperator::Biimplication) == binary_operator("<->").unwrap());
    }
}
