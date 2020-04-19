//! Parser combinators for parsing propositional formulas from strings.

pub mod operators;

/// Newtype for `nom::IResult` so we don't expose third-party API.
///
/// # Type Parameters
///
/// - `I`: input type.
/// - `O`: output type.
///
/// # Error Type
///
/// We default to `nom`'s error type which implements `std::error::Error` so we can use the `?`
/// operator.
pub type ParseResult<I, O> = nom::IResult<I, O>;
