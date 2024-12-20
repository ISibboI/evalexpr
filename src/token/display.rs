#![cfg(not(tarpaulin_include))]

use std::fmt;

use crate::{
    token::{PartialToken, Token},
    value::numeric_types::EvalexprNumericTypes,
};

impl<NumericTypes: EvalexprNumericTypes> fmt::Display for Token<NumericTypes> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::Token::*;
        match self {
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),
            Percent => write!(f, "%"),
            Hat => write!(f, "^"),

            // Logic
            Eq => write!(f, "=="),
            Neq => write!(f, "!="),
            Gt => write!(f, ">"),
            Lt => write!(f, "<"),
            Geq => write!(f, ">="),
            Leq => write!(f, "<="),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
            Not => write!(f, "!"),

            // Precedence
            LBrace => write!(f, "("),
            RBrace => write!(f, ")"),

            // Assignment
            Assign => write!(f, "="),
            PlusAssign => write!(f, "+="),
            MinusAssign => write!(f, "-="),
            StarAssign => write!(f, "*="),
            SlashAssign => write!(f, "/="),
            PercentAssign => write!(f, "%="),
            HatAssign => write!(f, "^="),
            AndAssign => write!(f, "&&="),
            OrAssign => write!(f, "||="),

            // Special
            Comma => write!(f, ","),
            Semicolon => write!(f, ";"),

            // Values => write!(f, ""), Variables and Functions
            Identifier(identifier) => identifier.fmt(f),
            Float(float) => float.fmt(f),
            Int(int) => int.fmt(f),
            Boolean(boolean) => boolean.fmt(f),
            String(string) => fmt::Debug::fmt(string, f),
        }
    }
}

impl<NumericTypes: EvalexprNumericTypes> fmt::Display for PartialToken<NumericTypes> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::PartialToken::*;
        match self {
            Token(token) => token.fmt(f),
            Literal(literal) => literal.fmt(f),
            Whitespace => write!(f, " "),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Star => write!(f, "*"),
            Slash => write!(f, "/"),
            Percent => write!(f, "%"),
            Hat => write!(f, "^"),
            Eq => write!(f, "="),
            ExclamationMark => write!(f, "!"),
            Gt => write!(f, ">"),
            Lt => write!(f, "<"),
            Ampersand => write!(f, "&"),
            VerticalBar => write!(f, "|"),
        }
    }
}
