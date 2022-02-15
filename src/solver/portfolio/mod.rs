use super::{
    Solver,
    Model,
};
use crate::cnf::CNF;
use std::thread;
use std::sync::mpsc;

#[derive(Clone)]
pub struct Portfolio {
    solvers: Vec<Box<dyn Solver + Send>>,
}

impl Portfolio {
    pub fn from(solvers: Vec<Box<dyn Solver + Send>>) -> Self {
        if solvers.len() == 0 {
            panic!("No solvers provided");
        }
        Portfolio { solvers }
    }
}

impl Solver for Portfolio {
    fn solve(&self, cnf: &CNF) -> Option<Model> {
        let cnf = cnf.clone();
        let (tx, rx) = mpsc::channel();
        for solver in &self.solvers {
            let tx = tx.clone();
            let solver = solver.clone();
            let cnf = cnf.clone();
            thread::spawn(move || {
                tx.send(solver.solve(&cnf)).unwrap();
            });
        }
        rx.recv().expect("Parallelism failed")
    }
}