//! Parser combinators for unary and binary operators.

use super::ParseResult;
use crate::formula::operators::{BinaryOperator, Operator, UnaryOperator};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;

/// Parses a unary operator.
pub fn unary_operator(input: &str) -> ParseResult<&str, UnaryOperator> {
    let negation = char('-');
    map(negation, |_| UnaryOperator::Negation)(input)
}

/// Parses a binary operator.
pub fn binary_operator(input: &str) -> ParseResult<&str, BinaryOperator> {
    let and = adapter_ignore_input(char('^'), BinaryOperator::And);
    let or = adapter_ignore_input(char('|'), BinaryOperator::Or);
    let implication = adapter_ignore_input(tag("->"), BinaryOperator::Implication);
    let biimplication = adapter_ignore_input(tag("<->"), BinaryOperator::Biimplication);

    alt((and, or, implication, biimplication))(input)
}

/// Converts a parser of the form (for some original output type `C`):
///
/// ```no_run
/// Fn(I) -> ParseResult<I, C>
/// ```
///
/// into the desired form (for some desired output type `R`):
///
/// ```no_run
/// Fn(I) -> ParseResult<I, R>
/// ```
///
/// Given that a mapping function `M: Fn(C) -> R` is provided.
#[inline]
fn adapter<I, C, R, M, F>(original: F, mapping_fn: M) -> impl Fn(I) -> ParseResult<I, R>
where
    F: Fn(I) -> ParseResult<I, C>,
    M: Fn(C) -> R,
{
    map(original, mapping_fn)
}

/// Specialization of the `adapter_helper` where the value is independent from the input string.
#[inline]
fn adapter_ignore_input<'a, I, C, R, F>(
    original: F,
    value: R,
) -> impl Fn(I) -> ParseResult<I, R> + 'a
where
    I: 'a,
    C: 'a,
    R: Copy + 'a,
    F: Fn(I) -> ParseResult<I, C> + 'a,
{
    adapter(original, move |_| value)
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
