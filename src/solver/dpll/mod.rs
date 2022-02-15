use crate::cnf::{
    CNF,
    Clause,
    Lit,
    Var,
};
use crate::solver::{
    Model,
    Solver,
};

/// A SAT solver that uses DPLL algorithm.
pub struct DPLL;

impl DPLL {
    /// Creates a new DPLL solver.
    pub fn new() -> Self { Self }
    /// Chooses the next literal to assign
    fn choose_literal(&self, cnf: &CNF) -> Lit {
        cnf.get_clauses()[0].get_lits()[0]
    }
    fn unit_propagation(&self, cnf: &mut CNF, model: &mut Model) {
        while let Some(x) = cnf.find_unit_clause() {
            cnf.remove_clauses_containing(x);
            cnf.remove_lit(x.not());
            model.add(x);
        }
    }
    // A helper function to simplify the code
    fn solve_aux(&self, mut cnf: CNF, vars: &Vec<Var>, mut model: Model) -> Option<Model> {
        self.unit_propagation(&mut cnf, &mut model);
        // If an empty clause exists, the formula is unsatisfiable
        if cnf.has_empty_clause() {
            return None;
        }
        // If there are no clauses, the formula is satisfiable
        if cnf.has_no_clauses() {
            // Assign arbitrarily all variables that are not assigned
            // to complete the model
            for var in vars {
                let var = *var;
                if !model.contains(var) {
                    model.add(Lit::pos(var));
                }
            }
            return Some(model);
        }
        let x = self.choose_literal(&cnf);
        // Create a new CNF formula with the chosen literal added
        let mut cnf1 = cnf.clone();
        cnf1.add_clause(Clause::from(vec![x]));
        // Create a new CNF formula with the negation of the
        // chosen literal added
        let mut cnf2 = cnf;
        cnf2.add_clause(Clause::from(vec![x.not()]));
        // Returns the first solution found, if any
        self.solve_aux(cnf1, vars, model.clone())
            .or_else(|| self.solve_aux(cnf2, vars, model))
    }
}

impl Solver for DPLL {
    fn solve(&self, cnf: &CNF) -> Option<Model> {
        let vars = cnf.get_variables();
        self.solve_aux(cnf.clone(), &vars, Model::new())
    }
}
