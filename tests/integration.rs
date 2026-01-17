#![cfg(not(tarpaulin_include))]

use evalexpr::{error::*, *};
use std::convert::TryFrom;

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
    assert_eq!(eval("1e0"), Ok(Value::Float(1.0)));
    assert_eq!(eval("1e-0"), Ok(Value::Float(1.0)));
    assert_eq!(eval("10e3"), Ok(Value::Float(10000.0)));
    assert_eq!(eval("10e+3"), Ok(Value::Float(10000.0)));
    assert_eq!(eval("10e-3"), Ok(Value::Float(0.01)));
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
    assert_eq!(eval("5e2--3"), Ok(Value::Float(503.0)));
    assert_eq!(eval("-5e-2--3"), Ok(Value::Float(2.95)));
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
    assert_eq!(
        eval("6 ^ 4"),
        Ok(Value::Float(
            (6.0 as <DefaultNumericTypes as EvalexprNumericTypes>::Float).powf(4.0)
        ))
    );
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
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
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

    assert_eq!(context.remove_value("half"), Ok(Some(Value::Float(0.5))));
    assert_eq!(context.remove_value("zero"), Ok(Some(Value::Int(0))));
    assert_eq!(context.remove_value("zero"), Ok(None));
    assert_eq!(
        eval_with_context("zero", &context),
        Err(EvalexprError::VariableIdentifierNotFound(
            "zero".to_string()
        ))
    );
}

#[test]
fn test_functions() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
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
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
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
                Value::Tuple(tuple) => Ok(Value::from_int(
                    tuple.len() as <DefaultNumericTypes as EvalexprNumericTypes>::Int
                )),
                Value::Empty => Ok(Value::from_int(0)),
                _ => Ok(Value::from_int(1)),
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
fn test_capturing_functions() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
    // this variable is captured by the function
    let three = 3;
    context
        .set_function(
            "mult_3".into(),
            Function::new(move |argument| {
                if let Value::Int(int) = argument {
                    Ok(Value::Int(int * three))
                } else if let Value::Float(float) = argument {
                    Ok(Value::Float(
                        float * three as <DefaultNumericTypes as EvalexprNumericTypes>::Float,
                    ))
                } else {
                    Err(EvalexprError::expected_number(argument.clone()))
                }
            }),
        )
        .unwrap();

    let four = 4;
    context
        .set_function(
            "function_four".into(),
            Function::new(move |_| Ok(Value::Int(four))),
        )
        .unwrap();

    assert_eq!(eval_with_context("mult_3 2", &context), Ok(Value::Int(6)));
    assert_eq!(eval_with_context("mult_3(3)", &context), Ok(Value::Int(9)));
    assert_eq!(
        eval_with_context("mult_3(function_four())", &context),
        Ok(Value::Int(12))
    );
}

#[test]
fn test_builtin_functions() {
    // Log
    assert_eq!(eval("math::ln(2.718281828459045)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("math::log(9, 9)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("math::log2(2)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("math::log10(10)"), Ok(Value::Float(1.0)));
    // Powers
    assert_eq!(
        eval("math::exp(2)"),
        Ok(Value::Float(
            (2.0 as <DefaultNumericTypes as EvalexprNumericTypes>::Float).exp()
        ))
    );
    assert_eq!(
        eval("math::exp2(2)"),
        Ok(Value::Float(
            (2.0 as <DefaultNumericTypes as EvalexprNumericTypes>::Float).exp2()
        ))
    );
    assert_eq!(
        eval("math::pow(1.5, 1.3)"),
        Ok(Value::Float(
            (1.5 as <DefaultNumericTypes as EvalexprNumericTypes>::Float).powf(1.3)
        ))
    );
    // Cos
    assert_eq!(eval("math::cos(0)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("math::acos(1)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("math::cosh(0)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("math::acosh(1)"), Ok(Value::Float(0.0)));
    // Sin
    assert_eq!(eval("math::sin(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("math::asin(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("math::sinh(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("math::asinh(0)"), Ok(Value::Float(0.0)));
    // Tan
    assert_eq!(eval("math::tan(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("math::atan(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("math::tanh(0)"), Ok(Value::Float(0.0)));
    assert_eq!(eval("math::atanh(0)"), Ok(Value::Float(0.0)));
    assert_eq!(
        eval("math::atan2(1.2, -5.5)"),
        Ok(Value::Float(
            (1.2 as <DefaultNumericTypes as EvalexprNumericTypes>::Float).atan2(-5.5)
        ))
    );
    // Root
    assert_eq!(eval("math::sqrt(25)"), Ok(Value::Float(5.0)));
    assert_eq!(eval("math::cbrt(8)"), Ok(Value::Float(2.0)));
    // Hypotenuse
    assert_eq!(
        eval("math::hypot(8.2, 1.1)"),
        Ok(Value::Float(
            (8.2 as <DefaultNumericTypes as EvalexprNumericTypes>::Float).hypot(1.1)
        ))
    );
    // Rounding
    assert_eq!(eval("floor(1.1)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("floor(1.9)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("round(1.1)"), Ok(Value::Float(1.0)));
    assert_eq!(eval("round(1.5)"), Ok(Value::Float(2.0)));
    assert_eq!(eval("round(2.5)"), Ok(Value::Float(3.0)));
    assert_eq!(eval("round(1.9)"), Ok(Value::Float(2.0)));
    assert_eq!(eval("ceil(1.1)"), Ok(Value::Float(2.0)));
    assert_eq!(eval("ceil(1.9)"), Ok(Value::Float(2.0)));
    assert_eq!(eval("math::is_nan(1.0/0.0)"), Ok(Value::Boolean(false)));
    assert_eq!(eval("math::is_nan(0.0/0.0)"), Ok(Value::Boolean(true)));
    assert_eq!(eval("math::is_finite(1.0/0.0)"), Ok(Value::Boolean(false)));
    assert_eq!(eval("math::is_finite(0.0/0.0)"), Ok(Value::Boolean(false)));
    assert_eq!(eval("math::is_finite(0.0)"), Ok(Value::Boolean(true)));
    assert_eq!(
        eval("math::is_infinite(0.0/0.0)"),
        Ok(Value::Boolean(false))
    );
    assert_eq!(eval("math::is_infinite(1.0/0.0)"), Ok(Value::Boolean(true)));
    assert_eq!(eval("math::is_normal(1.0/0.0)"), Ok(Value::Boolean(false)));
    assert_eq!(eval("math::is_normal(0)"), Ok(Value::Boolean(false)));
    // Absolute
    assert_eq!(eval("math::abs(15.4)"), Ok(Value::Float(15.4)));
    assert_eq!(eval("math::abs(-15.4)"), Ok(Value::Float(15.4)));
    assert_eq!(eval("math::abs(15)"), Ok(Value::Int(15)));
    assert_eq!(eval("math::abs(-15)"), Ok(Value::Int(15)));
    // Other
    assert_eq!(eval("typeof(4.0, 3)"), Ok(Value::String("tuple".into())));
    assert_eq!(eval("typeof(4.0)"), Ok(Value::String("float".into())));
    assert_eq!(eval("typeof(4)"), Ok(Value::String("int".into())));
    assert_eq!(eval("typeof(\"\")"), Ok(Value::String("string".into())));
    assert_eq!(eval("typeof(true)"), Ok(Value::String("boolean".into())));
    assert_eq!(eval("typeof()"), Ok(Value::String("empty".into())));
    assert_eq!(eval("min(4.0, 3)"), Ok(Value::Int(3)));
    assert_eq!(eval("max(4.0, 3)"), Ok(Value::Float(4.0)));
    assert_eq!(eval("len(\"foobar\")"), Ok(Value::Int(6)));
    assert_eq!(eval("len(\"a\", \"b\")"), Ok(Value::Int(2)));
    //Contians
    assert_eq!(
        eval("contains(1, 2, 3)"),
        Err(EvalexprError::expected_fixed_len_tuple(
            2,
            Value::Tuple(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        ))
    );
    assert_eq!(
        eval("contains((\"foo\", \"bar\"), \"bar\")"),
        Ok(Value::Boolean(true))
    );
    assert_eq!(
        eval("contains((\"foo\", \"bar\"), \"buzz\")"),
        Ok(Value::Boolean(false)),
    );
    assert_eq!(
        eval("contains(\"foo\", \"bar\")"),
        Err(EvalexprError::expected_tuple(Value::String("foo".into())))
    );
    assert_eq!(
        eval("contains((\"foo\", \"bar\", 123), 123)"),
        Ok(Value::Boolean(true))
    );
    assert_eq!(
        eval("contains((\"foo\", \"bar\"), (\"buzz\", \"bazz\"))"),
        Err(EvalexprError::type_error(
            Value::Tuple(vec![
                Value::String("buzz".into()),
                Value::String("bazz".into())
            ]),
            vec![
                ValueType::String,
                ValueType::Int,
                ValueType::Float,
                ValueType::Boolean
            ]
        ))
    );
    //Contains Any
    assert_eq!(
        eval("contains_any(1, 2, 3)"),
        Err(EvalexprError::expected_fixed_len_tuple(
            2,
            Value::Tuple(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        ))
    );
    assert_eq!(
        eval("contains_any((\"foo\", \"bar\"), (\"bar\", \"buzz\"))"),
        Ok(Value::Boolean(true))
    );
    assert_eq!(
        eval("contains_any((\"foo\", \"bar\"), (\"buzz\", \"bazz\"))"),
        Ok(Value::Boolean(false)),
    );
    assert_eq!(
        eval("contains_any((1,2,3), (3,4,5))"),
        Ok(Value::Boolean(true))
    );
    assert_eq!(
        eval("contains_any((1,2,3), (4,5,6))"),
        Ok(Value::Boolean(false))
    );
    assert_eq!(
        eval("contains_any((true, false, true, true), (false, false, false))"),
        Ok(Value::Boolean(true))
    );
    assert_eq!(
        eval("contains_any(\"foo\", \"bar\")"),
        Err(EvalexprError::expected_tuple(Value::String("foo".into())))
    );
    assert_eq!(
        eval("contains_any((\"foo\", \"bar\"), \"buzz\")"),
        Err(EvalexprError::expected_tuple(Value::String("buzz".into())))
    );
    assert_eq!(
        eval("contains_any((\"foo\", \"bar\"), (\"buzz\", (1, 2, 3)))"),
        Err(EvalexprError::type_error(
            Value::Tuple(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
            vec![
                ValueType::String,
                ValueType::Int,
                ValueType::Float,
                ValueType::Boolean
            ]
        ))
    );
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
    assert_eq!(
        eval("str::from(\"a\")"),
        Ok(Value::String(String::from("a")))
    );
    assert_eq!(eval("str::from(1.0)"), Ok(Value::String(String::from("1"))));
    assert_eq!(
        eval("str::from(4.2)"),
        Ok(Value::String(String::from("4.2")))
    );
    assert_eq!(eval("str::from(1)"), Ok(Value::String(String::from("1"))));
    assert_eq!(
        eval("str::from(true)"),
        Ok(Value::String(String::from("true")))
    );
    assert_eq!(
        eval(r#"str::from((1, "foo", , false))"#),
        Ok(Value::String(String::from(r#"(1, "foo", (), false)"#)))
    );
    assert_eq!(
        eval("str::from(true)"),
        Ok(Value::String(String::from("true")))
    );
    assert_eq!(
        eval("str::from(1, 2, 3)"),
        Ok(Value::String(String::from("(1, 2, 3)")))
    );
    assert_eq!(eval("str::from()"), Ok(Value::String(String::from("()"))));
    assert_eq!(
        eval("str::substring(\"foobar\", 3)"),
        Ok(Value::String(String::from("bar")))
    );
    assert_eq!(
        eval("str::substring(\"foobar\", 3, 3)"),
        Ok(Value::String(String::from("")))
    );
    assert_eq!(
        eval("str::substring(\"foobar\", 3, 4)"),
        Ok(Value::String(String::from("b")))
    );
    assert!(eval("str::substring()").is_err());
    assert!(eval("str::substring(\"foobar\")").is_err());
    assert!(eval("str::substring(\"foobar\", 2, 1)").is_err());
    assert!(eval("str::substring(\"foobar\", 99999)").is_err());
    assert!(eval("str::substring(\"foobar\", -1)").is_err());
    assert!(eval("str::substring(\"foobar\", 0, -1)").is_err());
    assert!(eval("str::substring(\"foobar\", 0, 1, 1)").is_err());
    // Bitwise
    assert_eq!(eval("bitand(5, -1)"), Ok(Value::Int(5)));
    assert_eq!(eval("bitand(6, 5)"), Ok(Value::Int(4)));
    assert_eq!(eval("bitor(5, -1)"), Ok(Value::Int(-1)));
    assert_eq!(eval("bitor(6, 5)"), Ok(Value::Int(7)));
    assert_eq!(eval("bitxor(5, -1)"), Ok(Value::Int(-6)));
    assert_eq!(eval("bitxor(6, 5)"), Ok(Value::Int(3)));
    assert_eq!(eval("bitnot(5)"), Ok(Value::Int(-6)));
    assert_eq!(eval("bitnot(-1)"), Ok(Value::Int(0)));
    assert_eq!(eval("shl(5, 1)"), Ok(Value::Int(10)));
    assert_eq!(eval("shl(-6, 5)"), Ok(Value::Int(-192)));
    assert_eq!(eval("shr(5, 1)"), Ok(Value::Int(2)));
    assert_eq!(eval("shr(-6, 5)"), Ok(Value::Int(-1)));
    assert_eq!(eval("if(true, -6, 5)"), Ok(Value::Int(-6)));
    assert_eq!(eval("if(false, -6, 5)"), Ok(Value::Int(5)));
    assert_eq!(
        eval("if(2-1==1, \"good\", 0)"),
        Ok(Value::String(String::from("good")))
    );
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
            expected: 2,
        })
    );
    assert_eq!(eval("!(()true)"), Err(EvalexprError::AppendedToLeafNode));
    assert_eq!(
        eval("math::is_nan(\"xxx\")"),
        Err(EvalexprError::ExpectedNumber {
            actual: Value::String("xxx".to_string())
        })
    );
}

#[test]
fn test_no_panic() {
    assert!(eval(&format!(
        "{} + {}",
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX,
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX
    ))
    .is_err());
    assert!(eval(&format!(
        "-{} - {}",
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX,
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX
    ))
    .is_err());
    assert!(eval(&format!(
        "-(-{} - 1)",
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX
    ))
    .is_err());
    assert!(eval(&format!(
        "{} * {}",
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX,
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX
    ))
    .is_err());
    assert!(eval(&format!(
        "{} / {}",
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX,
        0
    ))
    .is_err());
    assert!(eval(&format!(
        "{} % {}",
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX,
        0
    ))
    .is_err());
    assert!(eval(&format!(
        "{} ^ {}",
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX,
        <DefaultNumericTypes as EvalexprNumericTypes>::Int::MAX
    ))
    .is_ok());
    assert!(eval("if").is_err());
    assert!(eval("if()").is_err());
    assert!(eval("if(true, 1)").is_err());
    assert!(eval("if(false, 2)").is_err());
    assert!(eval("if(1,1,1)").is_err());
    assert!(eval("if(true,1,1,1)").is_err());
}

#[test]
fn test_shortcut_functions() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
    context
        .set_value("string".into(), Value::from("a string"))
        .unwrap();

    assert_eq!(eval_string("\"3.3\""), Ok("3.3".to_owned()));
    assert_eq!(
        eval_string("3.3"),
        Err(EvalexprError::ExpectedString {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        eval_string("3..3"),
        Err(EvalexprError::VariableIdentifierNotFound("3..3".to_owned()))
    );
    assert_eq!(
        eval_string_with_context("string", &context),
        Ok("a string".to_owned())
    );
    assert_eq!(
        eval_string_with_context("3.3", &context),
        Err(EvalexprError::ExpectedString {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        eval_string_with_context("3..3", &context),
        Err(EvalexprError::VariableIdentifierNotFound("3..3".to_owned()))
    );
    assert_eq!(
        eval_string_with_context_mut("string", &mut context),
        Ok("a string".to_string())
    );
    assert_eq!(
        eval_string_with_context_mut("3.3", &mut context),
        Err(EvalexprError::ExpectedString {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        eval_string_with_context_mut("3..3", &mut context),
        Err(EvalexprError::VariableIdentifierNotFound("3..3".to_owned()))
    );

    assert_eq!(eval_float("3.3"), Ok(3.3));
    assert_eq!(
        eval_float("33"),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        eval_float("asd()"),
        Err(EvalexprError::FunctionIdentifierNotFound("asd".to_owned()))
    );
    assert_eq!(eval_float_with_context("3.3", &context), Ok(3.3));
    assert_eq!(
        eval_float_with_context("33", &context),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        eval_float_with_context("asd)", &context),
        Err(EvalexprError::UnmatchedRBrace)
    );
    assert_eq!(eval_float_with_context_mut("3.3", &mut context), Ok(3.3));
    assert_eq!(
        eval_float_with_context_mut("33", &mut context),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        eval_float_with_context_mut("asd(", &mut context),
        Err(EvalexprError::UnmatchedLBrace)
    );

    assert_eq!(eval_int("3"), Ok(3));
    assert_eq!(
        eval_int("3.3"),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        eval_int("(,);."),
        Err(EvalexprError::VariableIdentifierNotFound(".".to_owned()))
    );
    assert_eq!(eval_int_with_context("3", &context), Ok(3));
    assert_eq!(
        eval_int_with_context("3.3", &context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        eval_int_with_context("(,);.", &context),
        Err(EvalexprError::VariableIdentifierNotFound(".".to_owned()))
    );
    assert_eq!(eval_int_with_context_mut("3", &mut context), Ok(3));
    assert_eq!(
        eval_int_with_context_mut("3.3", &mut context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        eval_int_with_context_mut("(,);.", &mut context),
        Err(EvalexprError::VariableIdentifierNotFound(".".to_owned()))
    );

    assert_eq!(eval_number("3"), Ok(3.0));
    assert_eq!(
        eval_number("true"),
        Err(EvalexprError::ExpectedNumber {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        eval_number("abc"),
        Err(EvalexprError::VariableIdentifierNotFound("abc".to_owned()))
    );
    assert_eq!(eval_number_with_context("3.5", &context), Ok(3.5));
    assert_eq!(eval_number_with_context("3", &context), Ok(3.0));
    assert_eq!(
        eval_number_with_context("true", &context),
        Err(EvalexprError::ExpectedNumber {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        eval_number_with_context("abc", &context),
        Err(EvalexprError::VariableIdentifierNotFound("abc".to_owned()))
    );
    assert_eq!(eval_number_with_context_mut("3.5", &mut context), Ok(3.5));
    assert_eq!(eval_number_with_context_mut("3", &mut context), Ok(3.0));
    assert_eq!(
        eval_number_with_context_mut("true", &mut context),
        Err(EvalexprError::ExpectedNumber {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        eval_number_with_context_mut("abc", &mut context),
        Err(EvalexprError::VariableIdentifierNotFound("abc".to_owned()))
    );

    assert_eq!(eval_boolean("true"), Ok(true));
    assert_eq!(
        eval_boolean("4"),
        Err(EvalexprError::ExpectedBoolean {
            actual: Value::Int(4)
        })
    );
    assert_eq!(
        eval_boolean("trueee"),
        Err(EvalexprError::VariableIdentifierNotFound(
            "trueee".to_owned()
        ))
    );
    assert_eq!(eval_boolean_with_context("true", &context), Ok(true));
    assert_eq!(
        eval_boolean_with_context("4", &context),
        Err(EvalexprError::ExpectedBoolean {
            actual: Value::Int(4)
        })
    );
    assert_eq!(
        eval_boolean_with_context("trueee", &context),
        Err(EvalexprError::VariableIdentifierNotFound(
            "trueee".to_owned()
        ))
    );
    assert_eq!(
        eval_boolean_with_context_mut("true", &mut context),
        Ok(true)
    );
    assert_eq!(
        eval_boolean_with_context_mut("4", &mut context),
        Err(EvalexprError::ExpectedBoolean {
            actual: Value::Int(4)
        })
    );
    assert_eq!(
        eval_boolean_with_context_mut("trueee", &mut context),
        Err(EvalexprError::VariableIdentifierNotFound(
            "trueee".to_owned()
        ))
    );

    assert_eq!(eval_tuple("3,3"), Ok(vec![Value::Int(3), Value::Int(3)]));
    assert_eq!(
        eval_tuple("33"),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        eval_tuple("3a3"),
        Err(EvalexprError::VariableIdentifierNotFound("3a3".to_owned()))
    );
    assert_eq!(
        eval_tuple_with_context("3,3", &context),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        eval_tuple_with_context("33", &context),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        eval_tuple_with_context("3a3", &context),
        Err(EvalexprError::VariableIdentifierNotFound("3a3".to_owned()))
    );
    assert_eq!(
        eval_tuple_with_context_mut("3,3", &mut context),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        eval_tuple_with_context_mut("33", &mut context),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        eval_tuple_with_context_mut("3a3", &mut context),
        Err(EvalexprError::VariableIdentifierNotFound("3a3".to_owned()))
    );

    assert_eq!(eval_empty(""), Ok(EMPTY_VALUE));
    assert_eq!(eval_empty("()"), Ok(EMPTY_VALUE));
    assert_eq!(
        eval_empty("(,)"),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Tuple(vec![Value::Empty, Value::Empty])
        })
    );
    assert_eq!(
        eval_empty("xaq"),
        Err(EvalexprError::VariableIdentifierNotFound("xaq".to_owned()))
    );
    assert_eq!(eval_empty_with_context("", &context), Ok(EMPTY_VALUE));
    assert_eq!(eval_empty_with_context("()", &context), Ok(EMPTY_VALUE));
    assert_eq!(
        eval_empty_with_context("(,)", &context),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Tuple(vec![Value::Empty, Value::Empty])
        })
    );
    assert_eq!(
        eval_empty_with_context("xaq", &context),
        Err(EvalexprError::VariableIdentifierNotFound("xaq".to_owned()))
    );
    assert_eq!(
        eval_empty_with_context_mut("", &mut context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        eval_empty_with_context_mut("()", &mut context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        eval_empty_with_context_mut("(,)", &mut context),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Tuple(vec![Value::Empty, Value::Empty])
        })
    );
    assert_eq!(
        eval_empty_with_context_mut("xaq", &mut context),
        Err(EvalexprError::VariableIdentifierNotFound("xaq".to_owned()))
    );

    // With detour via build_operator_tree

    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("\"3.3\"")
            .unwrap()
            .eval_string(),
        Ok("3.3".to_owned())
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("3.3")
            .unwrap()
            .eval_string(),
        Err(EvalexprError::ExpectedString {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("3..3")
            .unwrap()
            .eval_string(),
        Err(EvalexprError::VariableIdentifierNotFound("3..3".to_owned()))
    );
    assert_eq!(
        build_operator_tree("string")
            .unwrap()
            .eval_string_with_context(&context),
        Ok("a string".to_owned())
    );
    assert_eq!(
        build_operator_tree("3.3")
            .unwrap()
            .eval_string_with_context(&context),
        Err(EvalexprError::ExpectedString {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        build_operator_tree("3..3")
            .unwrap()
            .eval_string_with_context(&context),
        Err(EvalexprError::VariableIdentifierNotFound("3..3".to_owned()))
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
            .eval_string_with_context_mut(&mut context),
        Err(EvalexprError::ExpectedString {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        build_operator_tree("3..3")
            .unwrap()
            .eval_string_with_context_mut(&mut context),
        Err(EvalexprError::VariableIdentifierNotFound("3..3".to_owned()))
    );

    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("3.3")
            .unwrap()
            .eval_float(),
        Ok(3.3)
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("33")
            .unwrap()
            .eval_float(),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("asd()")
            .unwrap()
            .eval_float(),
        Err(EvalexprError::FunctionIdentifierNotFound("asd".to_owned()))
    );
    assert_eq!(
        build_operator_tree("3.3")
            .unwrap()
            .eval_float_with_context(&context),
        Ok(3.3)
    );
    assert_eq!(
        build_operator_tree("33")
            .unwrap()
            .eval_float_with_context(&context),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        build_operator_tree("asd")
            .unwrap()
            .eval_float_with_context(&context),
        Err(EvalexprError::VariableIdentifierNotFound("asd".to_owned()))
    );
    assert_eq!(
        build_operator_tree("3.3")
            .unwrap()
            .eval_float_with_context_mut(&mut context),
        Ok(3.3)
    );
    assert_eq!(
        build_operator_tree("33")
            .unwrap()
            .eval_float_with_context_mut(&mut context),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        build_operator_tree("asd")
            .unwrap()
            .eval_float_with_context_mut(&mut context),
        Err(EvalexprError::VariableIdentifierNotFound("asd".to_owned()))
    );

    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("3")
            .unwrap()
            .eval_int(),
        Ok(3)
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("3.3")
            .unwrap()
            .eval_int(),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("(,);.")
            .unwrap()
            .eval_int(),
        Err(EvalexprError::VariableIdentifierNotFound(".".to_owned()))
    );
    assert_eq!(
        build_operator_tree("3")
            .unwrap()
            .eval_int_with_context(&context),
        Ok(3)
    );
    assert_eq!(
        build_operator_tree("3.3")
            .unwrap()
            .eval_int_with_context(&context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        build_operator_tree("(,);.")
            .unwrap()
            .eval_int_with_context(&context),
        Err(EvalexprError::VariableIdentifierNotFound(".".to_owned()))
    );
    assert_eq!(
        build_operator_tree("3")
            .unwrap()
            .eval_int_with_context_mut(&mut context),
        Ok(3)
    );
    assert_eq!(
        build_operator_tree("3.3")
            .unwrap()
            .eval_int_with_context_mut(&mut context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(3.3)
        })
    );
    assert_eq!(
        build_operator_tree("(,);.")
            .unwrap()
            .eval_int_with_context_mut(&mut context),
        Err(EvalexprError::VariableIdentifierNotFound(".".to_owned()))
    );

    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("3")
            .unwrap()
            .eval_number(),
        Ok(3.0)
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("true")
            .unwrap()
            .eval_number(),
        Err(EvalexprError::ExpectedNumber {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("abc")
            .unwrap()
            .eval_number(),
        Err(EvalexprError::VariableIdentifierNotFound("abc".to_owned()))
    );
    assert_eq!(
        build_operator_tree("3")
            .unwrap()
            .eval_number_with_context(&context),
        Ok(3.0)
    );
    assert_eq!(
        build_operator_tree("true")
            .unwrap()
            .eval_number_with_context(&context),
        Err(EvalexprError::ExpectedNumber {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        build_operator_tree("abc")
            .unwrap()
            .eval_number_with_context(&context),
        Err(EvalexprError::VariableIdentifierNotFound("abc".to_owned()))
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
            .eval_number_with_context_mut(&mut context),
        Err(EvalexprError::ExpectedNumber {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        build_operator_tree("abc")
            .unwrap()
            .eval_number_with_context_mut(&mut context),
        Err(EvalexprError::VariableIdentifierNotFound("abc".to_owned()))
    );

    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("true")
            .unwrap()
            .eval_boolean(),
        Ok(true)
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("4")
            .unwrap()
            .eval_boolean(),
        Err(EvalexprError::ExpectedBoolean {
            actual: Value::Int(4)
        })
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("trueee")
            .unwrap()
            .eval_boolean(),
        Err(EvalexprError::VariableIdentifierNotFound(
            "trueee".to_owned()
        ))
    );
    assert_eq!(
        build_operator_tree("true")
            .unwrap()
            .eval_boolean_with_context(&context),
        Ok(true)
    );
    assert_eq!(
        build_operator_tree("4")
            .unwrap()
            .eval_boolean_with_context(&context),
        Err(EvalexprError::ExpectedBoolean {
            actual: Value::Int(4)
        })
    );
    assert_eq!(
        build_operator_tree("trueee")
            .unwrap()
            .eval_boolean_with_context(&context),
        Err(EvalexprError::VariableIdentifierNotFound(
            "trueee".to_owned()
        ))
    );
    assert_eq!(
        build_operator_tree("true")
            .unwrap()
            .eval_boolean_with_context_mut(&mut context),
        Ok(true)
    );
    assert_eq!(
        build_operator_tree("4")
            .unwrap()
            .eval_boolean_with_context_mut(&mut context),
        Err(EvalexprError::ExpectedBoolean {
            actual: Value::Int(4)
        })
    );
    assert_eq!(
        build_operator_tree("trueee")
            .unwrap()
            .eval_boolean_with_context_mut(&mut context),
        Err(EvalexprError::VariableIdentifierNotFound(
            "trueee".to_owned()
        ))
    );

    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("3,3")
            .unwrap()
            .eval_tuple(),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("33")
            .unwrap()
            .eval_tuple(),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("3a3")
            .unwrap()
            .eval_tuple(),
        Err(EvalexprError::VariableIdentifierNotFound("3a3".to_owned()))
    );
    assert_eq!(
        build_operator_tree("3,3")
            .unwrap()
            .eval_tuple_with_context(&context),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        build_operator_tree("33")
            .unwrap()
            .eval_tuple_with_context(&context),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        build_operator_tree("3a3")
            .unwrap()
            .eval_tuple_with_context(&context),
        Err(EvalexprError::VariableIdentifierNotFound("3a3".to_owned()))
    );
    assert_eq!(
        build_operator_tree("3,3")
            .unwrap()
            .eval_tuple_with_context_mut(&mut context),
        Ok(vec![Value::Int(3), Value::Int(3)])
    );
    assert_eq!(
        build_operator_tree("33")
            .unwrap()
            .eval_tuple_with_context_mut(&mut context),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Int(33)
        })
    );
    assert_eq!(
        build_operator_tree("3a3")
            .unwrap()
            .eval_tuple_with_context_mut(&mut context),
        Err(EvalexprError::VariableIdentifierNotFound("3a3".to_owned()))
    );

    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("")
            .unwrap()
            .eval_empty(),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("()")
            .unwrap()
            .eval_empty(),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("(,)")
            .unwrap()
            .eval_empty(),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Tuple(vec![Value::Empty, Value::Empty])
        })
    );
    assert_eq!(
        build_operator_tree::<DefaultNumericTypes>("xaq")
            .unwrap()
            .eval_empty(),
        Err(EvalexprError::VariableIdentifierNotFound("xaq".to_owned()))
    );
    assert_eq!(
        build_operator_tree("")
            .unwrap()
            .eval_empty_with_context(&context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        build_operator_tree("()")
            .unwrap()
            .eval_empty_with_context(&context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        build_operator_tree("(,)")
            .unwrap()
            .eval_empty_with_context(&context),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Tuple(vec![Value::Empty, Value::Empty])
        })
    );
    assert_eq!(
        build_operator_tree("xaq")
            .unwrap()
            .eval_empty_with_context(&context),
        Err(EvalexprError::VariableIdentifierNotFound("xaq".to_owned()))
    );
    assert_eq!(
        build_operator_tree("")
            .unwrap()
            .eval_empty_with_context_mut(&mut context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        build_operator_tree("()")
            .unwrap()
            .eval_empty_with_context_mut(&mut context),
        Ok(EMPTY_VALUE)
    );
    assert_eq!(
        build_operator_tree("(,)")
            .unwrap()
            .eval_empty_with_context_mut(&mut context),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Tuple(vec![Value::Empty, Value::Empty])
        })
    );
    assert_eq!(
        build_operator_tree("xaq")
            .unwrap()
            .eval_empty_with_context_mut(&mut context),
        Err(EvalexprError::VariableIdentifierNotFound("xaq".to_owned()))
    );
}

#[test]
fn test_whitespace() {
    assert!(eval_boolean("2 < = 3").is_err());
}

#[test]
fn test_assignment() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
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
        Ok(vec![Value::from_int(1), Value::from_int(1)])
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
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
    assert_eq!(
        eval_int_with_context_mut("a = 5; a = a + 2; a", &mut context),
        Ok(7)
    );
}

#[test]
fn test_strings() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
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
    assert_eq!(eval("\"a\" >= \"b\""), Ok(Value::from(false)));
    assert_eq!(eval("\"a\" <= \"b\""), Ok(Value::from(true)));
    assert_eq!(eval("\"a\" >= \"a\""), Ok(Value::from(true)));
    assert_eq!(eval("\"a\" <= \"a\""), Ok(Value::from(true)));
    assert_eq!(eval("\"xa\" > \"xb\""), Ok(Value::from(false)));
    assert_eq!(eval("\"xa\" < \"xb\""), Ok(Value::from(true)));
    assert_eq!(eval("\"{}\" != \"{}\""), Ok(Value::from(false)));
    assert_eq!(eval("\"{}\" == \"{}\""), Ok(Value::from(true)));
}

#[test]
fn test_string_escaping() {
    assert_eq!(
        eval("\"\\\"str\\\\ing\\\"\""),
        Ok(Value::from("\"str\\ing\""))
    );
}

#[test]
fn test_tuple_definitions() {
    assert_eq!(eval_empty("()"), Ok(()));
    assert_eq!(eval_int("(3)"), Ok(3));
    assert_eq!(
        eval_tuple("(3, 4)"),
        Ok(vec![Value::from_int(3), Value::from_int(4)])
    );
    assert_eq!(
        eval_tuple("2, (5, 6)"),
        Ok(vec![
            Value::from_int(2),
            Value::from(vec![Value::from_int(5), Value::from_int(6)])
        ])
    );
    assert_eq!(
        eval_tuple("1, 2"),
        Ok(vec![Value::from_int(1), Value::from_int(2)])
    );
    assert_eq!(
        eval_tuple("1, 2, 3, 4"),
        Ok(vec![
            Value::from_int(1),
            Value::from_int(2),
            Value::from_int(3),
            Value::from_int(4)
        ])
    );
    assert_eq!(
        eval_tuple("(1, 2, 3), 5, 6, (true, false, 0)"),
        Ok(vec![
            Value::from(vec![
                Value::from_int(1),
                Value::from_int(2),
                Value::from_int(3)
            ]),
            Value::from_int(5),
            Value::from_int(6),
            Value::from(vec![
                Value::from(true),
                Value::from(false),
                Value::from_int(0)
            ])
        ])
    );
    assert_eq!(
        eval_tuple("1, (2)"),
        Ok(vec![Value::from_int(1), Value::from_int(2)])
    );
    assert_eq!(
        eval_tuple("1, ()"),
        Ok(vec![Value::from_int(1), Value::from(())])
    );
    assert_eq!(
        eval_tuple("1, ((2))"),
        Ok(vec![Value::from_int(1), Value::from_int(2)])
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
        Ok(vec![Value::from_int(10), Value::from_float(10.5)])
    );
    assert_eq!(
        eval_string("a = \"xyz\"; b = \"abc\"; c = a + b; c"),
        Ok("xyzabc".to_string())
    );
}

#[test]
fn test_operator_assignments() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
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

    let mut context = HashMapContext::<DefaultNumericTypes>::new();
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

#[test]
fn test_empty_context() {
    let mut context = EmptyContext::<DefaultNumericTypes>::default();
    assert_eq!(context.get_value("abc"), None);
    assert_eq!(
        context.call_function("abc", &Value::Empty),
        Err(EvalexprError::FunctionIdentifierNotFound("abc".to_owned()))
    );
    assert_eq!(
        eval_with_context("max(1,3)", &context),
        Err(EvalexprError::FunctionIdentifierNotFound(String::from(
            "max"
        )))
    );
    assert_eq!(context.set_builtin_functions_disabled(true), Ok(()));
    assert_eq!(
        context.set_builtin_functions_disabled(false),
        Err(EvalexprError::BuiltinFunctionsCannotBeEnabled)
    )
}

#[test]
fn test_empty_context_with_builtin_functions() {
    let mut context = EmptyContextWithBuiltinFunctions::<DefaultNumericTypes>::default();
    assert_eq!(context.get_value("abc"), None);
    assert_eq!(
        context.call_function("abc", &Value::Empty),
        Err(EvalexprError::FunctionIdentifierNotFound("abc".to_owned()))
    );
    assert_eq!(eval_with_context("max(1,3)", &context), Ok(Value::Int(3)));
    assert_eq!(context.set_builtin_functions_disabled(false), Ok(()));
    assert_eq!(
        context.set_builtin_functions_disabled(true),
        Err(EvalexprError::BuiltinFunctionsCannotBeDisabled)
    );
}

#[test]
fn test_hashmap_context_type_safety() {
    let mut context: HashMapContext<DefaultNumericTypes> =
        context_map! {"a" => int 5, "b" => float 5.0}.unwrap();
    assert_eq!(
        eval_with_context_mut("a = 4", &mut context),
        Ok(Value::Empty)
    );
    assert_eq!(
        eval_with_context_mut("a = 4.0", &mut context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(4.0)
        })
    );
    assert_eq!(
        eval_with_context_mut("a += 4.0", &mut context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(8.0)
        })
    );
    assert_eq!(
        eval_with_context_mut("a -= 4.0", &mut context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(0.0)
        })
    );
    assert_eq!(
        eval_with_context_mut("a *= 4.0", &mut context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(16.0)
        })
    );
    assert_eq!(
        eval_with_context_mut("a /= 4.0", &mut context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(1.0)
        })
    );
    assert_eq!(
        eval_with_context_mut("a %= 4.0", &mut context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(0.0)
        })
    );
    assert_eq!(
        eval_with_context_mut("a ^= 4.0", &mut context),
        Err(EvalexprError::ExpectedInt {
            actual: Value::Float(256.0)
        })
    );

    assert_eq!(
        eval_with_context_mut("b = 4.0", &mut context),
        Ok(Value::Empty)
    );
    assert_eq!(
        eval_with_context_mut("b = 4", &mut context),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Int(4)
        })
    );
    assert_eq!(
        eval_with_context_mut("b += 4", &mut context),
        Ok(Value::Empty)
    );
    assert_eq!(
        eval_with_context_mut("b -= 4", &mut context),
        Ok(Value::Empty)
    );
    assert_eq!(
        eval_with_context_mut("b *= 4", &mut context),
        Ok(Value::Empty)
    );
    assert_eq!(
        eval_with_context_mut("b /= 4", &mut context),
        Ok(Value::Empty)
    );
    assert_eq!(
        eval_with_context_mut("b %= 4", &mut context),
        Ok(Value::Empty)
    );
    assert_eq!(
        eval_with_context_mut("b ^= 4", &mut context),
        Ok(Value::Empty)
    );
}

#[test]
fn test_hashmap_context_clone_debug() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
    // this variable is captured by the function
    let three = 3;
    context
        .set_function(
            "mult_3".into(),
            Function::new(move |argument| {
                if let Value::Int(int) = argument {
                    Ok(Value::Int(int * three))
                } else if let Value::Float(float) = argument {
                    Ok(Value::Float(
                        float * three as <DefaultNumericTypes as EvalexprNumericTypes>::Float,
                    ))
                } else {
                    Err(EvalexprError::expected_number(argument.clone()))
                }
            }),
        )
        .unwrap();

    let four = 4;
    context
        .set_function(
            "function_four".into(),
            Function::new(move |_| Ok(Value::Int(four))),
        )
        .unwrap();
    context
        .set_value("variable_five".into(), Value::from_int(5))
        .unwrap();
    let context = context;
    #[allow(clippy::redundant_clone)]
    let cloned_context = context.clone();

    assert_eq!(format!("{:?}", &context), format!("{:?}", &cloned_context));
    assert_eq!(
        cloned_context.get_value("variable_five"),
        Some(&Value::from_int(5))
    );
    assert_eq!(
        eval_with_context("mult_3 2", &cloned_context),
        Ok(Value::Int(6))
    );
    assert_eq!(
        eval_with_context("mult_3(3)", &cloned_context),
        Ok(Value::Int(9))
    );
    assert_eq!(
        eval_with_context("mult_3(function_four())", &cloned_context),
        Ok(Value::Int(12))
    );
}

#[test]
fn test_error_constructors() {
    assert_eq!(
        eval("a = true + \"4\""),
        Err(EvalexprError::ExpectedNumberOrString {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        eval("a = true && \"4\""),
        Err(EvalexprError::ExpectedBoolean {
            actual: Value::from("4")
        })
    );
    assert_eq!(
        eval_tuple("4"),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Int(4)
        })
    );
    assert_eq!(
        Value::Tuple(vec![Value::<DefaultNumericTypes>::Int(4), Value::Int(5)])
            .as_fixed_len_tuple(3),
        Err(EvalexprError::ExpectedFixedLengthTuple {
            expected_length: 3,
            actual: Value::Tuple(vec![Value::Int(4), Value::Int(5)])
        })
    );
    assert_eq!(
        eval_empty("4"),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Int(4)
        })
    );
    assert_eq!(
        eval("&"),
        Err(EvalexprError::UnmatchedPartialToken {
            first: PartialToken::Ampersand,
            second: None
        })
    );

    assert_eq!(
        expect_function_argument_amount::<DefaultNumericTypes>(2, 2),
        Ok(())
    );
    assert_eq!(
        expect_function_argument_amount::<DefaultNumericTypes>(2, 3),
        Err(EvalexprError::WrongFunctionArgumentAmount {
            expected: 3..=3,
            actual: 2
        })
    );
}

#[test]
fn test_iterators() {
    let tree =
        build_operator_tree::<DefaultNumericTypes>("writevar = 5 + 3 + fun(4) + var").unwrap();
    let mut iter = tree.iter_identifiers();
    assert_eq!(iter.next(), Some("writevar"));
    assert_eq!(iter.next(), Some("fun"));
    assert_eq!(iter.next(), Some("var"));
    assert_eq!(iter.next(), None);

    let mut iter = tree.iter_variable_identifiers();
    assert_eq!(iter.next(), Some("writevar"));
    assert_eq!(iter.next(), Some("var"));
    assert_eq!(iter.next(), None);

    let mut iter = tree.iter_read_variable_identifiers();
    assert_eq!(iter.next(), Some("var"));
    assert_eq!(iter.next(), None);

    let mut iter = tree.iter_write_variable_identifiers();
    assert_eq!(iter.next(), Some("writevar"));
    assert_eq!(iter.next(), None);

    let mut iter = tree.iter_function_identifiers();
    assert_eq!(iter.next(), Some("fun"));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_same_operator_chains() {
    #![allow(clippy::eq_op)]
    assert_eq!(
        eval("3.0 / 3.0 / 3.0 / 3.0"),
        Ok(Value::from_float(3.0 / 3.0 / 3.0 / 3.0))
    );
    assert_eq!(
        eval("3.0 - 3.0 - 3.0 - 3.0"),
        Ok(Value::from_float(3.0 - 3.0 - 3.0 - 3.0))
    );
}

#[test]
fn test_long_expression_i89() {
    let tree = build_operator_tree::<DefaultNumericTypes>(
        "x*0.2*5/4+x*2*4*1*1*1*1*1*1*1+7*math::sin(y)-z/math::sin(3.0/2.0/(1-x*4*1*1*1*1))",
    )
    .unwrap();
    let x = 0.0;
    let y: <DefaultNumericTypes as EvalexprNumericTypes>::Float = 3.0;
    let z = 4.0;
    let context = context_map! {
        "x" => float 0.0,
        "y" => float 3.0,
        "z" => float 4.0
    }
    .unwrap();
    let expected = x * 0.2 * 5.0 / 4.0
        + x * 2.0 * 4.0 * 1.0 * 1.0 * 1.0 * 1.0 * 1.0 * 1.0 * 1.0
        + 7.0 * y.sin()
        - z / (3.0 / 2.0 / (1.0 - x * 4.0 * 1.0 * 1.0 * 1.0 * 1.0)).sin();
    let actual: <DefaultNumericTypes as EvalexprNumericTypes>::Float =
        tree.eval_float_with_context(&context).unwrap();
    assert!(
        (expected - actual).abs() < expected.abs().min(actual.abs()) * 1e-12,
        "expected: {}, actual: {}",
        expected,
        actual
    );
}

#[test]
fn test_value_type() {
    assert_eq!(
        ValueType::from(&Value::<DefaultNumericTypes>::String(String::new())),
        ValueType::String
    );
    assert_eq!(
        ValueType::from(&Value::<DefaultNumericTypes>::Float(0.0)),
        ValueType::Float
    );
    assert_eq!(
        ValueType::from(&Value::<DefaultNumericTypes>::Int(0)),
        ValueType::Int
    );
    assert_eq!(
        ValueType::from(&Value::<DefaultNumericTypes>::Boolean(true)),
        ValueType::Boolean
    );
    assert_eq!(
        ValueType::from(&Value::<DefaultNumericTypes>::Tuple(Vec::new())),
        ValueType::Tuple
    );
    assert_eq!(
        ValueType::from(&Value::<DefaultNumericTypes>::Empty),
        ValueType::Empty
    );

    assert_eq!(
        ValueType::from(&mut Value::<DefaultNumericTypes>::String(String::new())),
        ValueType::String
    );
    assert_eq!(
        ValueType::from(&mut Value::<DefaultNumericTypes>::Float(0.0)),
        ValueType::Float
    );
    assert_eq!(
        ValueType::from(&mut Value::<DefaultNumericTypes>::Int(0)),
        ValueType::Int
    );
    assert_eq!(
        ValueType::from(&mut Value::<DefaultNumericTypes>::Boolean(true)),
        ValueType::Boolean
    );
    assert_eq!(
        ValueType::from(&mut Value::<DefaultNumericTypes>::Tuple(Vec::new())),
        ValueType::Tuple
    );
    assert_eq!(
        ValueType::from(&mut Value::<DefaultNumericTypes>::Empty),
        ValueType::Empty
    );

    assert!(!Value::<DefaultNumericTypes>::String(String::new()).is_number());
    assert!(Value::<DefaultNumericTypes>::Float(0.0).is_number());
    assert!(Value::<DefaultNumericTypes>::Int(0).is_number());
    assert!(!Value::<DefaultNumericTypes>::Boolean(true).is_number());
    assert!(!Value::<DefaultNumericTypes>::Tuple(Vec::new()).is_number());
    assert!(!Value::<DefaultNumericTypes>::Empty.is_number());

    assert!(!Value::<DefaultNumericTypes>::String(String::new()).is_empty());
    assert!(!Value::<DefaultNumericTypes>::Float(0.0).is_empty());
    assert!(!Value::<DefaultNumericTypes>::Int(0).is_empty());
    assert!(!Value::<DefaultNumericTypes>::Boolean(true).is_empty());
    assert!(!Value::<DefaultNumericTypes>::Tuple(Vec::new()).is_empty());
    assert!(Value::<DefaultNumericTypes>::Empty.is_empty());

    assert_eq!(
        Value::<DefaultNumericTypes>::String(String::new()).as_float(),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::String(String::new())
        })
    );
    assert_eq!(Value::<DefaultNumericTypes>::Float(0.0).as_float(), Ok(0.0));
    assert_eq!(
        Value::<DefaultNumericTypes>::Int(0).as_float(),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Int(0)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Boolean(true).as_float(),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Tuple(Vec::new()).as_float(),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Tuple(Vec::new())
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Empty.as_float(),
        Err(EvalexprError::ExpectedFloat {
            actual: Value::Empty
        })
    );

    assert_eq!(
        Value::<DefaultNumericTypes>::String(String::new()).as_tuple(),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::String(String::new())
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Float(0.0).as_tuple(),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Float(0.0)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Int(0).as_tuple(),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Int(0)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Boolean(true).as_tuple(),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Tuple(Vec::new()).as_tuple(),
        Ok(Vec::new())
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Empty.as_tuple(),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Empty
        })
    );

    assert_eq!(
        Value::<DefaultNumericTypes>::String(String::new()).as_fixed_len_tuple(0),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::String(String::new())
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Float(0.0).as_fixed_len_tuple(0),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Float(0.0)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Int(0).as_fixed_len_tuple(0),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Int(0)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Boolean(true).as_fixed_len_tuple(0),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Tuple(Vec::new()).as_fixed_len_tuple(0),
        Ok(Vec::new())
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Empty.as_fixed_len_tuple(0),
        Err(EvalexprError::ExpectedTuple {
            actual: Value::Empty
        })
    );

    assert_eq!(
        Value::<DefaultNumericTypes>::String(String::new()).as_empty(),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::String(String::new())
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Float(0.0).as_empty(),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Float(0.0)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Int(0).as_empty(),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Int(0)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Boolean(true).as_empty(),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Boolean(true)
        })
    );
    assert_eq!(
        Value::<DefaultNumericTypes>::Tuple(Vec::new()).as_empty(),
        Err(EvalexprError::ExpectedEmpty {
            actual: Value::Tuple(Vec::new())
        })
    );
    assert_eq!(Value::<DefaultNumericTypes>::Empty.as_empty(), Ok(()));

    assert_eq!(
        Result::from(Value::<DefaultNumericTypes>::String(String::new())),
        Ok(Value::String(String::new()))
    );
}

#[test]
fn test_parenthese_combinations() {
    // These are from issue #94
    assert_eq!(
        eval("123(1*2)"),
        Err(EvalexprError::MissingOperatorOutsideOfBrace)
    );
    assert_eq!(
        eval("1()"),
        Err(EvalexprError::MissingOperatorOutsideOfBrace)
    );
    assert_eq!(
        eval("1()()()()"),
        Err(EvalexprError::MissingOperatorOutsideOfBrace)
    );
    assert_eq!(
        eval("1()()()(9)()()"),
        Err(EvalexprError::MissingOperatorOutsideOfBrace)
    );
    assert_eq!(
        eval_with_context("a+100(a*2)", &context_map! {"a" => int 4}.unwrap()),
        Err(EvalexprError::<DefaultNumericTypes>::MissingOperatorOutsideOfBrace)
    );
    assert_eq!(eval_int("(((1+2)*(3+4)+(5-(6)))/((7-8)))"), Ok(-20));
    assert_eq!(eval_int("(((((5)))))"), Ok(5));
}

#[test]
fn test_try_from() {
    #![allow(clippy::redundant_clone)]

    let value = Value::<DefaultNumericTypes>::String("abc".to_string());
    assert_eq!(String::try_from(value.clone()), Ok("abc".to_string()));
    assert_eq!(
        bool::try_from(value.clone()),
        Err(EvalexprError::ExpectedBoolean {
            actual: value.clone()
        })
    );
    assert_eq!(
        TupleType::try_from(value.clone()),
        Err(EvalexprError::ExpectedTuple {
            actual: value.clone()
        })
    );
    assert_eq!(
        EmptyType::try_from(value.clone()),
        Err(EvalexprError::ExpectedEmpty {
            actual: value.clone()
        })
    );

    let value = Value::<DefaultNumericTypes>::Float(1.3);
    assert_eq!(
        String::try_from(value.clone()),
        Err(EvalexprError::ExpectedString {
            actual: value.clone()
        })
    );
    assert_eq!(
        bool::try_from(value.clone()),
        Err(EvalexprError::ExpectedBoolean {
            actual: value.clone()
        })
    );
    assert_eq!(
        TupleType::try_from(value.clone()),
        Err(EvalexprError::ExpectedTuple {
            actual: value.clone()
        })
    );
    assert_eq!(
        EmptyType::try_from(value.clone()),
        Err(EvalexprError::ExpectedEmpty {
            actual: value.clone()
        })
    );

    let value = Value::<DefaultNumericTypes>::Int(13);
    assert_eq!(
        String::try_from(value.clone()),
        Err(EvalexprError::ExpectedString {
            actual: value.clone()
        })
    );
    assert_eq!(
        bool::try_from(value.clone()),
        Err(EvalexprError::ExpectedBoolean {
            actual: value.clone()
        })
    );
    assert_eq!(
        TupleType::try_from(value.clone()),
        Err(EvalexprError::ExpectedTuple {
            actual: value.clone()
        })
    );
    assert_eq!(
        EmptyType::try_from(value.clone()),
        Err(EvalexprError::ExpectedEmpty {
            actual: value.clone()
        })
    );

    let value = Value::<DefaultNumericTypes>::Boolean(true);
    assert_eq!(
        String::try_from(value.clone()),
        Err(EvalexprError::ExpectedString {
            actual: value.clone()
        })
    );
    assert_eq!(bool::try_from(value.clone()), Ok(true));
    assert_eq!(
        TupleType::try_from(value.clone()),
        Err(EvalexprError::ExpectedTuple {
            actual: value.clone()
        })
    );
    assert_eq!(
        EmptyType::try_from(value.clone()),
        Err(EvalexprError::ExpectedEmpty {
            actual: value.clone()
        })
    );

    let value =
        Value::<DefaultNumericTypes>::Tuple(vec![Value::Int(1), Value::String("abc".to_string())]);
    assert_eq!(
        String::try_from(value.clone()),
        Err(EvalexprError::ExpectedString {
            actual: value.clone()
        })
    );
    assert_eq!(
        bool::try_from(value.clone()),
        Err(EvalexprError::ExpectedBoolean {
            actual: value.clone()
        })
    );
    assert_eq!(
        TupleType::try_from(value.clone()),
        Ok(vec![Value::Int(1), Value::String("abc".to_string())])
    );
    assert_eq!(
        EmptyType::try_from(value.clone()),
        Err(EvalexprError::ExpectedEmpty {
            actual: value.clone()
        })
    );

    let value = Value::<DefaultNumericTypes>::Empty;
    assert_eq!(
        String::try_from(value.clone()),
        Err(EvalexprError::ExpectedString {
            actual: value.clone()
        })
    );
    assert_eq!(
        bool::try_from(value.clone()),
        Err(EvalexprError::ExpectedBoolean {
            actual: value.clone()
        })
    );
    assert_eq!(
        TupleType::try_from(value.clone()),
        Err(EvalexprError::ExpectedTuple {
            actual: value.clone()
        })
    );
    assert_eq!(EmptyType::try_from(value.clone()), Ok(()));
}

#[test]
fn assignment_lhs_is_identifier() {
    let tree = build_operator_tree("a = 1").unwrap();
    let operators: Vec<_> = tree.iter().map(|node| node.operator().clone()).collect();

    let mut context = HashMapContext::<DefaultNumericTypes>::new();
    tree.eval_with_context_mut(&mut context).unwrap();
    assert_eq!(context.get_value("a"), Some(&Value::Int(1)));

    assert!(
        matches!(
            operators.as_slice(),
            [
                Operator::Assign,
                Operator::VariableIdentifierWrite { identifier: value },
                Operator::Const {
                    value: Value::Int(1)
                }
            ] if value == "a"
        ),
        "actual: {:#?}",
        operators
    );
}

#[test]
fn test_variable_assignment_and_iteration() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
    eval_with_context_mut("a = 5; b = 5.0", &mut context).unwrap();

    let mut variables: Vec<_> = context.iter_variables().collect();
    variables.sort_unstable_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
    assert_eq!(
        variables,
        vec![
            ("a".to_string(), Value::from_int(5)),
            ("b".to_string(), Value::from_float(5.0))
        ],
    );

    let mut variables: Vec<_> = context.iter_variable_names().collect();
    variables.sort_unstable();
    assert_eq!(variables, vec!["a".to_string(), "b".to_string()],);
}

#[test]
fn test_negative_power() {
    println!(
        "{:?}",
        build_operator_tree::<DefaultNumericTypes>("3^-2").unwrap()
    );
    assert_eq!(eval("3^-2"), Ok(Value::Float(1.0 / 9.0)));
    assert_eq!(eval("3^(-2)"), Ok(Value::Float(1.0 / 9.0)));
    assert_eq!(eval("-3^2"), Ok(Value::Float(-9.0)));
    assert_eq!(eval("-(3)^2"), Ok(Value::Float(-9.0)));
    assert_eq!(eval("(-3)^-2"), Ok(Value::Float(1.0 / 9.0)));
    assert_eq!(eval("-(3^-2)"), Ok(Value::Float(-1.0 / 9.0)));
}

#[test]
fn test_builtin_functions_context() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
    // Builtin functions are enabled by default for HashMapContext.
    assert_eq!(
        eval_with_context("max(1,3)", &context),
        Ok(Value::from_int(3))
    );
    // Disabling builtin function in Context.
    context.set_builtin_functions_disabled(true).unwrap();
    // Builtin functions are disabled and using them returns an error.
    assert_eq!(
        eval_with_context("max(1,3)", &context),
        Err(EvalexprError::FunctionIdentifierNotFound(String::from(
            "max"
        )))
    );
}

#[test]
fn test_hex() {
    assert_eq!(eval("0x3"), Ok(Value::Int(3)));
    assert_eq!(eval("0xFF"), Ok(Value::Int(255)));
    assert_eq!(eval("-0xFF"), Ok(Value::Int(-255)));
    assert_eq!(
        eval("0x"),
        // The "VariableIdentifierNotFound" error is what evalexpr currently returns,
        // but ideally it would return more specific errors for "illegal" literals.
        Err(EvalexprError::VariableIdentifierNotFound("0x".into()))
    );
}

#[test]
fn test_binary() {
    assert_eq!(eval("0b11"), Ok(Value::Int(3)));
    assert_eq!(eval("0b0101"), Ok(Value::Int(5)));
    assert_eq!(eval("0b11111111"), Ok(Value::Int(255)));
    assert_eq!(eval("-0b11111111"), Ok(Value::Int(-255)));
    assert_eq!(
        eval("0b2"),
        // See `test_hex`.
        Err(EvalexprError::VariableIdentifierNotFound("0b2".into()))
    );
}

#[test]
fn test_octal() {
    assert_eq!(eval("0o12"), Ok(Value::Int(10)));
    assert_eq!(eval("0o3"), Ok(Value::Int(3)));
    assert_eq!(eval("0o377"), Ok(Value::Int(255)));
    assert_eq!(eval("-0o377"), Ok(Value::Int(-255)));
    assert_eq!(
        eval("0o8"),
        // See `test_hex`.
        Err(EvalexprError::VariableIdentifierNotFound("0o8".into()))
    );
}

#[test]
fn test_broken_string() {
    assert_eq!(
        eval(r#""abc" == "broken string"#),
        Err(EvalexprError::UnmatchedDoubleQuote)
    );
}

#[test]
fn test_comments() {
    assert_eq!(
        eval(
            "
            // input
            a = 1;  // assignment
            // output
            a + 2  // add"
        ),
        Ok(Value::Int(3))
    );

    assert_eq!(
        eval("0 /*"),
        Err(EvalexprError::CustomMessage(
            "unmatched inline comment".into()
        ))
    );

    assert_eq!(
        eval("1 % 4 + /*inline comment*/ 6 /*END*/"),
        Ok(Value::Int(7))
    );

    assert_eq!(
        eval("/* begin */ 10 /* middle */ + 5 /* end */ + 6 // DONE"),
        Ok(Value::Int(21))
    );
}

#[test]
fn test_clear() {
    let mut context = HashMapContext::<DefaultNumericTypes>::new();
    context.set_value("abc".into(), "def".into()).unwrap();
    assert_eq!(context.get_value("abc"), Some(&("def".into())));
    context.clear_functions();
    assert_eq!(context.get_value("abc"), Some(&("def".into())));
    context.clear_variables();
    assert_eq!(context.get_value("abc"), None);

    context
        .set_function(
            "abc".into(),
            Function::new(|input| Ok(Value::String(format!("{input}")))),
        )
        .unwrap();
    assert_eq!(
        eval_with_context("abc(5)", &context).unwrap(),
        Value::String("5".into())
    );
    context.clear_variables();
    assert_eq!(
        eval_with_context("abc(5)", &context).unwrap(),
        Value::String("5".into())
    );
    context.clear_functions();
    assert!(eval_with_context("abc(5)", &context).is_err());

    context
        .set_value("five".into(), Value::from_int(5))
        .unwrap();
    context
        .set_function(
            "abc".into(),
            Function::new(|input| Ok(Value::String(format!("{input}")))),
        )
        .unwrap();
    assert_eq!(
        eval_with_context("abc(five)", &context).unwrap(),
        Value::String("5".into())
    );
    context.clear();
    assert!(context.get_value("five").is_none());
    assert!(eval_with_context("abc(5)", &context).is_err());
}

#[test]
fn test_iter_empty_contexts() {
    assert_eq!(
        EmptyContext::<DefaultNumericTypes>::default()
            .iter_variables()
            .next(),
        None
    );
    assert_eq!(
        EmptyContext::<DefaultNumericTypes>::default()
            .iter_variable_names()
            .next(),
        None
    );
    assert_eq!(
        EmptyContextWithBuiltinFunctions::<DefaultNumericTypes>::default()
            .iter_variables()
            .next(),
        None
    );
    assert_eq!(
        EmptyContextWithBuiltinFunctions::<DefaultNumericTypes>::default()
            .iter_variable_names()
            .next(),
        None
    );
}

#[test]
fn test_empty_context_builtin_functions() {
    assert!(EmptyContext::<DefaultNumericTypes>::default().are_builtin_functions_disabled());
    assert!(
        !EmptyContextWithBuiltinFunctions::<DefaultNumericTypes>::default()
            .are_builtin_functions_disabled()
    );
}

#[test]
fn test_compare_different_numeric_types() {
    assert_eq!(eval("1 < 2.0"), Ok(true.into()));
    assert_eq!(eval("1 >= 2"), Ok(false.into()));
    assert_eq!(eval("1 >= 2.0"), Ok(false.into()));
}

#[test]
fn test_escape_sequences() {
    assert_eq!(
        eval("\"\\x\""),
        Err(EvalexprError::IllegalEscapeSequence("\\x".to_string()))
    );
    assert_eq!(
        eval("\"\\"),
        Err(EvalexprError::IllegalEscapeSequence("\\".to_string()))
    );
}

#[test]
fn test_unmatched_partial_tokens() {
    assert_eq!(
        eval("|"),
        Err(EvalexprError::UnmatchedPartialToken {
            first: PartialToken::VerticalBar,
            second: None
        })
    );
}

#[test]
fn test_node_mutable_access() {
    let mut node = build_operator_tree::<DefaultNumericTypes>("5").unwrap();
    assert_eq!(node.children_mut().len(), 1);
    assert_eq!(*node.operator_mut(), Operator::RootNode);
}

#[test]
fn test_tree_combining() {
    let a = build_operator_tree::<DefaultNumericTypes>("1").unwrap();
    let b = build_operator_tree::<DefaultNumericTypes>("4").unwrap();

    let combined = combine_trees(a.clone(), b, Operator::Add).unwrap();
    assert_eq!(combined.eval(), Ok(Value::Int(5)));

    let c = build_operator_tree::<DefaultNumericTypes>("4+2").unwrap();
    assert_eq!(
        combine_trees(a, c, Operator::Add).unwrap().eval(),
        Ok(Value::Int(7))
    );
}
