#![cfg(not(tarpaulin_include))]
#![cfg(feature = "regex_support")]

use evalexpr::*;

#[test]
fn test_regex_functions() {
    assert_eq!(
        eval("str::regex_matches(\"foobar\", \"[ob]{3}\")"),
        Ok(Value::Boolean(true))
    );
    assert_eq!(
        eval("str::regex_matches(\"gazonk\", \"[ob]{3}\")"),
        Ok(Value::Boolean(false))
    );
    match eval("str::regex_matches(\"foo\", \"[\")") {
        Err(EvalexprError::InvalidRegex { regex, message }) => {
            assert_eq!(regex, "[");
            assert!(message.contains("unclosed character class"));
        },
        v => panic!("{:?}", v),
    };
    assert_eq!(
        eval("str::regex_replace(\"foobar\", \".*?(o+)\", \"b$1\")"),
        Ok(Value::String("boobar".to_owned()))
    );
    assert_eq!(
        eval("str::regex_replace(\"foobar\", \".*?(i+)\", \"b$1\")"),
        Ok(Value::String("foobar".to_owned()))
    );
}
