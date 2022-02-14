#[macro_use]
mod cnf;
mod solver;
mod dpll;
#[cfg(test)]
mod tests;

use dpll::DPLL;
use solver::Solver;

fn main() {
    let cnf = cnf![
         1,  2;
        1
    ];
    println!("cnf:\n{}", cnf);
    let models = DPLL::new().get_all_models(cnf);
    println!("models:");
    for model in models {
        println!("{}", model);
    }
}
