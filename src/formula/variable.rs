//! Propositional variable.

/// A propositional formula variable.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd)]
pub struct Variable {
    name: String,
}

impl Variable {
    /// Construct a new propositional variable from a given `name`.
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }

    /// Get the name of the propositional variable.
    pub fn name(&self) -> &str {
        &self.name
    }
}
