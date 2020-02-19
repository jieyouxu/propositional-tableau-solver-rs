#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum PropositionalFormula {
    Variable(String),
    Negation(Box<PropositionalFormula>),
    Conjunction(Conjunction),
    Disjunction(Disjunction),
    Implication(Implication),
    Biimplication(Biimplication),
}

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

pub(crate) fn parse_formula(formula: &str) -> Option<PropositionalFormula> {
	unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let result = parse_formula("");
        assert!(result.is_none());
    }

    #[test]
    fn blank_string() {
        let result = parse_formula(" \t");
        assert!(result.is_none());
    }
}

