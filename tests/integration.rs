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
        Err(EvalexprError::VariableIdentifierNotFound(
            "blub".to_string()
        ))
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
fn test_with_context() {
    let mut context = HashMapContext::new();
    context
        .set_value("tr".into(), Value::Boolean(true))
        .unwrap();
    context
        .set_value("fa".into(), Value::Boolean(false))
        .unwrap();
    context.set_value("five".into(), Value::Int(5)).unwrap();
    context.set_value("six".into(), Value::Int(6)).unwrap();
    context.set_value("half".into(), Value::Float(0.5)).unwrap();
    context.set_value("zero".into(), Value::Int(0)).unwrap();

    assert_eq!(eval_with_context("tr", &context), Ok(Value::Boolean(true)));
    assert_eq!(eval_with_context("fa", &context), Ok(Value::Boolean(false)));
    assert_eq!(
        eval_with_context("tr && false", &context),
        Ok(Value::Boolean(false))
    );
    assert_eq!(
        eval_with_context("five + six", &context),
        Ok(Value::Int(11))
    );
    assert_eq!(
        eval_with_context("five * half", &context),
        Ok(Value::Float(2.5))
    );
    assert_eq!(
        eval_with_context("five < six && true", &context),
        Ok(Value::Boolean(true))
    );
}

#[test]
fn test_functions() {
    let mut context = HashMapContext::new();
    context
        .set_function(
            "sub2".to_string(),
            Function::new(|argument| {
                if let Value::Int(int) = argument {
                    Ok(Value::Int(int - 2))
                } else if let Value::Float(float) = argument {
                    Ok(Value::Float(float - 2.0))
                } else {
                    Err(EvalexprError::expected_number(argument.clone()))
                }
            }),
        )
        .unwrap();
    context
        .set_value("five".to_string(), Value::Int(5))
        .unwrap();

    assert_eq!(eval_with_context("sub2 5", &context), Ok(Value::Int(3)));
    assert_eq!(eval_with_context("sub2(5)", &context), Ok(Value::Int(3)));
    assert_eq!(eval_with_context("sub2 five", &context), Ok(Value::Int(3)));
    assert_eq!(eval_with_context("sub2(five)", &context), Ok(Value::Int(3)));
    assert_eq!(
        eval_with_context("sub2(3) + five", &context),
        Ok(Value::Int(6))
    );
}

#[test]
fn test_n_ary_functions() {
    let mut context = HashMapContext::new();
    context
        .set_function(
            "sub2".into(),
            Function::new(|argument| {
                if let Value::Int(int) = argument {
                    Ok(Value::Int(int - 2))
                } else if let Value::Float(float) = argument {
                    Ok(Value::Float(float - 2.0))
                } else {
                    Err(EvalexprError::expected_number(argument.clone()))
                }
            }),
        )
        .unwrap();
    context
        .set_function(
            "avg".into(),
            Function::new(|argument| {
                let arguments = argument.as_tuple()?;
                arguments[0].as_number()?;
                arguments[1].as_number()?;

                if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
                    Ok(Value::Int((a + b) / 2))
                } else {
                    Ok(Value::Float(
                        (arguments[0].as_float()? + arguments[1].as_float()?) / 2.0,
                    ))
                }
            }),
        )
        .unwrap();
    context
        .set_function(
            "muladd".into(),
            Function::new(|argument| {
                let arguments = argument.as_tuple()?;
                arguments[0].as_number()?;
                arguments[1].as_number()?;
                arguments[2].as_number()?;

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
        )
        .unwrap();
    context
        .set_function(
            "count".into(),
            Function::new(|arguments| match arguments {
                Value::Tuple(tuple) => Ok(Value::from(tuple.len() as IntType)),
                Value::Empty => Ok(Value::from(0)),
                _ => Ok(Value::from(1)),
            }),
        )
        .unwrap();
    context
        .set_value("five".to_string(), Value::Int(5))
        .unwrap();
    context
        .set_function("function_four".into(), Function::new(|_| Ok(Value::Int(4))))
        .unwrap();

    assert_eq!(eval_with_context("avg(7, 5)", &context), Ok(Value::Int(6)));
    assert_eq!(
        eval_with_context("avg(sub2 5, 5)", &context),
        Ok(Value::Int(4))
    );
    assert_eq!(
        eval_with_context("sub2(avg(3, 6))", &context),
        Ok(Value::Int(2))
    );
    assert_eq!(
        eval_with_context("sub2 avg(3, 6)", &context),
        Ok(Value::Int(2))
    );
    assert_eq!(
        eval_with_context("muladd(3, 6, -4)", &context),
        Ok(Value::Int(14))
    );
    assert_eq!(eval_with_context("count()", &context), Ok(Value::Int(0)));
    assert_eq!(
        eval_with_context("count((1, 2, 3))", &context),
        Ok(Value::Int(3))
    );
    assert_eq!(
        eval_with_context("count(3, 5.5, 2)", &context),
        Ok(Value::Int(3))
    );
    assert_eq!(eval_with_context("count 5", &context), Ok(Value::Int(1)));
    assert_eq!(
        eval_with_context("function_four()", &context),
        Ok(Value::Int(4))
    );
}

#[test]
fn test_builtin_functions() {
    // Log
    assert_eq!(eval("ln(2.718281828459045)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("log(9, 9)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("log2(2)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("log10(10)"), Ok(Value::Float(1.0)));
    // Cos
    assert_eq!(eval("cos(0)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("acos(1)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("cosh(0)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("acosh(1)"), Ok(Value::Float(0.0)));
    // Sin
    assert_eq!(eval("sin(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("asin(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("sinh(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("asinh(0)"), Ok(Value::Float(0.0)));
    // Tan
    assert_eq!(eval("tan(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("atan(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("tanh(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("atanh(0)"), Ok(Value::Float(0.0)));
    // Root
    assert_eq!(eval("sqrt(25)"), Ok(Value::Float(5.0)));
    assert_eq!(eval("cbrt(8)"), Ok(Value::Float(2.0)));
    // Rounding
    assert_eq!(eval("floor(1.1)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("floor(1.9)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("round(1.1)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("round(1.5)"), Ok(Value::Float(2.0)));
    assert_eq!(eval("round(2.5)"), Ok(Value::Float(3.0)));
    assert_eq!(eval("round(1.9)"), Ok(Value::Float(2.0)));
    assert_eq!(eval("ceil(1.1)"), Ok(Value::Float(2.0)));
    assert_eq!(eval("ceil(1.9)"), Ok(Value::Float(2.0)));
    // Other
    assert_eq!(eval("min(4.0, 3)"), Ok(Value::Int(3)));
    assert_eq!(eval("max(4.0, 3)"), Ok(Value::Float(4.0)));
    assert_eq!(eval("len(\"foobar\")"), Ok(Value::Int(6)));
    assert_eq!(eval("len(\"a\", \"b\")"), Ok(Value::Int(2)));
    // String
    assert_eq!(
        eval("str::to_lowercase(\"FOOBAR\")"),
        Ok(Value::from("foobar"))
    );
    assert_eq!(
        eval("str::to_uppercase(\"foobar\")"),
        Ok(Value::from("FOOBAR"))
    );
    assert_eq!(
        eval("str::trim(\"  foo  bar \")"),
        Ok(Value::from("foo  bar"))
    );
    assert_eq!(eval("str::from(\"a\")"), Ok(Value::String(String::from("\"a\""))));
    assert_eq!(eval("str::from(1.0)"), Ok(Value::String(String::from("1"))));
    assert_eq!(eval("str::from(1)"), Ok(Value::String(String::from("1"))));
    assert_eq!(eval("str::from(true)"), Ok(Value::String(String::from("true"))));
    assert_eq!(eval("str::from(1, 2, 3)"), Ok(Value::String(String::from("(1, 2, 3)"))));
    assert_eq!(eval("str::from()"), Ok(Value::String(String::from("()"))));
}

#[test]
fn test_errors() {
    assert_eq!(
        eval("-true"),
        Err(EvalexprError::expected_number(Value::Boolean(true)))
    );
    assert_eq!(
        eval("1-true"),
        Err(EvalexprError::expected_number(Value::Boolean(true)))
    );
    assert_eq!(
        eval("true-"),
        Err(EvalexprError::WrongOperatorArgumentAmount {
            actual: 1,
            expected: 2
        })
    );
    assert_eq!(eval("!(()true)"), Err(EvalexprError::AppendedToLeafNode));
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
    let mut context = HashMapContext::new();
    context
        .set_value("string".into(), Value::from("a string"))
        .unwrap();

    // assert_eq!(eval_string("???"));
    assert_eq!(
        eval_string_with_context("string", &context),
        Ok("a string".to_string())
    );
    assert_eq!(eval_float("3.3"), Ok(3.3));
    assert_eq!(eval_float_with_context("3.3", &context), Ok(3.3));
    assert_eq!(eval_int("3"), Ok(3));
    assert_eq!(eval_int_with_context("3", &context), Ok(3));
    assert_eq!(eval_number("3"), Ok(3.0));
    assert_eq!(eval_number_with_context("3", &context), Ok(3.0));
    assert_eq!(eval_boolean("true"), Ok(true));
    assert_eq!(eval_boolean_with_context("true", &context), Ok(true));
    assert_eq!(eval_tuple("3,3"), Ok(vec![Value::Int(3), Value::Int(3)]));
    assert_eq!(
        eval_tuple_with_context("3,3", &context),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(eval_empty(""), Ok(EMPTY_VALUE));
    assert_eq!(eval_empty_with_context("", &context), Ok(EMPTY_VALUE));

    // assert_eq!(build_operator_tree("???").unwrap().eval_string());
    assert_eq!(
        build_operator_tree("string")
            .unwrap()
            .eval_string_with_context(&context),
        Ok("a string".to_string())
    );
    assert_eq!(build_operator_tree("3.3").unwrap().eval_float(), Ok(3.3));
    assert_eq!(
        build_operator_tree("3.3")
            .unwrap()
            .eval_float_with_context(&context),
        Ok(3.3)
    );
    assert_eq!(build_operator_tree("3").unwrap().eval_int(), Ok(3));
    assert_eq!(
        build_operator_tree("3")
            .unwrap()
            .eval_int_with_context(&context),
        Ok(3)
    );
    assert_eq!(build_operator_tree("3").unwrap().eval_number(), Ok(3.0));
    assert_eq!(
        build_operator_tree("3")
            .unwrap()
            .eval_number_with_context(&context),
        Ok(3.0)
    );
    assert_eq!(
        build_operator_tree("true").unwrap().eval_boolean(),
        Ok(true)
    );
    assert_eq!(
        build_operator_tree("true")
            .unwrap()
            .eval_boolean_with_context(&context),
        Ok(true)
    );
    assert_eq!(
        build_operator_tree("3,3").unwrap().eval_tuple(),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        build_operator_tree("3,3")
            .unwrap()
            .eval_tuple_with_context(&context),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        build_operator_tree("")
            .unwrap()
            .eval_empty_with_context(&context),
        Ok(EMPTY_VALUE)
    );

    assert_eq!(
        eval_string_with_context_mut("string", &mut context),
        Ok("a string".to_string())
    );
    assert_eq!(eval_float_with_context_mut("3.3", &mut context), Ok(3.3));
    assert_eq!(eval_int_with_context_mut("3", &mut context), Ok(3));
    assert_eq!(eval_number_with_context_mut("3", &mut context), Ok(3.0));
    assert_eq!(
        eval_boolean_with_context_mut("true", &mut context),
        Ok(true)
    );
    assert_eq!(
        eval_tuple_with_context_mut("3,3", &mut context),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        eval_empty_with_context_mut("", &mut context),
        Ok(EMPTY_VALUE)
    );

    assert_eq!(
        build_operator_tree("string")
            .unwrap()
            .eval_string_with_context_mut(&mut context),
        Ok("a string".to_string())
    );
    assert_eq!(
        build_operator_tree("3.3")
            .unwrap()
            .eval_float_with_context_mut(&mut context),
        Ok(3.3)
    );
    assert_eq!(
        build_operator_tree("3")
            .unwrap()
            .eval_int_with_context_mut(&mut context),
        Ok(3)
    );
    assert_eq!(
        build_operator_tree("3")
            .unwrap()
            .eval_number_with_context_mut(&mut context),
        Ok(3.0)
    );
    assert_eq!(
        build_operator_tree("true")
            .unwrap()
            .eval_boolean_with_context_mut(&mut context),
        Ok(true)
    );
    assert_eq!(
        build_operator_tree("3,3")
            .unwrap()
            .eval_tuple_with_context_mut(&mut context),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        build_operator_tree("")
            .unwrap()
            .eval_empty_with_context_mut(&mut context),
        Ok(EMPTY_VALUE)
    );
}

#[test]
fn test_whitespace() {
    assert!(eval_boolean("2 < = 3").is_err());
}

#[test]
fn test_assignment() {
    let mut context = HashMapContext::new();
    assert_eq!(
        eval_empty_with_context_mut("int = 3", &mut context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        eval_empty_with_context_mut("float = 2.0", &mut context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        eval_empty_with_context_mut("tuple = (1,1)", &mut context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        eval_empty_with_context_mut("empty = ()", &mut context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        eval_empty_with_context_mut("boolean = false", &mut context),
        Ok(EMPTY_VALUE)
    );

    assert_eq!(eval_int_with_context("int", &context), Ok(3));
    assert_eq!(eval_float_with_context("float", &context), Ok(2.0));
    assert_eq!(
        eval_tuple_with_context("tuple", &context),
        Ok(vec![1.into(), 1.into()])
    );
    assert_eq!(eval_empty_with_context("empty", &context), Ok(EMPTY_VALUE));
    assert_eq!(eval_boolean_with_context("boolean", &context), Ok(false));

    assert_eq!(
        eval_empty_with_context_mut("b = a = 5", &mut context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(eval_empty_with_context("b", &context), Ok(EMPTY_VALUE));
}

#[test]
fn test_expression_chaining() {
    let mut context = HashMapContext::new();
    assert_eq!(
        eval_int_with_context_mut("a = 5; a = a + 2; a", &mut context),
        Ok(7)
    );
}

#[test]
fn test_strings() {
    let mut context = HashMapContext::new();
    assert_eq!(eval("\"string\""), Ok(Value::from("string")));
    assert_eq!(
        eval_with_context_mut("a = \"a string\"", &mut context),
        Ok(Value::Empty)
    );
    assert_eq!(
        eval_boolean_with_context("a == \"a string\"", &context),
        Ok(true)
    );
    assert_eq!(eval("\"a\" + \"b\""), Ok(Value::from("ab")));
    assert_eq!(eval("\"a\" > \"b\""), Ok(Value::from(false)));
    assert_eq!(eval("\"a\" < \"b\""), Ok(Value::from(true)));
    assert_eq!(eval("\"xa\" > \"xb\""), Ok(Value::from(false)));
    assert_eq!(eval("\"xa\" < \"xb\""), Ok(Value::from(true)));
    assert_eq!(eval("\"{}\" != \"{}\""), Ok(Value::from(false)));
    assert_eq!(eval("\"{}\" == \"{}\""), Ok(Value::from(true)));
}

#[test]
fn test_tuple_definitions() {
    assert_eq!(eval_empty("()"), Ok(()));
    assert_eq!(eval_int("(3)"), Ok(3));
    assert_eq!(
        eval_tuple("(3, 4)"),
        Ok(vec![Value::from(3), Value::from(4)])
    );
    assert_eq!(
        eval_tuple("2, (5, 6)"),
        Ok(vec![
            Value::from(2),
            Value::from(vec![Value::from(5), Value::from(6)])
        ])
    );
    assert_eq!(eval_tuple("1, 2"), Ok(vec![Value::from(1), Value::from(2)]));
    assert_eq!(
        eval_tuple("1, 2, 3, 4"),
        Ok(vec![
            Value::from(1),
            Value::from(2),
            Value::from(3),
            Value::from(4)
        ])
    );
    assert_eq!(
        eval_tuple("(1, 2, 3), 5, 6, (true, false, 0)"),
        Ok(vec![
            Value::from(vec![Value::from(1), Value::from(2), Value::from(3)]),
            Value::from(5),
            Value::from(6),
            Value::from(vec![Value::from(true), Value::from(false), Value::from(0)])
        ])
    );
    assert_eq!(
        eval_tuple("1, (2)"),
        Ok(vec![Value::from(1), Value::from(2)])
    );
    assert_eq!(
        eval_tuple("1, ()"),
        Ok(vec![Value::from(1), Value::from(())])
    );
    assert_eq!(
        eval_tuple("1, ((2))"),
        Ok(vec![Value::from(1), Value::from(2)])
    );
}

#[test]
fn test_implicit_context() {
    assert_eq!(
        eval("a = 2 + 4 * 2; b = -5 + 3 * 5; a == b"),
        Ok(Value::from(true))
    );
    assert_eq!(
        eval_boolean("a = 2 + 4 * 2; b = -5 + 3 * 5; a == b"),
        Ok(true)
    );
    assert_eq!(eval_int("a = 2 + 4 * 2; b = -5 + 3 * 5; a - b"), Ok(0));
    assert_eq!(
        eval_float("a = 2 + 4 * 2; b = -5 + 3 * 5; a - b + 0.5"),
        Ok(0.5)
    );
    assert_eq!(eval_number("a = 2 + 4 * 2; b = -5 + 3 * 5; a - b"), Ok(0.0));
    assert_eq!(eval_empty("a = 2 + 4 * 2; b = -5 + 3 * 5;"), Ok(()));
    assert_eq!(
        eval_tuple("a = 2 + 4 * 2; b = -5 + 3 * 5; a, b + 0.5"),
        Ok(vec![Value::from(10), Value::from(10.5)])
    );
    assert_eq!(
        eval_string("a = \"xyz\"; b = \"abc\"; c = a + b; c"),
        Ok("xyzabc".to_string())
    );
}

#[test]
fn test_operator_assignments() {
    let mut context = HashMapContext::new();
    assert_eq!(eval_empty_with_context_mut("a = 5", &mut context), Ok(()));
    assert_eq!(eval_empty_with_context_mut("a += 5", &mut context), Ok(()));
    assert_eq!(eval_empty_with_context_mut("a -= 5", &mut context), Ok(()));
    assert_eq!(eval_empty_with_context_mut("a *= 5", &mut context), Ok(()));
    assert_eq!(eval_empty_with_context_mut("b = 5.0", &mut context), Ok(()));
    assert_eq!(eval_empty_with_context_mut("b /= 5", &mut context), Ok(()));
    assert_eq!(eval_empty_with_context_mut("b %= 5", &mut context), Ok(()));
    assert_eq!(eval_empty_with_context_mut("b ^= 5", &mut context), Ok(()));
    assert_eq!(
        eval_empty_with_context_mut("c = true", &mut context),
        Ok(())
    );
    assert_eq!(
        eval_empty_with_context_mut("c &&= false", &mut context),
        Ok(())
    );
    assert_eq!(
        eval_empty_with_context_mut("c ||= true", &mut context),
        Ok(())
    );

    let mut context = HashMapContext::new();
    assert_eq!(eval_int_with_context_mut("a = 5; a", &mut context), Ok(5));
    assert_eq!(eval_int_with_context_mut("a += 3; a", &mut context), Ok(8));
    assert_eq!(eval_int_with_context_mut("a -= 5; a", &mut context), Ok(3));
    assert_eq!(eval_int_with_context_mut("a *= 5; a", &mut context), Ok(15));
    assert_eq!(
        eval_float_with_context_mut("b = 5.0; b", &mut context),
        Ok(5.0)
    );
    assert_eq!(
        eval_float_with_context_mut("b /= 2; b", &mut context),
        Ok(2.5)
    );
    assert_eq!(
        eval_float_with_context_mut("b %= 2; b", &mut context),
        Ok(0.5)
    );
    assert_eq!(
        eval_float_with_context_mut("b ^= 2; b", &mut context),
        Ok(0.25)
    );
    assert_eq!(
        eval_boolean_with_context_mut("c = true; c", &mut context),
        Ok(true)
    );
    assert_eq!(
        eval_boolean_with_context_mut("c &&= false; c", &mut context),
        Ok(false)
    );
    assert_eq!(
        eval_boolean_with_context_mut("c ||= true; c", &mut context),
        Ok(true)
    );
}

#[test]
fn test_type_errors_in_binary_operators() {
    // Only addition supports incompatible types, all others work only on numbers or only on booleans.
    // So only addition requires the more fancy error message.
    assert_eq!(
        eval("4 + \"abc\""),
        Err(EvalexprError::wrong_type_combination(
            Operator::Add,
            vec![ValueType::Int, ValueType::String]
        ))
    );
    assert_eq!(
        eval("\"abc\" + 4"),
        Err(EvalexprError::wrong_type_combination(
            Operator::Add,
            vec![ValueType::String, ValueType::Int]
        ))
    );
}
