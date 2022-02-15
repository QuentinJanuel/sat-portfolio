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

pub struct Minisat {
    solver: *mut bindings::minisat_solver_t,
}

impl Minisat {
    pub fn new() -> Self {
        let s = Self { solver: unsafe { bindings::minisat_new() } };
        unsafe { bindings::minisat_eliminate(s.solver, 1) };
        s
    }
}

impl Solver for Minisat {
    fn solve(&self, cnf: &CNF) -> Option<Model> {
        let mut m_vars: HashMap<Var, i32> = HashMap::new();
        for clause in cnf.get_clauses() {
            unsafe { bindings::minisat_addClause_begin(self.solver) };
            for lit in clause.get_lits() {
                unsafe {
                    let m_var = match m_vars.get(&lit.get_var()) {
                        Some(m_var) => *m_var,
                        None => {
                            let m_lit = bindings::minisat_newLit(self.solver);
                            let m_var = bindings::minisat_var(m_lit);
                            m_vars.insert(lit.get_var(), m_var);
                            m_var
                        },
                    };
                    let mut m_lit = bindings::minisat_mkLit(m_var);
                    if !lit.get_sign() {
                        m_lit = bindings::minisat_negate(m_lit);
                    }
                    bindings::minisat_addClause_addLit(self.solver, m_lit);
                };
            }
            let conflict = unsafe {
                bindings::minisat_addClause_commit(self.solver)
            } != 0;
            if !conflict {
                return None;
            }
        }
        let sat = unsafe {
            bindings::minisat_solve_begin(self.solver);
            bindings::minisat_solve_commit(self.solver)
        } != 0;
        if sat {
            let mut model = Model::new();
            for (var, m_var) in m_vars {
                let val = unsafe { bindings::minisat_modelValue_Var(self.solver, m_var) };
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
        }
    }
}

impl Drop for Minisat {
    fn drop(&mut self) {
        unsafe { bindings::minisat_delete(self.solver) };
    }
}
