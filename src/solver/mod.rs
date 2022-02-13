mod model;

pub use model::Model;
use crate::cnf::{
    CNF,
    Clause,
};

pub trait Solver {
    fn solve(&self, cnf: CNF) -> Option<Model>;
    fn get_all_models(&self, mut cnf: CNF) -> Vec<Model> {
        let mut models = vec![];
        while let Some(model) = self.solve(cnf.clone()) {
            models.push(model.clone());
            let new_clause = Clause::from(model.0.iter().map(|l| l.not()).collect());
            cnf.add_clause(new_clause);
        }
        models
    }
}
