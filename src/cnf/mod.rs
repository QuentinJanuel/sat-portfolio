mod var;
mod lit;
mod clause;

pub use var::Var;
pub use lit::Lit;
pub use clause::Clause;

#[derive(Clone)]
pub struct CNF {
    clauses: Vec<Clause>,
}

impl CNF {
    pub fn new() -> Self {
        Self {
            clauses: Vec::new(),
        }
    }
    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
    pub fn find_unit_clause(&self) -> Option<Lit> {
        for clause in &self.clauses {
            if clause.is_unit() {
                return Some(clause.get_unit_lit());
            }
        }
        None
    }
    pub fn has_empty_clause(&self) -> bool {
        for clause in &self.clauses {
            if clause.is_empty() {
                return true;
            }
        }
        false
    }
    pub fn remove_clauses_containing(&mut self, lit: &Lit) {
        self.clauses.retain(|clause| {
            !clause.contains(lit)
        });
    }
    pub fn remove_lit(&mut self, lit: &Lit) {
        for clause in &mut self.clauses {
            clause.remove(lit);
        }
    }
    pub fn has_no_clauses(&self) -> bool {
        self.clauses.is_empty()
    }
    pub fn get_clause(&self, index: usize) -> &Clause {
        &self.clauses[index]
    }
    pub fn get_lits(&self) -> Vec<Lit> {
        self.clauses
            .iter()
            .flat_map(|clause| clause.get_lits())
            .collect::<Vec<_>>()
    }
}

impl std::fmt::Display for CNF {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.clauses.iter().map(|c| format!("{}", c)).collect::<Vec<_>>().join("\n"))
    }
}
