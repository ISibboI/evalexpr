#![cfg(not(tarpaulin_include))]
#![cfg(feature = "serde")]

use evalexpr::{build_operator_tree, Node};

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
        Err(ron::Error {
            code: ron::de::ErrorCode::ExpectedString,
            position: ron::de::Position { col: 1, line: 1 }
        })
    );
    assert_eq!(
        ron::de::from_str::<Node>("\"&\""),
        Err(ron::Error {
            code: ron::de::ErrorCode::Message(
                "Found a partial token '&' that should be followed by another partial token."
                    .to_owned()
            ),
            position: ron::de::Position { line: 0, col: 0 }
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
