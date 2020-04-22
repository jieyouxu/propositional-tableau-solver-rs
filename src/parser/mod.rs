//! Parser combinators for parsing propositional formulas from strings.

pub mod operators;
pub mod propositional_formula;
pub mod variable;

use libprop_sat_solver::formula::PropositionalFormula;

/// Newtype for [`nom::IResult`] so we don't expose third-party API.
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
///
/// [`nom::IResult`]: https://docs.rs/nom/5.1.1/nom/type.IResult.html
pub type ParseResult<I, O> = nom::IResult<I, O>;

pub fn parse(input: &str) -> Result<PropositionalFormula, String> {
    let (remaining_input, formula) =
        propositional_formula::propositional_formula(input).map_err(|_| "failed to parse input")?;

    if !remaining_input.trim().is_empty() {
        Err(format!("still input remaining: {:?}", &input))
    } else {
        Ok(formula)
    }
}
