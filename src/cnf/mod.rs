mod var;
mod lit;
mod clause;

pub use var::Var;
pub use lit::Lit;
pub use clause::Clause;

/// Represents a CNF formula.
#[derive(Clone)]
pub struct CNF {
    /// The conjunction of clauses.
    clauses: Vec<Clause>,
}

impl CNF {
    /// Creates a new CNF formula with no clauses.
    pub fn new() -> Self {
        Self { clauses: Vec::new() }
    }
    /// Adds the given clause to the CNF formula.
    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }
    /// Finds a unit clause and returns its literal if one exists.
    /// Otherwise returns None.
    pub fn find_unit_clause(&self) -> Option<Lit> {
        for clause in &self.clauses {
            if let Some(lit) = clause.get_unit_literal() {
                return Some(lit);
            }
        }
        None
    }
    /// Returns true if at least one clause is empty.
    pub fn has_empty_clause(&self) -> bool {
        self.clauses.iter().any(|c| c.is_empty())
    }
    /// Removes all clauses containing the given literal.
    pub fn remove_clauses_containing(&mut self, lit: Lit) {
        self.clauses.retain(|c| !c.contains(lit));
    }
    /// Removes the given literal from all clauses.
    pub fn remove_lit(&mut self, lit: Lit) {
        for clause in &mut self.clauses {
            clause.remove(lit);
        }
    }
    /// Returns true if the CNF formula has no clauses.
    pub fn has_no_clauses(&self) -> bool {
        self.clauses.is_empty()
    }
    /// Returns the clauses in the CNF formula.
    pub fn get_clauses(&self) -> &Vec<Clause> {
        &self.clauses
    }
    /// Returns the list of variables in the CNF formula.
    pub fn get_variables(&self) -> Vec<Var> {
        let mut vars = self.clauses
            .iter()
            .flat_map(|clause| clause.get_lits())
            .map(|lit| lit.get_var())
            .collect::<Vec<_>>();
        vars.sort();  // Sort the variables, needed for the next step.
        vars.dedup(); // Remove duplicates.
        vars
    }
}

// Display
impl std::fmt::Display for CNF {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.clauses.iter().map(|c| format!("{}", c)).collect::<Vec<_>>().join("\n"))
    }
}
