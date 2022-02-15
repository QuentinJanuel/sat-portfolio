mod model;
pub mod dpll;
pub mod minisat;

pub use model::Model;
use crate::cnf::{
    CNF,
};

/// Represents a SAT solver.
/// The solver can be used either to find a model of a CNF formula,
/// if one exists, or to enumerate all models of a CNF formula.
pub trait Solver {
    /// Finds a model of the given CNF formula.
    /// Returns None if no model exists.
    fn solve(&self, cnf: &CNF) -> Option<Model>;
    /// Enumerates all models of the given CNF formula.
    fn get_all_models(&self, cnf: &CNF) -> Vec<Model> {
        // Default implementation
        let mut cnf = cnf.clone();
        let mut models = vec![];
        // While there are still models to enumerate
        while let Some(model) = self.solve(&cnf) {
            // Add the model to the list
            models.push(model.clone());
            // Remove the model from the CNF formula
            cnf.add_clause(model.get_prevent_clause());
        }
        models
    }
}
