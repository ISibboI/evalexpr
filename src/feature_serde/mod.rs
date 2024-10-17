use crate::{interface::build_operator_tree, EvalexprNumericTypes, Node};
use serde::{de, Deserialize, Deserializer};
use std::{fmt, marker::PhantomData};

impl<'de, NumericTypes: EvalexprNumericTypes> Deserialize<'de> for Node<NumericTypes> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NodeVisitor(PhantomData))
    }
}

struct NodeVisitor<NumericTypes: EvalexprNumericTypes>(PhantomData<NumericTypes>);

impl<NumericTypes: EvalexprNumericTypes> de::Visitor<'_> for NodeVisitor<NumericTypes> {
    type Value = Node<NumericTypes>;

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
