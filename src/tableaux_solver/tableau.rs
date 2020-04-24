//! A `Tableau` is a collection of `Theory`-ies. This corresponds to the entire propositional
//! tableau tree, where each `Theory` is a branch (from the root node to each leaf).

use std::collections::VecDeque;

use crate::formula::PropositionalFormula;

use super::Theory;

/// A `Tableau` is a collection of `Theory`-ies. This corresponds to the entire propositional
/// tableau tree, where each `Theory` is a branch (from the root node to each leaf).
///
/// For example, given the tableau (tree)
///
/// ```text
///     (a^b)
///     /   \
///    a     b
/// ```
///
/// There are two branches (hence two `Theory`-ies):
///
/// 1. `{ (a^b), a }`
/// 2. `{ (a^b), b }`
#[derive(Debug, Clone, PartialEq)]
pub struct Tableau {
    theories: VecDeque<Theory>,
}

impl Tableau {
    /// Construct a new `Tableau` with no theories.
    pub fn new() -> Self {
        Self {
            theories: VecDeque::new(),
        }
    }

    /// Construct a `Tableau` with the starting root node being the given propositional formula.
    pub fn from_starting_propositional_formula(formula: PropositionalFormula) -> Self {
        let mut theories = VecDeque::new();
        theories.push_back(Theory::from_propositional_formula(formula));
        Self { theories }
    }

    /// Check if the `Tableau` contains no `Theory`-ies.
    pub fn is_empty(&self) -> bool {
        self.theories.is_empty()
    }

    /// Retrieve a `Theory` from the `Tableau`.
    pub fn pop_theory(&mut self) -> Option<Theory> {
        self.theories.pop_front()
    }

    /// Add a `Theory` to the `Tableau`.
    pub fn push_theory(&mut self, theory: Theory) {
        self.theories.push_back(theory)
    }

    /// Check if the `Tableau` already contains the `Theory`.
    pub fn contains(&self, theory: &Theory) -> bool {
        self.theories.contains(theory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula::Variable;
    use assert2::check;

    #[test]
    fn test_empty_construction() {
        let empty_tab = Tableau::new();
        check!(empty_tab.is_empty());
    }

    #[test]
    fn test_single_construction() {
        let mut single_tab = Tableau::from_starting_propositional_formula(
            PropositionalFormula::variable(Variable::new("a")),
        );

        check!(!single_tab.is_empty());
        check!(single_tab.pop_theory().unwrap().formulas().count() == 1);
    }

    #[test]
    fn test_push_theory() {
        let mut tab = Tableau::new();
        check!(tab.is_empty());

        tab.push_theory(Theory::from_propositional_formula(
            PropositionalFormula::variable(Variable::new("a")),
        ));

        check!(!tab.is_empty());

        let theory = tab.pop_theory().unwrap();

        check!(
            &PropositionalFormula::variable(Variable::new("a"))
                == theory.formulas().next().unwrap()
        );
    }

    #[test]
    fn test_pop_theory() {
        let mut tab = Tableau::from_starting_propositional_formula(PropositionalFormula::variable(
            Variable::new("a"),
        ));
        check!(!tab.is_empty());

        let theory = tab.pop_theory().unwrap();

        check!(
            &PropositionalFormula::variable(Variable::new("a"))
                == theory.formulas().next().unwrap()
        );
    }

    #[test]
    fn test_push_pop_theory() {
        let mut tab = Tableau::new();

        tab.push_theory(Theory::from_propositional_formula(
            PropositionalFormula::variable(Variable::new("a")),
        ));
        let _ = tab.pop_theory();

        check!(tab.is_empty());
    }

    #[test]
    fn test_contains_theory() {
        let tab = Tableau::from_starting_propositional_formula(PropositionalFormula::variable(
            Variable::new("a"),
        ));

        check!(tab.contains(&Theory::from_propositional_formula(
            PropositionalFormula::variable(Variable::new("a"))
        )));
    }

    #[test]
    fn test_does_not_contain_theory() {
        let tab = Tableau::from_starting_propositional_formula(PropositionalFormula::variable(
            Variable::new("a"),
        ));

        check!(!tab.contains(&Theory::from_propositional_formula(
            PropositionalFormula::variable(Variable::new("b"))
        )));
    }
}
