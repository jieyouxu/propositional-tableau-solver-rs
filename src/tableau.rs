use crate::propositional_formula;

use propositional_formula::*;

use std::collections::HashSet;

#[derive(Clone)]
pub(crate) struct Tableau {
    theories: Vec<Theory>,
}

impl Tableau {
    pub(crate) fn new() -> Tableau {
        Tableau {
            theories: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Theory {
    formulas: HashSet<PropositionalFormula>,
}

impl Theory {
    pub(crate) fn new() -> Theory {
        Theory {
            formulas: HashSet::new(),
        }
    }
}

pub(crate) fn build_tableau(formula: &str) -> Option<Tableau> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
}

