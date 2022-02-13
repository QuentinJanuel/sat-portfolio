use super::Var;

#[derive(Clone, PartialEq)]
pub struct Lit {
    pub var: Var,
    pub pos: bool,
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", if self.pos { "" } else { "-" }, self.var)
    }
}

impl Lit {
    pub fn not(&self) -> Lit {
        Lit {
            var: self.var.clone(),
            pos: !self.pos,
        }
    }
}
