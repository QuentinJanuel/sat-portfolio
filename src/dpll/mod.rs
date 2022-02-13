use crate::cnf::{
    CNF,
    Clause,
    Lit,
};
use crate::solver::{
    Model,
    Solver,
};

pub struct DPLL {
    literals: Vec<Lit>,
}

impl DPLL {
    pub fn new(literals: Vec<Lit>) -> Self {
        Self { literals }
    }
    fn solve_aux(&self, mut cnf: CNF, mut model: Model) -> Option<Model> {
        while let Some(x) = cnf.find_unit_clause() {
            cnf.remove_clauses_containing(&x);
            // Remove not_x from all clauses
            cnf.remove_lit(&x.not());
            model.add(x);
        }
        if cnf.has_empty_clause() {
            return None;
        }
        if cnf.has_no_clauses() {
            for lit in self.literals.iter() {
                if !model.0.contains(&lit) && !model.0.contains(&lit.not()) {
                    model.add(*lit);
                }
            }
            return Some(model.clone());
        }
        // select a literal {X}
        let x = cnf
            .get_clause(0)
            .get_lit(0);
        // cnf1 = cnf + {X}
        let mut cnf1 = cnf.clone();
        cnf1.add_clause(Clause::from(vec![x]));
        // cnf2 = cnf + {!X}
        let mut cnf2 = cnf.clone();
        cnf2.add_clause(Clause::from(vec![x.not()]));
        self.solve_aux(cnf1, model.clone())
            .or_else(|| self.solve_aux(cnf2, model))
    }
}

impl Solver for DPLL {
    fn solve(&self, cnf: CNF) -> Option<Model> {
        self.solve_aux(cnf, Model::new())
    }
}
