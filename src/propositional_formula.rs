use std::str::FromStr;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum PropositionalFormula {
    Variable(Variable),
    Negation(Negation),
    Conjunction(Conjunction),
    Disjunction(Disjunction),
    Implication(Implication),
    Biimplication(Biimplication),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Variable(String);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Negation(Box<PropositionalFormula>);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Conjunction {
    left: Box<PropositionalFormula>,
    right: Box<PropositionalFormula>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Disjunction {
    left: Box<PropositionalFormula>,
    right: Box<PropositionalFormula>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Implication {
    premise: Box<PropositionalFormula>,
    conclusion: Box<PropositionalFormula>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Biimplication {
    left: Box<PropositionalFormula>,
    right: Box<PropositionalFormula>,
}

impl FromStr for PropositionalFormula {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s.trim().is_empty() {
            return Err(());
        }

        let formula = s.trim();

        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn empty_string() {
        let result = PropositionalFormula::from_str("");
        assert!(result.is_err());
    }

    #[test]
    fn blank_string() {
        let result = PropositionalFormula::from_str(" \t");
        assert!(result.is_err());
    }
}

