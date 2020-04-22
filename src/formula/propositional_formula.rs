//! A propositional formula.

use std::convert::{From, Into};

use super::Variable;

/// A propositional formula is defined inductively, conforming to the following BNF:
///
/// ```ebnf
/// <formula>
///     ::= <propositional-variable>
///     | ( - <formula> )
///     | ( <formula> ^ <formula>)
///     | ( <formula> | <formula> )
///     | ( <formula> -> <formula> )
///     | ( <formula> <-> <formula> )
/// ```
///
/// Notice the requirement for explicit parentheses around the unary and binary operators, which
/// eliminates the requirement for operator precedence due to grammar ambiguity at the cost of being
/// more verbose.
///
/// # Ownership, Interior Mutability and Optional Sub-formulas
///
/// Since we don't need any fancy multiple-threading or multi-owner business, we'll stick with the
/// most trivial `Box` pointer indirection instead of the fancier alternatives:
///
/// - Reference-counted smart pointer `Rc`
/// - Atomically-reference-counted smart pointer `Arc`
/// - Lifetime-bounded references `&'a`
///
/// We do not support interior mutability as we do not need it for our use cases with respect to the
/// propositional formula AST.
///
/// We need sub-formulas to be wrapped in `Option` for construction purposes, so we can build a
/// `PropPropositionalFormula` during parsing.
///
/// # No Default
///
/// We cannot soundly define a sane default for a `PropositionalFormula` – even in the base case of
/// a single propositional variable, what would the default propositional variable be?
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub enum PropositionalFormula {
    /// Base case: a single propositional variable.
    Variable(Variable),
    /// Unary case: negated formula.
    Negation(Option<Box<PropositionalFormula>>),
    /// Binary formula with the main connective being the logical AND connective.
    Conjunction(
        Option<Box<PropositionalFormula>>,
        Option<Box<PropositionalFormula>>,
    ),
    /// Binary formula with the main connective being the logical OR operator.
    Disjunction(
        Option<Box<PropositionalFormula>>,
        Option<Box<PropositionalFormula>>,
    ),
    /// Binary formula with the main connective being the implication operator.
    Implication(
        Option<Box<PropositionalFormula>>,
        Option<Box<PropositionalFormula>>,
    ),
    /// Binary formula with the main connective being the biimplication operator.
    Biimplication(
        Option<Box<PropositionalFormula>>,
        Option<Box<PropositionalFormula>>,
    ),
}

// Convenience methods for constructing a `PropositionalFormula`.
//
// Inspired by the blog post at https://endler.dev/2017/boxes-and-trees/.
impl PropositionalFormula {
    /// Construct a new propositional formula from a propositional `Variable`.
    ///
    /// # Example
    ///
    /// ```
    /// use libprop_sat_solver::formula::{PropositionalFormula, Variable};
    /// let formula = PropositionalFormula::variable(Variable::new("a"));
    /// println!("{:#?}", formula);
    /// ```
    #[inline]
    pub fn variable(v: Variable) -> Self {
        Self::Variable(v)
    }

    /// Construct a new propositional formula from a sub propositional formula with negation.
    ///
    /// # Example
    ///
    /// ```
    /// use libprop_sat_solver::formula::{PropositionalFormula, Variable};
    /// let sub_formula = PropositionalFormula::variable(Variable::new("a"));
    /// let formula = PropositionalFormula::negated(Box::new(sub_formula.clone()));
    /// println!("{:#?}", formula);
    /// ```
    #[inline]
    pub fn negated(formula: Box<PropositionalFormula>) -> Self {
        Self::Negation(Some(formula))
    }

    /// Construct a new propositional formula from two propositional sub-formulas with a conjunction
    /// main connective.
    ///
    /// # Example
    ///
    /// ```
    /// use libprop_sat_solver::formula::{PropositionalFormula, Variable};
    /// let sub_formula = PropositionalFormula::variable(Variable::new("a"));
    /// let formula = PropositionalFormula::conjunction(Box::new(sub_formula.clone()), Box::new(sub_formula.clone()));
    /// println!("{:#?}", formula);
    /// ```
    #[inline]
    pub fn conjunction(
        left_sub_formula: Box<PropositionalFormula>,
        right_sub_formula: Box<PropositionalFormula>,
    ) -> Self {
        Self::Conjunction(Some(left_sub_formula), Some(right_sub_formula))
    }

    /// Construct a new propositional formula from two propositional sub-formulas with a disjunction
    /// main connective.
    ///
    /// # Example
    ///
    /// ```
    /// use libprop_sat_solver::formula::{PropositionalFormula, Variable};
    /// let sub_formula = PropositionalFormula::variable(Variable::new("a"));
    /// let formula = PropositionalFormula::disjunction(Box::new(sub_formula.clone()), Box::new(sub_formula.clone()));
    /// println!("{:#?}", formula);
    /// ```
    #[inline]
    pub fn disjunction(
        left_sub_formula: Box<PropositionalFormula>,
        right_sub_formula: Box<PropositionalFormula>,
    ) -> Self {
        Self::Disjunction(Some(left_sub_formula), Some(right_sub_formula))
    }

    /// Construct a new propositional formula from two propositional sub-formulas with an
    /// implication main connective.
    ///
    /// # Example
    ///
    /// ```
    /// use libprop_sat_solver::formula::{PropositionalFormula, Variable};
    /// let sub_formula = PropositionalFormula::variable(Variable::new("a"));
    /// let formula = PropositionalFormula::implication(Box::new(sub_formula.clone()), Box::new(sub_formula.clone()));
    /// println!("{:#?}", formula);
    /// ```
    #[inline]
    pub fn implication(
        left_sub_formula: Box<PropositionalFormula>,
        right_sub_formula: Box<PropositionalFormula>,
    ) -> Self {
        Self::Implication(Some(left_sub_formula), Some(right_sub_formula))
    }

    /// Construct a new propositional formula from two propositional sub-formulas with a
    /// biimplication main connective.
    ///
    /// # Example
    ///
    /// ```
    /// use libprop_sat_solver::formula::{PropositionalFormula, Variable};
    /// let sub_formula = PropositionalFormula::variable(Variable::new("a"));
    /// let formula = PropositionalFormula::biimplication(Box::new(sub_formula.clone()), Box::new(sub_formula.clone()));
    /// println!("{:#?}", formula);
    /// ```
    #[inline]
    pub fn biimplication(
        left_sub_formula: Box<PropositionalFormula>,
        right_sub_formula: Box<PropositionalFormula>,
    ) -> Self {
        Self::Biimplication(Some(left_sub_formula), Some(right_sub_formula))
    }

    /// Checks if the given `PropositionalFormula` is a literal (either a propositional variable
    /// like `p` or its negation `-p`).
    pub fn is_literal(&self) -> bool {
        match self {
            // A propositional variable `p` is trivially a literal.
            Self::Variable(_) => true,
            // The negation of a propositional variable `(-p)` is also trivially a literal.
            Self::Negation(Some(ref inner_formula)) if inner_formula.is_literal() => true,
            // Any other complex propositional formula is not a literal.
            _ => false,
        }
    }
}

impl<V> From<V> for PropositionalFormula
where
    V: Into<Variable>,
{
    fn from(v: V) -> Self {
        Self::Variable(v.into())
    }
}
