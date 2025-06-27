#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Eq, Ord, Hash)]
pub enum Constant {
    Pi,
    E,
    Tau,
}

impl Constant {
    pub fn evaluate(&self) -> f64 {
        match self {
            Constant::Pi => std::f64::consts::PI,
            Constant::E => std::f64::consts::E,
            Constant::Tau => std::f64::consts::TAU,
        }
    }
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Constant::Pi => write!(f, "pi"),
            Constant::E => write!(f, "e"),
            Constant::Tau => write!(f, "tau"),
        }
    }
}

