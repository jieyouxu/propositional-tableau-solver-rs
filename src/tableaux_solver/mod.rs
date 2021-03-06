//! Propositional formula satisfiability solver using the Propositional Tableaux method.

use crate::formula::PropositionalFormula;

pub mod tableau;
pub mod theory;
pub use tableau::Tableau;
pub use theory::Theory;

use log::debug;

/// Result of expansion using various rules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpansionKind {
    /// The alpha (α) rule is applicable to the forms:
    ///
    /// | Form                   | α1           | α2       |
    /// | ---------------------- | ------------ | -------- |
    /// | `(A^B)`                | `A`          | `B`      |
    /// | `(A<->B)`              | `(A->B)`     | `(B->A)` |
    /// | <code>(-(A\|B))</code> | `(-A)`       | `(-B)`   |
    /// | `(-(A->B))`            | `A`          | `(-B)`   |
    /// | `(-(-A))`              | `A`          | -        |
    Alpha(Box<PropositionalFormula>, Option<Box<PropositionalFormula>>),
    /// The beta (β) rule is applicable to the forms:
    ///
    /// | Form                  | β1         | β2         |
    /// | --------------------- | ---------- | ---------- |
    /// | <code>(A\|B)</code >  | `A`        | `B`        |
    /// | <code>(-(A^B))</code> | `(-A)`     | `(-B)`     |
    /// | `(A->B)`              | `(-A)`     | `B`        |
    /// | `-(A<->B)`            | `(A^(-B))` | `(B^(-A))` |
    Beta(Box<PropositionalFormula>, Box<PropositionalFormula>),
}

/// Checks if the given propositional formula is _satisfiable_.
///
/// # Propositional Tableaux Algorithm
///
/// ## Key Terminology
///
/// | Term        | Meaning                                  |
/// | ----------- | ---------------------------------------- |
/// | `Theory`    | A _set_ of propositional **formulas**.   |
/// | `Tableau`   | A _queue_ of _alternative_ **theories**. |
///
/// ## Core Algorithm
///
/// The original algorithm is given by the pseudocode:
///
/// ```text
/// begin procedure Satisfiable(phi: PropositionalFormula) -> bool
///     // Initialize `phi` as the single element of the single set contained
///     // within the theory queue.
///     Tableau <- [{ phi }]
///
///     while !IsEmpty(Tab) do
///         Theory <- Dequeue(Tableau)
///
///         if FullyExpanded(Theory) && !ContainsContradictions(Theory) then
///             return true
///         else
///             NonLiteral <- PickNonLiteral(Theory)
///
///             ExpansionType <- DetermineExpansionType(NonLiteral)
///
///             match ExpansionType
///                 case ALPHA => {
///                     Theory = Theory[alpha <- {alpha_1, alpha_2}]
///                     if !ContainsContradictions(Theory) && !(Theory in Tableau) then
///                         Enqueue(Theory)
///                     end if
///                 }
///                 case BETA => {
///                     Theory_1 = Theory[beta <- beta_1]
///                     Theory_2 = Theory[beta <- beta_2]
///
///                     if !(Theory_1 in Tableau) && !ContainsContradictions(Theory_1) then
///                         Enqueue(Theory_1)
///                     end if
///
///                     if !(Theory_2 in Tableau) && !ContainsContradictions(Theory_2) then
///                         Enqueue(Theory_2)
///                     end if
///                 }
///             end match
///         end if
///     end while
///
///     assert IsEmpty(Tableau)
///     return false
/// end procedure
/// ```
///
/// Notice that the algorithm performs an optimization for early return by fusing the contradiction
/// checking logic (i.e. determining if a branch closes) with the branch construction logic.
pub fn is_satisfiable(propositional_formula: &PropositionalFormula) -> bool {
    let mut tableau = Tableau::from_starting_propositional_formula(propositional_formula.clone());
    debug!("starting with tableau:\n{:#?}", &tableau);

    while !tableau.is_empty() {
        // PANIC: Cannot panic because a `Theory` always exists if the `Tableau` is non-empty.
        let mut theory = tableau.pop_theory().unwrap();
        debug!("current_theory:\n{:#?}", &theory);

        if theory.is_fully_expanded() && !theory.has_contradictions() {
            // If the theory is:
            //
            // 1. fully expanded (contains only literals); and
            // 2. contains no contradictions; then
            //
            // The branch represented by the theory remains open, and so the tableau remains open
            // too because at least one branch (this branch) remains open, hence the
            // propositional formula is indeed satisfiable.
            return true;
        } else {
            // PANIC: should never panic because we already check that the theory is _not_ fully
            // expanded, hence it must contain _non-literals_.
            let non_literal_formula = theory.get_non_literal_formula().unwrap();
            debug!("current non_literal: {:#?}", &non_literal_formula);

            // PANIC: should never panic because we exhaustively apply expansion rules and ensure
            // that we pass in a _non-literal_ formula.
            match expand_non_literal_formula(&non_literal_formula).unwrap() {
                ExpansionKind::Alpha(literal_1, optional_literal_2) => {
                    debug!(
                        "apply alpha expansion: [LEFT = {:#?}], [RIGHT = {:#?}]",
                        &literal_1, &optional_literal_2
                    );

                    debug!("theory before expansion: {:#?}", &theory);
                    // FIXME: this `clone()` does not behave as the intended deep copy?
                    let mut new_theory = theory.clone();

                    debug!(
                        "new_theory before expansion:\n{:#?}",
                        &new_theory.formulas().collect::<Vec<_>>()
                    );

                    if let Some(literal_2) = optional_literal_2 {
                        new_theory.swap_formula2(&non_literal_formula, (*literal_1, *literal_2));
                    } else {
                        new_theory.swap_formula(&non_literal_formula, *literal_1);
                    }

                    debug!(
                        "new_theory after expansion:\n{:#?}",
                        &new_theory.formulas().collect::<Vec<_>>()
                    );

                    if !tableau.contains(&new_theory) && !new_theory.has_contradictions() {
                        tableau.push_theory(new_theory);
                    }
                }
                ExpansionKind::Beta(literal_1, literal_2) => {
                    let mut new_theory_1 = theory.clone();
                    let mut new_theory_2 = theory.clone();

                    new_theory_1.swap_formula(&non_literal_formula, *literal_1);
                    new_theory_2.swap_formula(&non_literal_formula, *literal_2);

                    if !tableau.contains(&new_theory_1) && !new_theory_1.has_contradictions() {
                        tableau.push_theory(new_theory_1);
                    }

                    if !tableau.contains(&new_theory_2) && !new_theory_2.has_contradictions() {
                        tableau.push_theory(new_theory_2);
                    }
                }
            }
        }
    }

    // An empty tableau means the propositional formula is unsatisfiable, because we fully expanded
    // the propositional formula to construct all possible branches, and all branches close, hence
    // the entire tableau closes.
    false
}

fn expand_non_literal_formula(non_literal: &PropositionalFormula) -> Option<ExpansionKind> {
    match non_literal {
        // (A <op> B) cases:
        //
        // 1. (A^B) => Alpha(A, Some(B)).
        // 2. (A<->B) => Alpha((A->B), Some((B->A))).
        // 3. (A|B) => Beta(A, B).
        // 4. (A->B) => Beta((-A), B).
        PropositionalFormula::Conjunction(Some(a), Some(b)) => {
            return Some(ExpansionKind::Alpha(a.clone(), Some(b.clone())));
        }
        PropositionalFormula::Biimplication(Some(a), Some(b)) => {
            let alpha_1 = PropositionalFormula::implication(a.clone(), b.clone());
            let alpha_2 = PropositionalFormula::implication(a.clone(), b.clone());
            return Some(ExpansionKind::Alpha(
                Box::new(alpha_1),
                Some(Box::new(alpha_2)),
            ));
        }
        PropositionalFormula::Disjunction(Some(a), Some(b)) => {
            return Some(ExpansionKind::Beta(a.clone(), b.clone()));
        }
        PropositionalFormula::Implication(Some(a), Some(b)) => {
            let beta_1 = PropositionalFormula::negated(a.clone());
            return Some(ExpansionKind::Beta(Box::new(beta_1), b.clone()));
        }

        // (-(-A)) case:
        //
        // 1. (-(-A)) => Alpha(A, None).
        //
        // (-(A <op> B)) cases:
        //
        // 1. (-(A|B)) => Alpha((-A), Some((-B))).
        // 2. (-(A^B)) => Beta((-A), (-B)).
        // 3. (-(A->B)) => Alpha(A, Some((-B))).
        // 4. (-(A<->B)) => Beta((A^(-B)), (B^(-A))).
        PropositionalFormula::Negation(Some(f)) => match &**f {
            PropositionalFormula::Negation(Some(a)) => {
                return Some(ExpansionKind::Alpha(a.clone(), None));
            }
            PropositionalFormula::Disjunction(Some(a), Some(b)) => {
                let alpha_1 = PropositionalFormula::negated(a.clone());
                let alpha_2 = PropositionalFormula::negated(b.clone());
                return Some(ExpansionKind::Alpha(
                    Box::new(alpha_1),
                    Some(Box::new(alpha_2)),
                ));
            }
            PropositionalFormula::Conjunction(Some(a), Some(b)) => {
                let beta_1 = PropositionalFormula::negated(a.clone());
                let beta_2 = PropositionalFormula::negated(b.clone());
                return Some(ExpansionKind::Beta(Box::new(beta_1), Box::new(beta_2)));
            }
            PropositionalFormula::Implication(Some(a), Some(b)) => {
                let alpha_2 = PropositionalFormula::negated(b.clone());
                return Some(ExpansionKind::Alpha(a.clone(), Some(Box::new(alpha_2))));
            }
            PropositionalFormula::Biimplication(Some(a), Some(b)) => {
                let beta_1 = PropositionalFormula::conjunction(
                    a.clone(),
                    Box::new(PropositionalFormula::negated(b.clone())),
                );
                let beta_2 = PropositionalFormula::conjunction(
                    b.clone(),
                    Box::new(PropositionalFormula::negated(a.clone())),
                );

                return Some(ExpansionKind::Beta(Box::new(beta_1), Box::new(beta_2)));
            }
            _ => {
                return None;
            }
        },
        _ => {
            return None;
        }
    }
}

/// Checks if a given propositional formula is _valid_.
///
/// This is done by checking that the contrapositive statement: "is `-<formula>` unsatisfiable?"
pub fn is_valid(formula: &PropositionalFormula) -> bool {
    let negated_formula = PropositionalFormula::negated(Box::new(formula.clone()));
    !is_satisfiable(&negated_formula)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula::Variable;
    use assert2::check;

    #[test]
    fn test_propositional_variable() {
        // a
        let formula = PropositionalFormula::variable(Variable::new("a"));

        check!(is_satisfiable(&formula));
        check!(!is_valid(&formula));
    }

    #[test]
    fn test_conjunction_same_variable() {
        // (a^a)
        let formula = PropositionalFormula::conjunction(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
        );

        check!(is_satisfiable(&formula));
        check!(!is_valid(&formula));
    }

    #[test]
    fn test_conjunction_different_variables() {
        // (a^b)
        let formula = PropositionalFormula::conjunction(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::variable(Variable::new("b"))),
        );

        check!(is_satisfiable(&formula));
        check!(!is_valid(&formula));
    }

    #[test]
    fn test_disjunction_same_variable() {
        // (a|a)
        let formula = PropositionalFormula::disjunction(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
        );

        check!(is_satisfiable(&formula));
        check!(!is_valid(&formula));
    }

    #[test]
    fn test_disjunction_different_variables() {
        // (a|b)
        let formula = PropositionalFormula::disjunction(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::variable(Variable::new("b"))),
        );

        check!(is_satisfiable(&formula));
        check!(!is_valid(&formula));
    }

    #[test]
    fn test_implication_different_variables() {
        // (a->b)
        let formula = PropositionalFormula::implication(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::variable(Variable::new("b"))),
        );

        check!(is_satisfiable(&formula));
        check!(!is_valid(&formula));
    }

    #[test]
    fn test_biimplication_different_variables() {
        // (a<->b)
        let formula = PropositionalFormula::biimplication(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::variable(Variable::new("b"))),
        );

        check!(is_satisfiable(&formula));
        check!(!is_valid(&formula));
    }

    #[test]
    fn test_contradiction() {
        // (a^-a)
        let formula = PropositionalFormula::conjunction(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::negated(Box::new(
                PropositionalFormula::variable(Variable::new("a")),
            ))),
        );

        check!(!is_satisfiable(&formula));
        check!(!is_valid(&formula));
    }

    #[test]
    fn test_tautology_disjunction() {
        // (a|(-a))
        let formula = PropositionalFormula::disjunction(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::negated(Box::new(
                PropositionalFormula::variable(Variable::new("a")),
            ))),
        );

        check!(is_satisfiable(&formula));
        check!(is_valid(&formula));
    }

    #[test]
    fn test_tautology_disjunction_nested_negation() {
        // ((-a)|(-(-a)))
        let formula = PropositionalFormula::disjunction(
            Box::new(PropositionalFormula::negated(Box::new(
                PropositionalFormula::variable(Variable::new("a")),
            ))),
            Box::new(PropositionalFormula::negated(Box::new(
                PropositionalFormula::negated(Box::new(PropositionalFormula::variable(
                    Variable::new("a"),
                ))),
            ))),
        );

        check!(is_satisfiable(&formula));
        check!(is_valid(&formula));
    }

    #[test]
    fn test_tautology_implication_literal() {
        // (a->a)
        let formula = PropositionalFormula::implication(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
        );

        check!(is_satisfiable(&formula));
        check!(is_valid(&formula));
    }

    #[test]
    fn test_tautology_implication_negated_literal() {
        // ((-a)->(-a))
        let formula = PropositionalFormula::implication(
            Box::new(PropositionalFormula::negated(Box::new(
                PropositionalFormula::variable(Variable::new("a")),
            ))),
            Box::new(PropositionalFormula::negated(Box::new(
                PropositionalFormula::variable(Variable::new("a")),
            ))),
        );

        check!(is_satisfiable(&formula));
        check!(is_valid(&formula));
    }

    #[test]
    fn test_tautology_biimplication_literal() {
        // (a<->a)
        let formula = PropositionalFormula::biimplication(
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
            Box::new(PropositionalFormula::variable(Variable::new("a"))),
        );

        check!(is_satisfiable(&formula));
        check!(is_valid(&formula));
    }

    #[test]
    fn test_tautology_biimplication_negated_literal() {
        // ((-a)<->(-a))
        let formula = PropositionalFormula::biimplication(
            Box::new(PropositionalFormula::negated(Box::new(
                PropositionalFormula::variable(Variable::new("a")),
            ))),
            Box::new(PropositionalFormula::negated(Box::new(
                PropositionalFormula::variable(Variable::new("a")),
            ))),
        );

        check!(is_satisfiable(&formula));
        check!(is_valid(&formula));
    }
}
