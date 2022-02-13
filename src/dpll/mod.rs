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

pub struct DPLL;

impl DPLL {
    pub fn new() -> Self { Self }
    fn solve_aux(&self, mut cnf: CNF, vars: &Vec<Var>, mut model: Model) -> Option<Model> {
        while let Some(x) = cnf.find_unit_clause() {
            cnf.remove_clauses_containing(x);
            // Remove not_x from all clauses
            cnf.remove_lit(x.not());
            model.add(x);
        }
        if cnf.has_empty_clause() {
            return None;
        }
        if cnf.has_no_clauses() {
            for var in vars {
                let var = *var;
                if !model.contains(var) {
                    model.add(Lit::pos(var));
                }
            }
            return Some(model);
        }
        // select a literal {X}
        let x = cnf
            .get_clauses()[0]
            .get_lits()[0];
        // cnf1 = cnf + {X}
        let mut cnf1 = cnf.clone();
        cnf1.add_clause(Clause::from(vec![x]));
        // cnf2 = cnf + {!X}
        let mut cnf2 = cnf;
        cnf2.add_clause(Clause::from(vec![x.not()]));
        self.solve_aux(cnf1, vars, model.clone())
            .or_else(|| self.solve_aux(cnf2, vars, model))
    }
}

impl Solver for DPLL {
    fn solve(&self, cnf: CNF) -> Option<Model> {
        let vars = cnf.get_variables();
        self.solve_aux(cnf, &vars, Model::new())
    }
}
