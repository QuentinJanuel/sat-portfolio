use super::Model;

pub enum Result {
    Satisfiable(Model),
    Unsatisfiable,
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Result::Satisfiable(model) => {
                write!(f, "Satisfiable\n{}", model)
            },
            Result::Unsatisfiable => {
                write!(f, "Unsatisfiable")
            },
        }
    }
}
