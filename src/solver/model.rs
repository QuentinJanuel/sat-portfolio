use crate::cnf::{
    Clause,
    Lit,
    Var,
    CNF,
};

/// Represents a model of a CNF formula.
/// i.e. a list of literals with truth values.
#[derive(Clone)]
pub struct Model(Vec<Lit>);

impl Model {
    /// Creates a new model with no literals.
    pub fn new() -> Self {
        Self(Vec::new())
    }
    /// Adds the given literal to the model.
    pub fn add(&mut self, lit: Lit) {
        if self.contains(lit.get_var()) {
            panic!("Model already contains literal");
        }
        self.0.push(lit);
    }
    /// Updates the sign of a given variable in the model.
    pub fn set_pos(&mut self, var: Var, pos: bool) {
        let lit = if pos {
            Lit::pos(var)
        } else {
            Lit::neg(var)
        };
        for l in self.0.iter_mut() {
            if l.get_var() == var {
                *l = lit;
                return;
            }
        }
        self.add(lit);
    }
    /// Returns true if the given variable is already defined
    /// in the model.
    pub fn contains(&self, var: Var) -> bool {
        self.0
            .iter()
            .any(|l| l.get_var() == var)
    }
    /// Generates the simplest clause that prevents this model
    /// from being a model of a CNF formula.
    /// vars: The list of variables we are interested in.
    /// If None, all variables
    pub fn get_prevent_clause(&self, vars: Option<&Vec<Var>>) -> Clause {
        let lits = if let Some(vars) = vars {
            self.0.iter()
                .filter(|l| vars.contains(&l.get_var()))
                .map(|l| l.not())
                .collect()
        } else {
            self.0.iter()
                .map(|l| l.not())
                .collect()
        };
        Clause::from(lits)
    }
    /// Returns the list of literals
    pub fn get_literals(&self) -> &Vec<Lit> {
        &self.0
    }
    /// Returns the list of positive variables
    pub fn get_pos_vars(&self) -> Vec<Var> {
        self.0
            .iter()
            .filter_map(|l|
                if l.get_sign() {
                    Some(l.get_var())
                } else {
                    None
                })
            .collect()
    }
    /// Returns the list of negative variables
    pub fn get_neg_vars(&self) -> Vec<Var> {
        self.0
            .iter()
            .filter_map(|l|
                if l.get_sign() {
                    None
                } else {
                    Some(l.get_var())
                })
            .collect()
    }
    /// Checks if the model is a model of the given CNF formula.
    pub fn is_model_of(&self, cnf: &CNF) -> bool {
        let pos_vars = self.get_pos_vars();
        cnf.get_clauses()
            .iter()
            .all(|c| {
                // Check if self is a model of the clause c
                // It is if any of its literals is satisfied
                c.get_lits()
                    .iter()
                    .any(|lit| {
                        let is_in_model = pos_vars.contains(&lit.get_var());
                        // The literal is satisfied
                        // if is is positive and in pos_vars
                        // or negative and not in pos_vars
                        if lit.get_sign() {
                            is_in_model
                        } else {
                            !is_in_model
                        }
                    })
            })
    }
}

// Display
impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0
            .iter()
            .map(|l| format!("{}", l))
            .collect::<Vec<_>>()
            .join(" ")
        )
    }
}
