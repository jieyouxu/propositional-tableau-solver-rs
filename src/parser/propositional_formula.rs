//! Parser for a propositional formula.

use super::operators::{
    and_operator, biimplication_operator, implication_operator, negation_operator, or_operator,
};
use super::variable::variable;
use super::ParseResult;

use libprop_sat_solver::formula::{BinaryOperator, PropositionalFormula};
use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::sequence::{preceded, separated_pair, terminated};

/// Parses a string into a propositional variable.
///
/// # Example
///
/// ```
/// use prop_sat_solver::parser::propositional_variable;
/// use libprop_sat_solver::formula::{PropositionalFormula, Variable};
/// let input = "foo";
/// let (_, variable) = propositional_variable(input)?;
/// assert_eq!(Variable::new("foo"), variable);
/// ```
pub fn propositional_variable(input: &str) -> ParseResult<&str, PropositionalFormula> {
    let (remaining_input, variable) = variable(input)?;
    let formula = PropositionalFormula::variable(variable);
    Ok((remaining_input, formula))
}

/// Parser for spaces (excluding newline delimiters, which are used to separate multiple
/// propositional formulas.
///
/// The characters
///
/// - Tab: `\t`
/// - Space: ` `
///
/// count as space.
pub fn space(input: &str) -> ParseResult<&str, &str> {
    let space_chars = " \t";
    take_while(move |c| space_chars.contains(c))(input)
}

/// Generic wrapper to generate a parser to match some `( <inner-content> )` with surrounding
/// parentheses, allowing space delimiters before, between and after the components, where the
/// `inner_parser` is responsible for matching the `<inner-content>` part.
pub fn paired_parentheses<'a, R, P>(inner_parser: P) -> impl Fn(&'a str) -> ParseResult<&'a str, R>
where
    P: Fn(&'a str) -> ParseResult<&'a str, R>,
{
    preceded(
        char('('),
        terminated(preceded(space, inner_parser), preceded(space, char(')'))),
    )
}

/// Parser for a propositional negated formula: `( - <propositional-formula> )`
pub fn negated_formula(input: &str) -> ParseResult<&str, PropositionalFormula> {
    let (remaining_input, sub_formula) =
        paired_parentheses(preceded(negation_operator, propositional_formula))(input)?;

    Ok((
        remaining_input,
        PropositionalFormula::negated(Box::new(sub_formula)),
    ))
}

/// Generic binary formula parser.
///
/// # Parameters
///
/// - `main_connective_parser`: a parser which is responsible for recognizing the main connective of
///   the binary propositional formula. Examples of this includes `^` in `(a ^ a)` where as the
///   negation operator is a _unary_ operator and is _not_ a main connective.
/// - `value_constructor_fn`: a mapping function from the two sub formulas `( <left> <op> <right> )`
///   which maps `(Box<PropositionalFormula>, Box<PropositionalFormula>) -> PropositionalFormula`,
///   corresponding to the bottom-up construction of `(<left>, <right>) => <propositional-formula>`.
///
/// # Implementation Notes
///
/// I wanted to make this a higher-order function which generates a new closure that, given the
/// input string `input`, will:
///
/// 1. Parse the form `( <left> <op> <right> )` and discard any whitespaces, the parentheses, the
///    main connective.
/// 2. Generate the desired `PropositionalFormula` value from the `<left>` and `<right>`
///    sub-formulas.
pub fn parse_binary_formula(
    main_connective_parser: fn(&str) -> ParseResult<&str, BinaryOperator>,
    value_constructor_fn: fn(
        Box<PropositionalFormula>,
        Box<PropositionalFormula>,
    ) -> PropositionalFormula,
) -> impl Fn(&str) -> ParseResult<&str, PropositionalFormula> {
    move |input| {
        let (remaining_input, (left_sub_formula, right_sub_formula)) =
            paired_parentheses(separated_pair(
                preceded(space, propositional_formula),
                preceded(space, main_connective_parser),
                preceded(space, propositional_formula),
            ))(input)?;

        Ok((
            remaining_input,
            value_constructor_fn(Box::new(left_sub_formula), Box::new(right_sub_formula)),
        ))
    }
}

/// Parser for a propositional formula with logical AND as the main connective.
pub fn conjunction_formula(input: &str) -> ParseResult<&str, PropositionalFormula> {
    parse_binary_formula(and_operator, PropositionalFormula::conjunction)(input)
}

/// Parser for a formula with logical OR as the main connective.
pub fn disjunction_formula(input: &str) -> ParseResult<&str, PropositionalFormula> {
    parse_binary_formula(or_operator, PropositionalFormula::disjunction)(input)
}

/// Parser for a propositional formula with implication as the main connective.
pub fn implication_formula(input: &str) -> ParseResult<&str, PropositionalFormula> {
    parse_binary_formula(implication_operator, PropositionalFormula::implication)(input)
}

/// Parser for a propositional formula with biimplication as the main connective.
pub fn biimplication_formula(input: &str) -> ParseResult<&str, PropositionalFormula> {
    parse_binary_formula(biimplication_operator, PropositionalFormula::biimplication)(input)
}

/// Parser for a propositional formula.
///
/// This is the root parser for a single propositional formula.
pub fn propositional_formula(input: &str) -> ParseResult<&str, PropositionalFormula> {
    alt((
        propositional_variable,
        negated_formula,
        conjunction_formula,
        disjunction_formula,
        implication_formula,
        biimplication_formula,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use libprop_sat_solver::formula::Variable;

    #[test]
    fn test_space() {
        check!(("", " \t") == space(" \t").unwrap());
    }

    #[test]
    fn simple_propositional_variable_formula() {
        let expected_formula = PropositionalFormula::variable(Variable::new("a"));
        check!(("", expected_formula) == propositional_variable("a").unwrap());
    }

    #[test]
    fn negated_formula() {
        let expected_formula = PropositionalFormula::negated(Box::new(
            PropositionalFormula::variable(Variable::new("a")),
        ));
        check!(("", expected_formula) == propositional_formula("(-a)").unwrap());
    }

    #[test]
    fn conjunction_formula() {
        let left_sub_formula = PropositionalFormula::variable(Variable::new("a"));
        let right_sub_formula = PropositionalFormula::variable(Variable::new("b"));
        let expected_formula = PropositionalFormula::conjunction(
            Box::new(left_sub_formula),
            Box::new(right_sub_formula),
        );
        check!(("", expected_formula) == propositional_formula("(a^b)").unwrap());
    }

    #[test]
    fn disjunction_formula() {
        let left_sub_formula = PropositionalFormula::variable(Variable::new("a"));
        let right_sub_formula = PropositionalFormula::variable(Variable::new("b"));
        let expected_formula = PropositionalFormula::disjunction(
            Box::new(left_sub_formula),
            Box::new(right_sub_formula),
        );
        check!(("", expected_formula) == propositional_formula("(a|b)").unwrap());
    }

    #[test]
    fn implication_formula() {
        let left_sub_formula = PropositionalFormula::variable(Variable::new("a"));
        let right_sub_formula = PropositionalFormula::variable(Variable::new("b"));
        let expected_formula = PropositionalFormula::implication(
            Box::new(left_sub_formula),
            Box::new(right_sub_formula),
        );
        check!(("", expected_formula) == propositional_formula("(a->b)").unwrap());
    }

    #[test]
    fn bimplication_formula() {
        let left_sub_formula = PropositionalFormula::variable(Variable::new("a"));
        let right_sub_formula = PropositionalFormula::variable(Variable::new("b"));
        let expected_formula = PropositionalFormula::biimplication(
            Box::new(left_sub_formula),
            Box::new(right_sub_formula),
        );
        check!(("", expected_formula) == propositional_formula("(a<->b)").unwrap());
    }
}
