use super::Lit;

#[derive(Clone)]
pub struct Clause {
    lits: Vec<Lit>,
}

impl Clause {
    pub fn from(lits: Vec<Lit>) -> Self {
        Self { lits }
    }
    pub fn is_unit(&self) -> bool {
        self.lits.len() == 1
    }
    pub fn is_empty(&self) -> bool {
        self.lits.is_empty()
    }
    pub fn get_unit_lit(&self) -> Lit {
        self.lits[0].clone()
    }
    pub fn contains(&self, lit: &Lit) -> bool {
        self.lits.contains(lit)
    }
    pub fn remove(&mut self, lit: &Lit) {
        self.lits.retain(|l| l != lit);
    }
    pub fn get_lit(&self, index: usize) -> &Lit {
        &self.lits[index]
    }
    pub fn get_lits(&self) -> Vec<Lit> {
        self.lits.clone()
    }
}

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
