mod bindings {
    #![allow(
        unused,
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
    )]
    include!("./bindings.rs");
}

use crate::{
    cnf::{
        CNF,
        Lit,
        Var,
    },
    solver::{
        Solver,
        Model,
    },
};
use std::collections::HashMap;
use super::config::Config;

/// The Maplesat solver
pub struct Maplesat {
    n_threads: u32,
    clause_limit: u32,
}

impl Maplesat {
    pub fn new() -> Self {
        Self {
            n_threads: 4,
            clause_limit: 10,
        }
    }
    pub fn set_n_threads(&mut self, n_threads: u32) {
        self.n_threads = n_threads;
    }
    pub fn set_clause_limit(&mut self, clause_limit: u32) {
        self.clause_limit = clause_limit;
    }
}

impl Solver for Maplesat {
    fn solve_with_config(
        &self,
        cnf: &CNF,
        config: &Config,
    ) -> Option<Model> {
        // Uses the C bindings to create a maplesat solver,
        // fill it with the CNF, and solve the CNF
        unsafe {
            let ptr = bindings::maplesat_new();
            let mut m_vars: HashMap<Var, i32> = HashMap::new();
            for clause in cnf.get_clauses() {
                if config.get_kill() { break; }
                bindings::maplesat_add_clause_begin(ptr);
                for lit in clause.get_lits() {
                    let m_var = match m_vars.get(&lit.get_var()) {
                        Some(m_var) => *m_var,
                        None => {
                            let m_lit = bindings::maplesat_new_lit(ptr);
                            let m_var = bindings::maplesat_lit_to_var(m_lit);
                            m_vars.insert(lit.get_var(), m_var);
                            m_var
                        },
                    };
                    let mut m_lit = bindings::maplesat_make_lit(m_var);
                    if !lit.get_sign() {
                        m_lit = bindings::maplesat_negate_lit(m_lit);
                    }
                    bindings::maplesat_add_clause_add_lit(ptr, m_lit);
                }
                if bindings::maplesat_add_clause_commit(ptr) == 0 {
                    return None;
                }
            }
            let sat = if config.get_kill() {
                false
            } else {
                bindings::maplesat_solve(ptr) != 0
            };
            let model = if sat {
                let mut model = Model::new();
                for (var, m_var) in m_vars {
                    let lbool = bindings::maplesat_model_value_var(
                        ptr,
                        m_var,
                    );
                    if lbool == bindings::maplesat_ltrue {
                        model.add(Lit::pos(var));
                    } else if lbool == bindings::maplesat_lfalse {
                        model.add(Lit::neg(var));
                    } else {
                        unreachable!()
                    }
                }
                Some(model)
            } else {
                None
            };
            bindings::maplesat_delete(ptr);
            model
        }
    }
}
