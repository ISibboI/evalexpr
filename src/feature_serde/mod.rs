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

#[cfg(test)]
mod tests {
    use std::{
        fmt::{Debug, Formatter, Write},
        marker::PhantomData,
    };

    use serde::de::Visitor;

    use crate::DefaultNumericTypes;

    use super::NodeVisitor;

    #[test]
    fn node_visitor() {
        struct Debugger;

        impl Debug for Debugger {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                NodeVisitor::<DefaultNumericTypes>(PhantomData).expecting(f)
            }
        }

        let mut output = String::new();
        write!(output, "{:?}", Debugger).unwrap();
        assert!(!output.is_empty());
    }
}
