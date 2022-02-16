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

/// The Minisat solver
#[derive(Clone)]
pub struct Minisat;

impl Minisat {
    pub fn new() -> Self { Self }
}

impl Solver for Minisat {
    fn solve_with_config(
        &self,
        cnf: &CNF,
        config: &Config,
    ) -> Option<Model> {
        // Uses the C bindings to create a minisat solver,
        // fill it with the CNF, and solve the CNF
        let ptr = unsafe {
            let ptr = bindings::minisat_new();
            bindings::minisat_eliminate(ptr, 1);
            ptr
        };
        let mut m_vars: HashMap<Var, i32> = HashMap::new();
        for clause in cnf.get_clauses() {
            if config.get_kill() {
                break;
            }
            unsafe {
                bindings::minisat_addClause_begin(ptr);
            }
            for lit in clause.get_lits() {
                unsafe {
                    let m_var = match m_vars.get(&lit.get_var()) {
                        Some(m_var) => *m_var,
                        None => {
                            let m_lit = bindings::minisat_newLit(ptr);
                            let m_var = bindings::minisat_var(m_lit);
                            m_vars.insert(lit.get_var(), m_var);
                            m_var
                        },
                    };
                    let mut m_lit = bindings::minisat_mkLit(m_var);
                    if !lit.get_sign() {
                        m_lit = bindings::minisat_negate(m_lit);
                    }
                    bindings::minisat_addClause_addLit(ptr, m_lit);
                }
            }
            let conflict = unsafe {
                bindings::minisat_addClause_commit(ptr) != 0
            };
            if !conflict {
                return None;
            }
        }
        let sat = if config.get_kill() {
            false
        } else {
            unsafe {
                bindings::minisat_solve_begin(ptr);
                bindings::minisat_solve_commit(ptr) != 0
            }
        };
        let model = if sat {
            let mut model = Model::new();
            for (var, m_var) in m_vars {
                let val = unsafe {
                    let lbool = bindings::minisat_modelValue_Var(
                        ptr,
                        m_var,
                    );
                    if lbool == bindings::minisat_l_True {
                        true
                    } else if lbool == bindings::minisat_l_False {
                        false
                    } else {
                        unreachable!()
                    }
                };
                let lit = if val {
                    Lit::pos(var)
                } else {
                    Lit::neg(var)
                };
                model.add(lit);
            }
            Some(model)
        } else {
            None
        };
        unsafe { bindings::minisat_delete(ptr) }
        model
    }
}
