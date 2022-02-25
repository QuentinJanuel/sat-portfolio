mod model;
pub mod dpll;
pub mod minisat;
pub mod manysat;
pub mod glucose;
#[macro_use]
pub mod portfolio;
pub mod config;

// use std::{ops::Deref, sync::Arc};

pub use model::Model;
use crate::cnf::CNF;
use config::{Config, ConfigAll};

/// Represents a SAT solver.
/// The solver can be used either to find a model of a CNF formula,
/// if one exists, or to enumerate all models of a CNF formula.
pub trait Solver: Send + Sync {
    /// Finds a model of the given CNF formula.
    /// Returns None if no model exists.
    fn solve(&self, cnf: &CNF) -> Option<Model> {
        self.solve_with_config(
            cnf,
            &Config::default(),
        )
    }
    /// Same as solve but with more options.
    fn solve_with_config(
        &self,
        cnf: &CNF,
        config: &Config,
    ) -> Option<Model>;
    /// Enumerates all models of the given CNF formula.
    /// cnf: The CNF formula to enumerate.
    /// vars: The list of variables we are interested in.
    /// If None, all variables
    fn get_all_models(&self, cnf: &mut CNF) -> Vec<Model> {
        self.get_all_models_with_config(
            cnf,
            &ConfigAll::default(),
        )
    }
    /// Same as get_all_models but with more options.
    fn get_all_models_with_config(
        &self,
        cnf: &mut CNF,
        config_all: &ConfigAll,
    ) -> Vec<Model> {
        // Default implementation
        let mut models = vec![];
        let solve_config = Config::from_config_all(config_all);
        // While there are still models to enumerate
        while let Some(model) = self.solve_with_config(
            &cnf,
            &solve_config,
        ) {
            // Stop if the solver is killed
            if config_all.get_kill() {
                return vec![];
            }
            // Add the model to the list
            models.push(model.clone());
            // Remove the model from the CNF formula
            cnf.add_clause(model.get_prevent_clause(config_all.get_vars()));
        }
        models
    }
}
