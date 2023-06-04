use crate::{Node, Operator};
use std::fmt::{Display, Error, Formatter};

fn write_binary(f: &mut Formatter, node: &Node, op: &str) -> Result<(), Error> {
    write!(
        f,
        "({} {} {})",
        node.children.get(0).ok_or(Error)?,
        op,
        node.children.get(1).ok_or(Error)?
    )
}

fn write_unary(f: &mut Formatter, node: &Node, op: &str) -> Result<(), Error> {
    write!(f, "{}{}", op, node.children.get(0).ok_or(Error)?,)
}


fn write_sequence(f: &mut Formatter, node: &Node, sep: &str) -> Result<(), Error> {
    for (i, c) in node.children.iter().enumerate() {
        write!(f, "{}", c)?;
        if i + 1 < node.children.len() {
            write!(f, "{} ", sep)?;
        }
    }
    Ok(())
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match &self.operator {
            Operator::RootNode => write_sequence(f, self, ""),
            Operator::Add => write_binary(f, self, "+"),
            Operator::Sub => write_binary(f, self, "-"),
            Operator::Neg => write_unary(f, self, "-"),
            Operator::Mul => write_binary(f, self, "*"),
            Operator::Div => write_binary(f, self, "/"),
            Operator::Mod => write_binary(f, self, "%"),
            Operator::Exp => write_binary(f, self, "^"),
            Operator::Eq => write_binary(f, self, "=="),
            Operator::Neq => write_binary(f, self, "!="),
            Operator::Gt => write_binary(f, self, ">"),
            Operator::Lt => write_binary(f, self, "<"),
            Operator::Geq => write_binary(f, self, ">="),
            Operator::Leq => write_binary(f, self, "<="),
            Operator::And => write_binary(f, self, "&&"),
            Operator::Or => write_binary(f, self, "||"),
            Operator::Not => write_unary(f, self, "!"),
            Operator::Assign => write_binary(f, self, "="),
            Operator::AddAssign => write_binary(f, self, "+="),
            Operator::SubAssign => write_binary(f, self, "-="),
            Operator::MulAssign => write_binary(f, self, "*="),
            Operator::DivAssign => write_binary(f, self, "/="),
            Operator::ModAssign => write_binary(f, self, "%="),
            Operator::ExpAssign => write_binary(f, self, "^="),
            Operator::AndAssign => write_binary(f, self, "&&="),
            Operator::OrAssign => write_binary(f, self, "||="),
            Operator::Tuple => {
                write!(f, "(")?;
                write_sequence(f, self, ",")?;
                write!(f, ")")
            },
            Operator::Chain => write_sequence(f, self, ";"),
            Operator::Const { value } => write!(f, "{}", value),
            Operator::VariableIdentifierWrite { identifier } => {
                write!(f, "{}", identifier)
            },
            Operator::VariableIdentifierRead { identifier } => {
                write!(f, "{}", identifier)
            },
            Operator::FunctionIdentifier { identifier } => {
                write!(f, "{} ", identifier)?;
                for (i, c) in self.children.iter().enumerate() {
                    write!(f, "{}", c)?;
                    if i + 1 < self.children.len() {
                        write!(f, " ")?;
                    }
                }
                Ok(())
            },
        }
    }
}
