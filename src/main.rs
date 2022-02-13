mod cnf;
mod solver;
mod dpll;

use cnf::{
    CNF,
    Clause,
    Lit,
    Var,
};
use dpll::DPLL;
use solver::Solver;

fn main() {
    let x1 = Lit { var: Var(1), pos: true };
    let x2 = Lit { var: Var(2), pos: true };
    let mut cnf = CNF::new();
    cnf.add_clause(Clause::from(vec![x1.clone(), x2.clone()]));
    cnf.add_clause(Clause::from(vec![x1.not(), x2.clone()]));
    println!("cnf:\n{}", cnf);
    let models = DPLL::new(cnf.get_lits())
        .get_all_models(cnf);
    for model in models {
        println!("model:\n{}", model);
    }
}
