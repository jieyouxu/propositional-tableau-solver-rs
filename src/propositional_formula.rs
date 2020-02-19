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

