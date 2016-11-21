
use std::str::FromStr;
use serde_json::{Value, to_value};
use error::Error;
use node::Node;


#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add(u8),
    Mul(u8),
    Sub(u8),
    Div(u8),
    Rem(u8),
    Not(u8),
    Eq(u8),
    Ne(u8),
    Gt(u8),
    Lt(u8),
    Ge(u8),
    Le(u8),
    And(u8),
    Or(u8),
    Dot(u8),
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket(u8),
    RightSquareBracket,
    DoubleQuotes,
    SingleQuote,
    WhiteSpace,
    Comma,
    Function(String),
    Identifier(String),
    Value(Value),
}

impl Operator {
    pub fn is_identifier(&self) -> bool {
        match *self {
            Operator::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn can_at_beginning(&self) -> bool {
        match *self {
            Operator::Not(_) |
            Operator::Function(_) |
            Operator::LeftParenthesis => true,
            _ => false,
        }
    }

    pub fn get_max_args(&self) -> Option<usize> {
        match *self {
            Operator::Add(_) | Operator::Sub(_) | Operator::Mul(_) | Operator::Div(_) |
            Operator::Eq(_) | Operator::Ne(_) | Operator::Gt(_) | Operator::Lt(_) |
            Operator::Ge(_) | Operator::Le(_) | Operator::And(_) | Operator::Or(_) |
            Operator::Rem(_) => Some(2),
            Operator::Not(_) => Some(1),
            Operator::Function(_) => None,
            _ => Some(0),
        }
    }

    pub fn get_min_args(&self) -> Option<usize> {
        match *self {
            Operator::Add(_) | Operator::Sub(_) | Operator::Mul(_) | Operator::Div(_) |
            Operator::Eq(_) | Operator::Ne(_) | Operator::Gt(_) | Operator::Lt(_) |
            Operator::Ge(_) | Operator::Le(_) | Operator::And(_) | Operator::Or(_) |
            Operator::Rem(_) => Some(2),
            Operator::Not(_) => Some(1),
            Operator::Function(_) => None,
            _ => Some(0),
        }
    }

    pub fn get_priority(&self) -> u8 {
        match *self {
            Operator::Add(priority) |
            Operator::Sub(priority) |
            Operator::Div(priority) |
            Operator::Mul(priority) |
            Operator::Eq(priority) |
            Operator::Ne(priority) |
            Operator::Gt(priority) |
            Operator::Lt(priority) |
            Operator::Ge(priority) |
            Operator::Le(priority) |
            Operator::And(priority) |
            Operator::Or(priority) |
            Operator::Rem(priority) => priority,
            Operator::Value(_) |
            Operator::Identifier(_) => 0,
            _ => 99,
        }
    }

    pub fn is_left_parenthesis(&self) -> bool {
        *self == Operator::LeftParenthesis
    }

    pub fn is_not(&self) -> bool {
        match *self {
            Operator::Not(_) => true,
            _ => false,
        }
    }

    pub fn is_left_square_bracket(&self) -> bool {
        match *self {
            Operator::LeftSquareBracket(_) => true,
            _ => false,
        }
    }

    pub fn is_dot(&self) -> bool {
        match *self {
            Operator::Dot(_) => true,
            _ => false,
        }
    }

    pub fn is_value_or_ident(&self) -> bool {
        match *self {
            Operator::Value(_) |
            Operator::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn can_have_child(&self) -> bool {
        match *self {
            Operator::Function(_) |
            Operator::Add(_) |
            Operator::Sub(_) |
            Operator::Div(_) |
            Operator::Mul(_) |
            Operator::Rem(_) |
            Operator::Eq(_) |
            Operator::Ne(_) |
            Operator::Gt(_) |
            Operator::Lt(_) |
            Operator::And(_) |
            Operator::Or(_) |
            Operator::Ge(_) |
            Operator::Not(_) |
            Operator::Dot(_) |
            Operator::LeftSquareBracket(_) |
            Operator::Le(_) => true,
            _ => false,
        }
    }

    pub fn is_left(&self) -> bool {
        match *self {
            Operator::LeftParenthesis |
            Operator::LeftSquareBracket(_) => true,
            _ => false,
        }
    }

    pub fn get_left(&self) -> Operator {
        match *self {
            Operator::RightParenthesis => Operator::LeftParenthesis,
            Operator::RightSquareBracket => Operator::LeftSquareBracket(100),
            _ => panic!("not bracket"),
        }
    }

    pub fn to_node(&self) -> Node {
        Node::new(self.clone())
    }

    pub fn children_to_node(&self, children: Vec<Node>) -> Node {
        let mut node = self.to_node();
        node.children = children;
        node
    }

    pub fn get_identifier(&self) -> &str {
        match *self {
            Operator::Identifier(ref ident) => ident,
            _ => panic!("not identifier"),
        }
    }
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(raw: &str) -> Result<Operator, Error> {
        match raw {
            "+" => Ok(Operator::Add(8)),
            "-" => Ok(Operator::Sub(8)),
            "*" => Ok(Operator::Mul(10)),
            "/" => Ok(Operator::Div(10)),
            "%" => Ok(Operator::Rem(10)),
            "(" => Ok(Operator::LeftParenthesis),
            ")" => Ok(Operator::RightParenthesis),
            "[" => Ok(Operator::LeftSquareBracket(100)),
            "]" => Ok(Operator::RightSquareBracket),
            "." => Ok(Operator::Dot(100)),
            "\"" => Ok(Operator::DoubleQuotes),
            "'" => Ok(Operator::SingleQuote),
            " " => Ok(Operator::WhiteSpace),
            "," => Ok(Operator::Comma),
            "!" => Ok(Operator::Not(99)),
            "false" => Ok(Operator::Value(to_value(false))),
            "true" => Ok(Operator::Value(to_value(true))),
            "==" => Ok(Operator::Eq(6)),
            "!=" => Ok(Operator::Ne(6)),
            ">" => Ok(Operator::Gt(6)),
            "<" => Ok(Operator::Lt(6)),
            ">=" => Ok(Operator::Ge(6)),
            "<=" => Ok(Operator::Le(6)),
            "&&" => Ok(Operator::And(4)),
            "||" => Ok(Operator::Or(2)),
            _ => Ok(Operator::Identifier(raw.to_owned())),
        }
    }
}
