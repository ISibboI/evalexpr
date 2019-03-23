use interface::build_operator_tree;
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use ::{Error, Node};

impl<'de> Deserialize<'de> for Node {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NodeVisitor)
    }
}

struct NodeVisitor;

impl<'de> de::Visitor<'de> for NodeVisitor {
    type Value = Node;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "a string in the expression format of the `evalexpr` crate"
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match build_operator_tree(v) {
            Ok(node) => Ok(node),
            Err(error) => Err(E::custom(error)),
        }
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
