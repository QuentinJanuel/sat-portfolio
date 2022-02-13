use crate::cnf::Lit;

#[derive(Clone)]
pub struct Model(pub Vec<Lit>);

impl Model {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn add(&mut self, lit: Lit) {
        self.0.push(lit);
    }
}

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
