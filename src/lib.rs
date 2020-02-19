mod tableau;
mod propositional_formula;
mod lexer;

use tableau::build_tableau;

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
