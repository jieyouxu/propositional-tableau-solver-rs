//! Propositional variable parser.

use super::ParseResult;
use crate::formula::Variable;

use nom::character::complete::{alphanumeric0, anychar};
use nom::character::is_alphabetic;
use nom::combinator::verify;
use nom::sequence::tuple;

/// Parser for a propositional variable.
///
/// The identifier for a propositional variable is legal if it is matched by the regular expression:
///
/// ```regex
/// [[:alpha:]][[:alnum:]]*
/// ```
///
/// That is, a single alpha character `[a-zA-Z]` followed by zero or more alpha or numeric
/// characters `[a-zA-Z0-9]`.
pub fn variable(input: &str) -> ParseResult<&str, Variable> {
    let leading = verify(anychar, is_alphabetic_char);
    let rest = alphanumeric0;
    let (remaining, (leading, rest)) = tuple((leading, rest))(input)?;

    let mut name = String::new();
    name.push(leading);
    name.push_str(rest);

    Ok((remaining, Variable::new(name)))
}

fn is_alphabetic_char(c: &char) -> bool {
    is_alphabetic(*c as u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn single_alpha() {
        check!(("", Variable::new("a")) == variable("a").unwrap());
    }

    #[test]
    fn mixed() {
        check!(("", Variable::new("a0ax4d")) == variable("a0ax4d").unwrap());
    }

    #[test]
    fn invalid_name_fails() {
        check!(variable("_").is_err());
    }
}
