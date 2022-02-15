use super::{
    Solver,
    Model,
};
use crate::cnf::CNF;
use std::thread;
use std::sync::mpsc;

/// A portfolio of SAT solvers
#[derive(Clone)]
pub struct Portfolio {
    solvers: Vec<Box<dyn Solver + Send>>,
}

impl Portfolio {
    /// Creates a new portfolio of solvers with the given solvers
    pub fn from(solvers: Vec<Box<dyn Solver + Send>>) -> Self {
        if solvers.len() == 0 {
            panic!("No solvers provided");
        }
        Portfolio { solvers }
    }
}

impl Solver for Portfolio {
    fn solve(&self, cnf: &CNF) -> Option<Model> {
        // Starts all the solvers in parallel
        // and returns the first result
        let cnf = cnf.clone();
        let (tx, rx) = mpsc::channel();
        for solver in &self.solvers {
            let tx = tx.clone();
            let solver = solver.clone();
            let cnf = cnf.clone();
            thread::spawn(move || {
                tx.send(solver.solve(&cnf))
                    .unwrap_or_else(|err| {
                        // println!("Failed to send: {}", err);
                    });
            });
        }
        rx.recv().expect("Parallelism failed")
    }
}
