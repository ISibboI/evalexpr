#![cfg(feature = "rand")]

use evalexpr::*;

fn assert_expr(expr: &str) {
    assert_eq!(eval(expr), Ok(Value::Boolean(true)))
}

#[test]
fn test_random() {
    for _ in 0..100 {
        assert_expr("random() != random()");
        assert_expr("0 <= random()");
        assert_expr("random() <= 1");
    }
}

#[test]
fn test_random_errors() {
    assert!(eval("random(9)").is_err());
    assert!(eval("random(\"a\", \"b\")").is_err());
}