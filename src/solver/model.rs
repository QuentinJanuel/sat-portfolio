use crate::cnf::{
    Clause,
    Lit,
    Var,
};

/// Represents a model of a CNF formula.
/// i.e. a list of literals with truth values.
#[derive(Clone)]
pub struct Model(Vec<Lit>);

impl Model {
    /// Creates a new model with no literals.
    pub fn new() -> Self {
        Self(Vec::new())
    }
    /// Adds the given literal to the model.
    pub fn add(&mut self, lit: Lit) {
        self.0.push(lit);
    }
    /// Returns true if the given variable is already defined
    /// in the model.
    pub fn contains(&self, var: Var) -> bool {
        self.0
            .iter()
            .any(|l| l.get_var() == var)
    }
    /// Generates the simplest clause that prevents this model
    /// from being a model of a CNF formula.
    pub fn get_prevent_clause(&self) -> Clause {
        let lits = self.0
            .iter()
            .map(|l| l.not())
            .collect();
        Clause::from(lits)
    }
}

// Display
impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0
            .iter()
            .map(|l| format!("{}", l))
            .collect::<Vec<_>>()
            .join(" ")
        )
    }
}
