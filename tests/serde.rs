#![cfg(not(tarpaulin_include))]
#![cfg(feature = "serde")]

use evalexpr::{Node, build_operator_tree};

#[test]
fn test_serde() {
    let strings = ["3", "4+4", "21^(2*2)--3>5||!true"];

    for string in &strings {
        let manual_tree = build_operator_tree(string).unwrap();
        let serde_tree: Node = ron::de::from_str(&format!("\"{}\"", string)).unwrap();
        assert_eq!(manual_tree.eval(), serde_tree.eval());
    }
}

#[test]
fn test_serde_errors() {
    assert_eq!(
        ron::de::from_str::<Node>("[\"5==5\"]"),
        Err(ron::de::SpannedError {
            code: ron::Error::ExpectedString,
            span: ron::de::Span {
                start: ron::de::Position { line: 1, col: 1 },
                end: ron::de::Position { line: 1, col: 1 }
            }
        })
    );
    assert_eq!(
        ron::de::from_str::<Node>("\"&\""),
        Err(ron::de::SpannedError {
            code: ron::Error::Message(
                "Found a partial token '&' that should be followed by another partial token."
                    .to_owned()
            ),
            span: ron::de::Span {
                start: ron::de::Position { line: 1, col: 2 },
                end: ron::de::Position { line: 1, col: 4 }
            }
        })
    );
    // Ensure that this does not panic.
    assert_ne!(
        ron::de::from_str::<Node>("[\"5==5\"]")
            .unwrap_err()
            .to_string(),
        ""
    );
}
