//! A `Theory` is a set of alternative `PropositionalFormula`s, which corresponds to a branch in a
//! tableau tree.

use std::collections::HashSet;

use crate::formula::PropositionalFormula;

/// A `Theory` is a set of alternative `PropositionalFormula`s.
///
/// It corresponds to one particular branch of the tableau tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Theory {
    formulas: HashSet<PropositionalFormula>,
}

impl Theory {
    /// Construct a `Theory` from a given propositional formula.
    pub fn from_propositional_formula(formula: PropositionalFormula) -> Self {
        let mut formulas = HashSet::new();
        formulas.insert(formula);

        Self { formulas }
    }

    /// Add a propositional formula to the theory iff the theory does not already contain the
    /// formula.
    pub fn add(&mut self, formula: PropositionalFormula) {
        self.formulas.insert(formula);
    }

    /// Checks if the `Theory` is _fully expanded_, i.e. each propositional_formula in the given
    /// `Theory` is a _literal_ (e.g. `p`, `-(p)`, a propositional variable or its negation).
    pub fn is_fully_expanded(&self) -> bool {
        self.formulas.iter().all(PropositionalFormula::is_literal)
    }

    /// Checks if a `Theory` contains _contradictions_. That is, if the `Theory` contains a literal
    /// `p` AND its negation `-p`.
    ///
    /// # Space and Time Complexity
    ///
    /// This function uses a [`HashMap`] (specifically, a map from some `&str` to the tuple
    /// `(has_literal, has_negation): (bool, bool)`. As soon as we encounter the case where
    /// `has_literal && has_negation` then we have found a _contradiction_.
    ///
    /// - Worst-case time complexity: `O(n)` because we iterate through all of the formulas
    ///   for the given theory.
    /// - Worst-case space complexity: `O(k)` for `k` propositional variables appearing in literals.
    ///
    /// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
    pub fn has_contradictions(&self) -> bool {
        use std::collections::HashMap;
        // Mapping from the variableb name `&str` to `(has_literal, has_negation)`.
        let mut literal_occurrence_map: HashMap<&str, (bool, bool)> = HashMap::new();

        for formula in &self.formulas {
            match formula {
                PropositionalFormula::Variable(ref v) => {
                    match literal_occurrence_map.get_mut(v.name()) {
                        Some((_, has_negation)) if *has_negation => {
                            // We already encountered the negation to `v`, so now we've found a
                            // contradiction.
                            return true;
                        }
                        Some((has_literal, _)) => {
                            *has_literal = true;
                        }
                        None => {
                            // Insert new occurrence entry with `(has_literal = true, ..)`.
                            literal_occurrence_map.insert(v.name(), (true, false));
                        }
                    }
                }
                PropositionalFormula::Negation(ref inner_formula) => {
                    if let Some(PropositionalFormula::Variable(v)) = inner_formula.as_deref() {
                        match literal_occurrence_map.get_mut(v.name()) {
                            Some((has_literal, _)) if *has_literal => {
                                // We already encountered the literal `v` to `-v`, so now we've
                                // found a contradiction.
                                return true;
                            }
                            Some((_, has_negation)) => {
                                *has_negation = true;
                            }
                            None => {
                                // Insert new occurrence entry with `(.., has_negation = true)`.
                                literal_occurrence_map.insert(v.name(), (false, true));
                            }
                        }
                    }

                    // The formula is not a negated propositional variable.
                }
                _ => {
                    // The formula is _not_ a literal, simply skip.
                }
            }
        }

        // We've gone through the entire collection of formulas in the `Theory` and did not find any
        // contradictions.
        false
    }

    /// Pop a non-literal formula (not a propositional variable or its negation) from the current
    /// `Theory`.
    ///
    /// # Space and Time Complexity
    ///
    /// - Worst-case time complexity: `O(n)`.
    /// - Worst-case space complexity: `O(n)`.
    ///
    /// TODO: This function needs some optimization to reduce the space/time complexity surrounding
    /// the removal of a non_literal from a [`HashSet`].
    ///
    /// [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
    pub fn pop_non_literal_formula(&mut self) -> Option<PropositionalFormula> {
        use std::iter::FromIterator;

        let mut formulas: Vec<PropositionalFormula> = self.formulas.iter().cloned().collect();

        if let Some(index) = formulas.iter().position(|f| !f.is_literal()) {
            // We pop off the first non-literal formula from the current `Theory`.
            let non_literal = formulas.remove(index);
            self.formulas = HashSet::from_iter(formulas.into_iter());
            return Some(non_literal);
        } else {
            None
        }
    }

    /// Replace existing formula with a new formula.
    pub fn swap_formula(
        &mut self,
        existing: &PropositionalFormula,
        replacement: PropositionalFormula,
    ) {
        if self.formulas.remove(existing) {
            self.formulas.insert(replacement);
        }
    }

    /// Replace existing formula with two new formulas.
    pub fn swap_formula2(
        &mut self,
        existing: &PropositionalFormula,
        replacements: (PropositionalFormula, PropositionalFormula),
    ) {
        if self.formulas.remove(existing) {
            self.formulas.insert(replacements.0);
            self.formulas.insert(replacements.1);
        }
    }
}
