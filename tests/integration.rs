extern crate evalexpr;

use evalexpr::{error::*, *};

#[test]
fn test_unary_examples() {
    assert_eq!(eval("3"), Ok(Value::Int(3)));
    assert_eq!(eval("3.3"), Ok(Value::Float(3.3)));
    assert_eq!(eval("true"), Ok(Value::Boolean(true)));
    assert_eq!(eval("false"), Ok(Value::Boolean(false)));
    assert_eq!(
        eval("blub"),
        Err(Error::VariableIdentifierNotFound("blub".to_string()))
    );
    assert_eq!(eval("-3"), Ok(Value::Int(-3)));
    assert_eq!(eval("-3.6"), Ok(Value::Float(-3.6)));
    assert_eq!(eval("----3"), Ok(Value::Int(3)));
}

#[test]
fn test_binary_examples() {
    assert_eq!(eval("1+3"), Ok(Value::Int(4)));
    assert_eq!(eval("3+1"), Ok(Value::Int(4)));
    assert_eq!(eval("3-5"), Ok(Value::Int(-2)));
    assert_eq!(eval("5-3"), Ok(Value::Int(2)));
    assert_eq!(eval("5 / 4"), Ok(Value::Int(1)));
    assert_eq!(eval("5 *3"), Ok(Value::Int(15)));
    assert_eq!(eval("1.0+3"), Ok(Value::Float(4.0)));
    assert_eq!(eval("3.0+1"), Ok(Value::Float(4.0)));
    assert_eq!(eval("3-5.0"), Ok(Value::Float(-2.0)));
    assert_eq!(eval("5-3.0"), Ok(Value::Float(2.0)));
    assert_eq!(eval("5 / 4.0"), Ok(Value::Float(1.25)));
    assert_eq!(eval("5.0 *3"), Ok(Value::Float(15.0)));
    assert_eq!(eval("5.0 *-3"), Ok(Value::Float(-15.0)));
    assert_eq!(eval("5.0 *- 3"), Ok(Value::Float(-15.0)));
    assert_eq!(eval("5.0 * -3"), Ok(Value::Float(-15.0)));
    assert_eq!(eval("5.0 * - 3"), Ok(Value::Float(-15.0)));
    assert_eq!(eval("-5.0 *-3"), Ok(Value::Float(15.0)));
    assert_eq!(eval("3+-1"), Ok(Value::Int(2)));
    assert_eq!(eval("-3-5"), Ok(Value::Int(-8)));
    assert_eq!(eval("-5--3"), Ok(Value::Int(-2)));
}

#[test]
fn test_arithmetic_precedence_examples() {
    assert_eq!(eval("1+3-2"), Ok(Value::Int(2)));
    assert_eq!(eval("3+1*5"), Ok(Value::Int(8)));
    assert_eq!(eval("2*3-5"), Ok(Value::Int(1)));
    assert_eq!(eval("5-3/3"), Ok(Value::Int(4)));
    assert_eq!(eval("5 / 4*2"), Ok(Value::Int(2)));
    assert_eq!(eval("1-5 *3/15"), Ok(Value::Int(0)));
    assert_eq!(eval("15/7/2.0"), Ok(Value::Float(1.0)));
    assert_eq!(eval("15.0/7/2"), Ok(Value::Float(15.0 / 7.0 / 2.0)));
    assert_eq!(eval("15.0/-7/2"), Ok(Value::Float(15.0 / -7.0 / 2.0)));
    assert_eq!(eval("-15.0/7/2"), Ok(Value::Float(-15.0 / 7.0 / 2.0)));
    assert_eq!(eval("-15.0/7/-2"), Ok(Value::Float(-15.0 / 7.0 / -2.0)));
}

#[test]
fn test_braced_examples() {
    assert_eq!(eval("(1)"), Ok(Value::Int(1)));
    assert_eq!(eval("( 1.0 )"), Ok(Value::Float(1.0)));
    assert_eq!(eval("( true)"), Ok(Value::Boolean(true)));
    assert_eq!(eval("( -1 )"), Ok(Value::Int(-1)));
    assert_eq!(eval("-(1)"), Ok(Value::Int(-1)));
    assert_eq!(eval("-(1 + 3) * 7"), Ok(Value::Int(-28)));
    assert_eq!(eval("(1 * 1) - 3"), Ok(Value::Int(-2)));
    assert_eq!(eval("4 / (2 * 2)"), Ok(Value::Int(1)));
    assert_eq!(eval("7/(7/(7/(7/(7/(7)))))"), Ok(Value::Int(1)));
}

#[test]
fn test_mod_examples() {
    assert_eq!(eval("1 % 4"), Ok(Value::Int(1)));
    assert_eq!(eval("6 % 4"), Ok(Value::Int(2)));
    assert_eq!(eval("1 % 4 + 2"), Ok(Value::Int(3)));
}

#[test]
fn test_pow_examples() {
    assert_eq!(eval("1 ^ 4"), Ok(Value::Float(1.0)));
    assert_eq!(eval("6 ^ 4"), Ok(Value::Float(6.0f64.powf(4.0))));
    assert_eq!(eval("1 ^ 4 + 2"), Ok(Value::Float(3.0)));
    assert_eq!(eval("2 ^ (4 + 2)"), Ok(Value::Float(64.0)));
}

#[test]
fn test_boolean_examples() {
    assert_eq!(eval("true && false"), Ok(Value::Boolean(false)));
    assert_eq!(
        eval("true && false || true && true"),
        Ok(Value::Boolean(true))
    );
    assert_eq!(eval("5 > 4 && 1 <= 1"), Ok(Value::Boolean(true)));
    assert_eq!(eval("5.0 <= 4.9 || !(4 > 3.5)"), Ok(Value::Boolean(false)));
}

#[test]
fn test_with_configuration() {
    let mut configuration = HashMapConfiguration::new();
    configuration.insert_variable("tr".to_string(), Value::Boolean(true));
    configuration.insert_variable("fa".to_string(), Value::Boolean(false));
    configuration.insert_variable("five".to_string(), Value::Int(5));
    configuration.insert_variable("six".to_string(), Value::Int(6));
    configuration.insert_variable("half".to_string(), Value::Float(0.5));
    configuration.insert_variable("zero".to_string(), Value::Int(0));

    assert_eq!(
        eval_with_configuration("tr", &configuration),
        Ok(Value::Boolean(true))
    );
    assert_eq!(
        eval_with_configuration("fa", &configuration),
        Ok(Value::Boolean(false))
    );
    assert_eq!(
        eval_with_configuration("tr && false", &configuration),
        Ok(Value::Boolean(false))
    );
    assert_eq!(
        eval_with_configuration("five + six", &configuration),
        Ok(Value::Int(11))
    );
    assert_eq!(
        eval_with_configuration("five * half", &configuration),
        Ok(Value::Float(2.5))
    );
    assert_eq!(
        eval_with_configuration("five < six && true", &configuration),
        Ok(Value::Boolean(true))
    );
}

#[test]
fn test_functions() {
    let mut configuration = HashMapConfiguration::new();
    configuration.insert_function(
        "sub2".to_string(),
        Function::new(
            Some(1),
            Box::new(|arguments| {
                if let Value::Int(int) = arguments[0] {
                    Ok(Value::Int(int - 2))
                } else if let Value::Float(float) = arguments[0] {
                    Ok(Value::Float(float - 2.0))
                } else {
                    Err(Error::expected_number(arguments[0].clone()))
                }
            }),
        ),
    );
    configuration.insert_variable("five".to_string(), Value::Int(5));

    assert_eq!(
        eval_with_configuration("sub2 5", &configuration),
        Ok(Value::Int(3))
    );
    assert_eq!(
        eval_with_configuration("sub2(5)", &configuration),
        Ok(Value::Int(3))
    );
    assert_eq!(
        eval_with_configuration("sub2 five", &configuration),
        Ok(Value::Int(3))
    );
    assert_eq!(
        eval_with_configuration("sub2(five)", &configuration),
        Ok(Value::Int(3))
    );
    assert_eq!(
        eval_with_configuration("sub2(3) + five", &configuration),
        Ok(Value::Int(6))
    );
}

#[test]
fn test_n_ary_functions() {
    let mut configuration = HashMapConfiguration::new();
    configuration.insert_function(
        "sub2",
        Function::new(
            Some(1),
            Box::new(|arguments| {
                if let Value::Int(int) = arguments[0] {
                    Ok(Value::Int(int - 2))
                } else if let Value::Float(float) = arguments[0] {
                    Ok(Value::Float(float - 2.0))
                } else {
                    Err(Error::expected_number(arguments[0].clone()))
                }
            }),
        ),
    );
    configuration.insert_function(
        "avg",
        Function::new(
            Some(2),
            Box::new(|arguments| {
                expect_number(&arguments[0])?;
                expect_number(&arguments[1])?;

                if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
                    Ok(Value::Int((a + b) / 2))
                } else {
                    Ok(Value::Float(
                        (arguments[0].as_float()? + arguments[1].as_float()?) / 2.0,
                    ))
                }
            }),
        ),
    );
    configuration.insert_function(
        "muladd",
        Function::new(
            Some(3),
            Box::new(|arguments| {
                expect_number(&arguments[0])?;
                expect_number(&arguments[1])?;
                expect_number(&arguments[2])?;

                if let (Value::Int(a), Value::Int(b), Value::Int(c)) =
                    (&arguments[0], &arguments[1], &arguments[2])
                {
                    Ok(Value::Int(a * b + c))
                } else {
                    Ok(Value::Float(
                        arguments[0].as_float()? * arguments[1].as_float()?
                            + arguments[2].as_float()?,
                    ))
                }
            }),
        ),
    );
    configuration.insert_function(
        "count",
        Function::new(
            None,
            Box::new(|arguments| Ok(Value::Int(arguments.len() as IntType))),
        ),
    );
    configuration.insert_variable("five".to_string(), Value::Int(5));

    assert_eq!(
        eval_with_configuration("avg(7, 5)", &configuration),
        Ok(Value::Int(6))
    );
    assert_eq!(
        eval_with_configuration("avg(sub2 5, 5)", &configuration),
        Ok(Value::Int(4))
    );
    assert_eq!(
        eval_with_configuration("sub2(avg(3, 6))", &configuration),
        Ok(Value::Int(2))
    );
    assert_eq!(
        eval_with_configuration("sub2 avg(3, 6)", &configuration),
        Ok(Value::Int(2))
    );
    assert_eq!(
        eval_with_configuration("muladd(3, 6, -4)", &configuration),
        Ok(Value::Int(14))
    );
    assert_eq!(
        eval_with_configuration("count()", &configuration),
        Err(Error::WrongOperatorArgumentAmount {
            actual: 0,
            expected: 1
        })
    );
    assert_eq!(
        eval_with_configuration("count(3, 5.5, 2)", &configuration),
        Ok(Value::Int(3))
    );
    assert_eq!(
        eval_with_configuration("count 5", &configuration),
        Ok(Value::Int(1))
    );

    assert_eq!(
        eval_with_configuration("min(4.0, 3)", &configuration),
        Ok(Value::Int(3))
    );
    assert_eq!(
        eval_with_configuration("max(4.0, 3)", &configuration),
        Ok(Value::Float(4.0))
    );
}

#[test]
fn test_errors() {
    assert_eq!(
        eval("-true"),
        Err(Error::expected_number(Value::Boolean(true)))
    );
    assert_eq!(
        eval("1-true"),
        Err(Error::expected_number(Value::Boolean(true)))
    );
    assert_eq!(
        eval("true-"),
        Err(Error::WrongOperatorArgumentAmount {
            actual: 1,
            expected: 2
        })
    );
    assert_eq!(eval("!(()true)"), Err(Error::AppendedToLeafNode));
}

#[test]
fn test_no_panic() {
    assert!(eval(&format!(
        "{} + {}",
        IntType::max_value(),
        IntType::max_value()
    ))
    .is_err());
    assert!(eval(&format!(
        "-{} - {}",
        IntType::max_value(),
        IntType::max_value()
    ))
    .is_err());
    assert!(eval(&format!("-(-{} - 1)", IntType::max_value())).is_err());
    assert!(eval(&format!(
        "{} * {}",
        IntType::max_value(),
        IntType::max_value()
    ))
    .is_err());
    assert!(eval(&format!("{} / {}", IntType::max_value(), 0)).is_err());
    assert!(eval(&format!("{} % {}", IntType::max_value(), 0)).is_err());
    assert!(eval(&format!(
        "{} ^ {}",
        IntType::max_value(),
        IntType::max_value()
    ))
    .is_ok());
}

#[test]
fn test_shortcut_functions() {
    let mut configuration = HashMapConfiguration::new();
    configuration.insert_variable("string", Value::from("a string"));

    // assert_eq!(eval_string("???"));
    assert_eq!(
        eval_string_with_configuration("string", &configuration),
        Ok("a string".to_string())
    );
    assert_eq!(eval_float("3.3"), Ok(3.3));
    assert_eq!(
        eval_float_with_configuration("3.3", &configuration),
        Ok(3.3)
    );
    assert_eq!(eval_int("3"), Ok(3));
    assert_eq!(eval_int_with_configuration("3", &configuration), Ok(3));
    assert_eq!(eval_boolean("true"), Ok(true));
    assert_eq!(
        eval_boolean_with_configuration("true", &configuration),
        Ok(true)
    );
    assert_eq!(eval_tuple("3,3"), Ok(vec![Value::Int(3), Value::Int(3)]));
    assert_eq!(
        eval_tuple_with_configuration("3,3", &configuration),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );

    // assert_eq!(build_operator_tree("???").unwrap().eval_string());
    assert_eq!(
        build_operator_tree("string")
            .unwrap()
            .eval_string_with_configuration(&configuration),
        Ok("a string".to_string())
    );
    assert_eq!(build_operator_tree("3.3").unwrap().eval_float(), Ok(3.3));
    assert_eq!(
        build_operator_tree("3.3")
            .unwrap()
            .eval_float_with_configuration(&configuration),
        Ok(3.3)
    );
    assert_eq!(build_operator_tree("3").unwrap().eval_int(), Ok(3));
    assert_eq!(
        build_operator_tree("3")
            .unwrap()
            .eval_int_with_configuration(&configuration),
        Ok(3)
    );
    assert_eq!(
        build_operator_tree("true").unwrap().eval_boolean(),
        Ok(true)
    );
    assert_eq!(
        build_operator_tree("true")
            .unwrap()
            .eval_boolean_with_configuration(&configuration),
        Ok(true)
    );
    assert_eq!(
        build_operator_tree("3,3").unwrap().eval_tuple(),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        build_operator_tree("3,3")
            .unwrap()
            .eval_tuple_with_configuration(&configuration),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
}
