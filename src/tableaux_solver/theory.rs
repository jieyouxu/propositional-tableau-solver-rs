//! A `Theory` is a set of alternative `PropositionalFormula`s, which corresponds to a branch in a
//! tableau tree.

use std::collections::{HashMap, HashSet};

use crate::formula::PropositionalFormula;

use log::debug;

/// A `Theory` is a set of alternative `PropositionalFormula`s.
///
/// It corresponds to one particular branch of the tableau tree.
#[derive(Debug, PartialEq, Clone)]
pub struct Theory {
	formulas: HashSet<PropositionalFormula>,
}

impl Theory {
	/// Construct an empty theory.
	pub fn new() -> Self {
		Self {
			formulas: HashSet::new(),
		}
	}

	/// Construct a `Theory` from a given propositional formula.
	pub fn from_propositional_formula(formula: PropositionalFormula) -> Self {
		let mut formulas: HashSet<PropositionalFormula> = HashSet::new();
		formulas.insert(formula);

		Self { formulas }
	}

	/// Get the formulas.
	pub fn formulas(&self) -> impl Iterator<Item = &PropositionalFormula> {
		self.formulas.iter()
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
		// Mapping from the variable name `&str` to `(has_literal, has_negation)`.
		let mut literal_occurrence_map: HashMap<&str, (bool, bool)> = HashMap::new();

		for formula in &self.formulas {
			if self.check_formula(formula, &mut literal_occurrence_map) {
				return true;
			}
		}

		debug!("for the formulas:\n{:#?}", &self.formulas);
		debug!("construct the HashSet:\n{:#?}", &literal_occurrence_map);
		debug!("the theory contains no contradictions:\n{:#?}", &self);

		// We've gone through the entire collection of formulas in the `Theory` and did not find any
		// contradictions.
		false
	}

	fn check_formula<'a>(
		&self,
		formula: &'a PropositionalFormula,
		literal_occurrence_map: &mut HashMap<&'a str, (bool, bool)>,
	) -> bool {
		match formula {
			PropositionalFormula::Variable(v) => {
				if let Some((has_literal, has_negation)) = literal_occurrence_map.get_mut(v.name())
				{
					if *has_negation {
						// We've already seen the negated literal, and now we have the literal, so
						// we've found a contradiction.
						true
					} else {
						*has_literal = true;
						false
					}
				} else {
					literal_occurrence_map.insert(v.name(), (true, false));
					false
				}
			}
			PropositionalFormula::Negation(Some(f)) => match &**f {
				PropositionalFormula::Variable(v) => {
					if let Some((has_literal, has_negation)) =
						literal_occurrence_map.get_mut(v.name())
					{
						if *has_literal {
							// We've already seen the literal, and now we have the negation, so
							// we've found a contradiction.
							true
						} else {
							*has_negation = true;
							false
						}
					} else {
						literal_occurrence_map.insert(v.name(), (false, true));
						false
					}
				}
				PropositionalFormula::Negation(Some(ref g)) => {
					// Now (-(-A)) == A so we've covered the base cases and we can simply
					// recursively call `self.check_formula()` to handle the inductive cases with
					// deeply nested negated literals.
					self.check_formula(g, literal_occurrence_map)
				}
				_ => false,
			},
			_ => false,
		}
	}

	/// Get a non-literal formula (not a propositional variable or its negation) from the current
	/// `Theory`.
	pub fn get_non_literal_formula(&mut self) -> Option<PropositionalFormula> {
		self.formulas.iter().cloned().find(|f| !f.is_literal())
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::formula::Variable;
	use assert2::check;

	#[test]
	fn test_construction() {
		let theory =
			Theory::from_propositional_formula(PropositionalFormula::variable(Variable::new("a")));

		check!(theory.formulas().count() == 1);
	}

	#[test]
	fn test_get_formulas() {
		let formula_1 = PropositionalFormula::variable(Variable::new("a"));
		let formula_2 = PropositionalFormula::variable(Variable::new("b"));

		let mut theory = Theory::new();
		theory.add(formula_1);
		theory.add(formula_2);

		check!(theory.formulas().count() == 2);
	}

	#[test]
	fn test_add_fresh_formula() {
		let formula_1 = PropositionalFormula::variable(Variable::new("a"));

		let mut theory = Theory::new();
		check!(theory.formulas().count() == 0);

		theory.add(formula_1);
		check!(theory.formulas().count() == 1);
	}

	#[test]
	fn test_add_duplicate_formula() {
		let formula_1 = PropositionalFormula::variable(Variable::new("a"));

		let mut theory = Theory::new();
		check!(theory.formulas().count() == 0);

		theory.add(formula_1.clone());
		check!(theory.formulas().count() == 1);

		theory.add(formula_1.clone());
		check!(theory.formulas().count() == 1);
	}

	#[test]
	fn test_all_fully_expanded() {
		let formula_1 = PropositionalFormula::variable(Variable::new("a"));
		let formula_2 = PropositionalFormula::negated(Box::new(PropositionalFormula::variable(
			Variable::new("b"),
		)));
		let formula_3 = PropositionalFormula::variable(Variable::new("a"));

		let mut theory = Theory::new();
		theory.add(formula_1);
		theory.add(formula_2);
		theory.add(formula_3);

		check!(theory.is_fully_expanded());
	}

	#[test]
	fn test_partially_expanded() {
		let formula_1 = PropositionalFormula::variable(Variable::new("a"));
		let formula_2 = PropositionalFormula::negated(Box::new(PropositionalFormula::conjunction(
			Box::new(PropositionalFormula::variable(Variable::new("b"))),
			Box::new(PropositionalFormula::variable(Variable::new("c"))),
		)));
		let formula_3 = PropositionalFormula::variable(Variable::new("d"));

		let mut theory = Theory::new();
		theory.add(formula_1);
		theory.add(formula_2);
		theory.add(formula_3);

		check!(!theory.is_fully_expanded());
	}

	#[test]
	fn test_none_fully_expanded() {
		let formula_1 =
			PropositionalFormula::negated(Box::new(PropositionalFormula::biimplication(
				Box::new(PropositionalFormula::variable(Variable::new("e"))),
				Box::new(PropositionalFormula::variable(Variable::new("a"))),
			)));
		let formula_2 = PropositionalFormula::negated(Box::new(PropositionalFormula::conjunction(
			Box::new(PropositionalFormula::variable(Variable::new("b"))),
			Box::new(PropositionalFormula::variable(Variable::new("c"))),
		)));
		let formula_3 = PropositionalFormula::negated(Box::new(PropositionalFormula::negated(
			Box::new(PropositionalFormula::variable(Variable::new("f"))),
		)));

		let mut theory = Theory::new();
		theory.add(formula_1);
		theory.add(formula_2);
		theory.add(formula_3);

		check!(!theory.is_fully_expanded());
	}

	#[test]
	fn test_simple_has_contradictions() {
		let literal_a = PropositionalFormula::variable(Variable::new("a"));
		let negated_literal_a = PropositionalFormula::negated(Box::new(literal_a.clone()));

		let mut theory = Theory::new();
		theory.add(literal_a);
		theory.add(negated_literal_a);

		check!(theory.has_contradictions());
	}

	#[test]
	fn test_simple_has_no_contradictions() {
		let literal_a = PropositionalFormula::variable(Variable::new("a"));
		let literal_b = PropositionalFormula::variable(Variable::new("b"));

		let mut theory = Theory::new();
		theory.add(literal_a);
		theory.add(literal_b);

		check!(!theory.has_contradictions());
	}

	#[test]
	fn test_complex_has_contradictions() {
		let literal_a = PropositionalFormula::variable(Variable::new("a"));
		let non_literal_1 =
			PropositionalFormula::negated(Box::new(PropositionalFormula::conjunction(
				Box::new(PropositionalFormula::variable(Variable::new("b"))),
				Box::new(PropositionalFormula::variable(Variable::new("c"))),
			)));
		let literal_d = PropositionalFormula::variable(Variable::new("d"));
		let negated_literal_a = PropositionalFormula::negated(Box::new(
			PropositionalFormula::variable(Variable::new("a")),
		));

		let mut theory = Theory::new();
		theory.add(literal_a);
		theory.add(non_literal_1);
		theory.add(literal_d);
		theory.add(negated_literal_a);

		check!(theory.has_contradictions());
	}

	#[test]
	fn test_complex_has_no_contradictions() {
		let literal_a = PropositionalFormula::variable(Variable::new("a"));
		let non_literal_1 =
			PropositionalFormula::negated(Box::new(PropositionalFormula::conjunction(
				Box::new(PropositionalFormula::variable(Variable::new("b"))),
				Box::new(PropositionalFormula::variable(Variable::new("c"))),
			)));
		let literal_d = PropositionalFormula::variable(Variable::new("d"));
		let negated_literal_f = PropositionalFormula::negated(Box::new(
			PropositionalFormula::variable(Variable::new("f")),
		));

		let mut theory = Theory::new();
		theory.add(literal_a);
		theory.add(non_literal_1);
		theory.add(literal_d);
		theory.add(negated_literal_f);

		check!(!theory.has_contradictions());
	}

	#[test]
	fn test_double_negation_no_contradiction() {
		// { a, --a } should have no contradictions
		let literal_a = PropositionalFormula::variable(Variable::new("a"));
		let double_negated_literal_a =
			PropositionalFormula::negated(Box::new(PropositionalFormula::negated(Box::new(
				PropositionalFormula::variable(Variable::new("a")),
			))));

		let mut theory = Theory::new();
		theory.add(literal_a);
		theory.add(double_negated_literal_a);

		check!(!theory.has_contradictions());
	}

	#[test]
	fn test_recursive_negation_no_contradictions() {
		// { -a, ---a } should have no contradictions
		let negated_literal_a = PropositionalFormula::negated(Box::new(
			PropositionalFormula::variable(Variable::new("a")),
		));
		let triple_negated_literal_a = PropositionalFormula::negated(Box::new(
			PropositionalFormula::negated(Box::new(PropositionalFormula::negated(Box::new(
				PropositionalFormula::variable(Variable::new("a")),
			)))),
		));

		let mut theory = Theory::new();
		theory.add(negated_literal_a);
		theory.add(triple_negated_literal_a);

		check!(!theory.has_contradictions());
	}

	#[test]
	fn test_recursive_negation_has_contradictions() {
		// { -a, ----a } should have contradictions
		let negated_literal_a = PropositionalFormula::negated(Box::new(
			PropositionalFormula::variable(Variable::new("a")),
		));
		let quad_negated_literal_a =
			PropositionalFormula::negated(Box::new(PropositionalFormula::negated(Box::new(
				PropositionalFormula::negated(Box::new(PropositionalFormula::negated(Box::new(
					PropositionalFormula::variable(Variable::new("a")),
				)))),
			))));

		let mut theory = Theory::new();
		theory.add(negated_literal_a);
		theory.add(quad_negated_literal_a);

		check!(theory.has_contradictions());
	}
}
