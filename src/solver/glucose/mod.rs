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

/// The Glucose solver
pub struct Glucose {
    preprocessing: bool,
    syrup: bool,
}

impl Glucose {
    pub fn new() -> Self {
        Glucose {
            preprocessing: false,
            syrup: false,
        }
    }
    pub fn enable_preprocessing(&mut self) {
        self.preprocessing = true;
    }
}

#[cfg(not(target_os = "windows"))]
impl Glucose {
    pub fn enable_syrup(&mut self) {
        self.syrup = true;
    }
}

impl Solver for Glucose {
    fn solve_with_config(
        &self,
        cnf: &CNF,
        config: &Config,
    ) -> Option<Model> {
        // Uses the C bindings to create a glucose solver,
        // fill it with the CNF, and solve the CNF
        unsafe {
            let ptr = bindings::glucose_new(
                self.preprocessing as i32,
                self.syrup as i32,
            );
            let mut m_vars: HashMap<Var, i32> = HashMap::new();
            for clause in cnf.get_clauses() {
                if config.get_kill() { break; }
                bindings::glucose_add_clause_begin(ptr);
                for lit in clause.get_lits() {
                    let m_var = match m_vars.get(&lit.get_var()) {
                        Some(m_var) => *m_var,
                        None => {
                            let m_lit = bindings::glucose_new_lit(ptr);
                            let m_var = bindings::glucose_lit_to_var(m_lit);
                            m_vars.insert(lit.get_var(), m_var);
                            m_var
                        },
                    };
                    let mut m_lit = bindings::glucose_make_lit(m_var);
                    if !lit.get_sign() {
                        m_lit = bindings::glucose_negate_lit(m_lit);
                    }
                    bindings::glucose_add_clause_add_lit(ptr, m_lit);
                }
                if bindings::glucose_add_clause_commit(ptr) == 0 {
                    return None;
                }
            }
            let sat = if config.get_kill() {
                false
            } else {
                bindings::glucose_solve(ptr) != 0
            };
            let model = if sat {
                let mut model = Model::new();
                for (var, m_var) in m_vars {
                    let lbool = bindings::glucose_model_value_var(
                        ptr,
                        m_var,
                    );
                    if lbool == bindings::glucose_ltrue {
                        model.add(Lit::pos(var));
                    } else if lbool == bindings::glucose_lfalse {
                        model.add(Lit::neg(var));
                    } else {
                        model.add(Lit::pos(var));
                    }
                }
                Some(model)
            } else {
                None
            };
            bindings::glucose_delete(ptr);
            model
        }
    }
}
