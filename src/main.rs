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
    let x1 = Lit::pos(Var(1));
    let x2 = Lit::pos(Var(2));
    let mut cnf = CNF::new();
    cnf.add_clause(Clause::from(vec![x1, x2]));
    cnf.add_clause(Clause::from(vec![x1.not(), x2]));
    println!("cnf:\n{}", cnf);
    let models = DPLL::new().get_all_models(cnf);
    for model in models {
        println!("model:\n{}", model);
    }
}
