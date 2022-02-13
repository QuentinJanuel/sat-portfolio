/// Represents an atom in a CNF formula.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var(pub u32);

// Display
impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
