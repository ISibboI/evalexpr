use operator::*;
use std::fmt::{Display, Error, Formatter};

impl Display for RootNode {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

impl Display for Add {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "+")
    }
}

impl Display for Sub {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "-")
    }
}

impl Display for Neg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "-")
    }
}

impl Display for Mul {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "*")
    }
}

impl Display for Div {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "/")
    }
}

impl Display for Mod {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "%")
    }
}

impl Display for Eq {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "==")
    }
}

impl Display for Neq {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "!=")
    }
}

impl Display for Gt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, ">")
    }
}

impl Display for Lt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<")
    }
}

impl Display for Geq {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, ">=")
    }
}

impl Display for Leq {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<=")
    }
}

impl Display for And {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "&&")
    }
}

impl Display for Or {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "||")
    }
}

impl Display for Not {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "!")
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, ", ")
    }
}

impl Display for Const {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.value)
    }
}

impl Display for VariableIdentifier {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.identifier)
    }
}

impl Display for FunctionIdentifier {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.identifier)
    }
}
