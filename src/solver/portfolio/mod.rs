use super::{
    Solver,
    Model,
    config::Config,
};
use crate::cnf::CNF;
use std::{
    thread,
    sync::{mpsc, Arc},
};

/// A portfolio of SAT solvers
pub struct Portfolio {
    solvers: Vec<Arc<dyn Solver>>,
}

impl Portfolio {
    /// Creates a new portfolio of solvers with the given solvers
    pub fn from(solvers: Vec<Arc<dyn Solver>>) -> Self {
        if solvers.len() == 0 {
            panic!("No solvers provided");
        }
        Portfolio { solvers }
    }
}

impl Solver for Portfolio {
    fn solve_with_config(
        &self,
        cnf: &CNF,
        config: &Config,
    ) -> Option<Model> {
        // If there is only one solver, just use it
        if self.solvers.len() == 1 {
            return self.solvers[0].solve_with_config(cnf, config);
        }
        // Starts all the subsolvers in parallel
        // and returns the first result
        let cnf = Arc::new(cnf.clone());
        // Every subsolver will have this config
        let subconfig = Config::default();
        let (tx, rx) = mpsc::channel();
        for solver in &self.solvers {
            // Spawn a thread for each subsolver
            let tx = tx.clone();
            let solver = Arc::clone(solver);
            let cnf = Arc::clone(&cnf);
            let subconfig = subconfig.clone();
            thread::spawn(move || {
                let res = solver.solve_with_config(
                    &*cnf,
                    &subconfig,
                );
                tx.send(res).ok();
            });
        }
        let handle = {
            // This checker thread will kill all the subsolvers
            // if the main config requires a kill
            let tx = tx.clone();
            let thread_config = config.clone();
            let thread_subconfig = subconfig.clone();
            thread::spawn(move || {
                loop {
                    // If the subsolvers are killed, we are done
                    if thread_subconfig.get_kill() {
                        break;
                    }
                    // Kill detected, kill all the subsolvers
                    if thread_config.get_kill() {
                        thread_subconfig.kill();
                    }
                    thread::yield_now();
                }
                // Unlock the receiver
                tx.send(None).ok();
            })
        };
        // Wait for the first response
        let res = rx.recv().unwrap();
        // Kill the checker thread and all the subsolvers
        subconfig.kill();
        handle.join().unwrap();
        res
    }
}

// A nice macro to create a portfolio of solvers
#[macro_export]
macro_rules! portfolio {
    ($($solver:expr),+ $(,)?) => (
        {
            use $crate::solver::portfolio::Portfolio;
            Portfolio::from(vec![
                $(std::sync::Arc::new($solver)),+
            ])
        }
    );
}
