use super::Var;

/// Represents a literal in a CNF formula.
/// i.e. a variable with a sign.
#[derive(Clone, Copy, PartialEq)]
pub struct Lit {
    /// The variable.
    var: Var,
    /// Whether the literal is positive.
    pos: bool,
}

impl Lit {
    /// Creates a positive literal with the given variable.
    pub fn pos(var: Var) -> Self {
        Self {
            var,
            pos: true,
        }
    }
    /// Creates a negative literal with the given variable.
    pub fn neg(var: Var) -> Self {
        Self {
            var,
            pos: false,
        }
    }
    /// Negates the literal (i.e. flips the sign).
    pub fn not(&self) -> Lit {
        Lit {
            var: self.var,
            pos: !self.pos,
        }
    }
    /// Gets the variable of the literal.
    pub fn get_var(&self) -> Var {
        self.var
    }
    /// Gets the positivity of the literal.
    /// i.e. returns true if the literal is positive.
    pub fn get_sign(&self) -> bool {
        self.pos
    }
}

// Display
impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", if self.pos { "" } else { "-" }, self.var)
    }
}
