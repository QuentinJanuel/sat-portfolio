use super::Lit;

/// Represents a clause in a CNF formula.
/// i.e. a disjunction of literals.
#[derive(Clone)]
pub struct Clause {
    /// The literals in the clause.
    lits: Vec<Lit>,
}

impl Clause {
    /// Creates a new clause from the given literals.
    pub fn from(lits: Vec<Lit>) -> Self {
        Self { lits }
    }
    /// Returns the literals in the clause.
    pub fn get_lits(&self) -> &Vec<Lit> {
        &self.lits
    }
    /// Returns a literal if the clause is unit.
    pub fn get_unit_literal(&self) -> Option<Lit> {
        if self.lits.len() == 1 {
            Some(self.lits[0])
        } else {
            None
        }
    }
    /// Returns true if the clause is empty.
    pub fn is_empty(&self) -> bool {
        self.lits.is_empty()
    }
    /// Returns true if the clause contains the given literal.
    pub fn contains(&self, lit: Lit) -> bool {
        self.lits.contains(&lit)
    }
    /// Removes the given literal from the clause.
    pub fn remove(&mut self, lit: Lit) {
        self.lits.retain(|l| l != &lit);
    }
}

// Display
impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self
            .lits
            .iter()
            .map(|l| format!("{}", l))
            .collect::<Vec<_>>()
            .join(" ")
        )
    }
}
