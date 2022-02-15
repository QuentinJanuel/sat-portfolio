mod bindings {
    #![allow(
        unused,
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
    )]
    include!(concat!(env!("OUT_DIR"), "/minisat-bindings.rs"));
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

#[derive(Clone)]
pub struct Minisat;

impl Minisat {
    pub fn new() -> Self { Self }
}

impl Solver for Minisat {
    fn solve(&self, cnf: &CNF) -> Option<Model> {
        let ptr = unsafe { bindings::minisat_new() };
        unsafe { bindings::minisat_eliminate(ptr, 1) };
        let mut m_vars: HashMap<Var, i32> = HashMap::new();
        for clause in cnf.get_clauses() {
            unsafe { bindings::minisat_addClause_begin(ptr) };
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
                };
            }
            let conflict = unsafe {
                bindings::minisat_addClause_commit(ptr)
            } != 0;
            if !conflict {
                return None;
            }
        }
        let sat = unsafe {
            bindings::minisat_solve_begin(ptr);
            bindings::minisat_solve_commit(ptr)
        } != 0;
        let model = if sat {
            let mut model = Model::new();
            for (var, m_var) in m_vars {
                let val = unsafe { bindings::minisat_modelValue_Var(ptr, m_var) };
                let (ltrue, lfalse) = unsafe {
                    (
                        bindings::minisat_l_True,
                        bindings::minisat_l_False,
                    )
                };
                let lit = if val == ltrue {
                    Lit::pos(var)
                } else if val == lfalse {
                    Lit::neg(var)
                } else {
                    unreachable!();
                };
                model.add(lit);
            }
            Some(model)
        } else {
            None
        };
        unsafe { bindings::minisat_delete(ptr) };
        model
    }
}
