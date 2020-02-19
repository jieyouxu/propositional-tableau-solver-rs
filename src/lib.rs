use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum SolverError {
    EmptyFormula,
    IllFormedFormula,
    UnknownError,
}

pub fn is_satisfiable(formula: &str) -> Result<bool, SolverError> {
    if formula.is_empty() || formula.trim().is_empty() {
        return Err(SolverError::EmptyFormula);
    };

    let formula = formula.trim();

    let tableau = match build_tableau(formula) {
        Some(tab) => tab,
        None => {
            return Err(SolverError::IllFormedFormula);
        }
    };

    unimplemented!()
}

#[derive(Clone)]
struct Tableau {
    theories: Vec<Theory>,
}

impl Tableau {
    fn new() -> Tableau {
        Tableau {
            theories: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Theory {
    formulas: HashSet<PropositionalFormula>,
}

impl Theory {
    fn new() -> Theory {
        Theory {
            formulas: HashSet::new(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum PropositionalFormula {
    Variable(String),
    Negation(Box<PropositionalFormula>),
    Conjunction(Conjunction),
    Disjunction(Disjunction),
    Implication(Implication),
    Biimplication(Biimplication),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Conjunction {
    left: Box<PropositionalFormula>,
    right: Box<PropositionalFormula>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Disjunction {
    left: Box<PropositionalFormula>,
    right: Box<PropositionalFormula>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Implication {
    premise: Box<PropositionalFormula>,
    conclusion: Box<PropositionalFormula>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Biimplication {
    left: Box<PropositionalFormula>,
    right: Box<PropositionalFormula>,
}

fn build_tableau(_formula: &str) -> Option<Tableau> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let result = is_satisfiable("");
        assert!(result.is_err());
        assert_eq!(SolverError::EmptyFormula, result.err().unwrap());
    }

    #[test]
    fn blank_string() {
        let result = is_satisfiable(" \t");
        assert!(result.is_err());
        assert_eq!(SolverError::EmptyFormula, result.err().unwrap());
    }
}
