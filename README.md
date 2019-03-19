# evalexpr

[![docs](https://docs.rs/evalexpr/badge.svg?version=0.4.4 "docs")](https://docs.rs/evalexpr)

Evalexpr is a powerful arithmetic and boolean expression evaluator.

## [Documentation](https://docs.rs/evalexpr)

<!-- cargo-sync-readme start -->


## Quickstart

Add `evalexpr` as dependency to your `Cargo.toml`:

```toml
[dependencies]
evalexpr = "0.5"
```

Add the `extern crate` definition to your `main.rs` or `lib.rs`:

```rust
extern crate evalexpr;
```

Then you can use `evalexpr` to evaluate expressions like this:

```rust
use evalexpr::*;

assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
assert_eq!(eval("1 - 2 * 3"), Ok(Value::from(-5)));
assert_eq!(eval("1.0 + 2 * 3"), Ok(Value::from(7.0)));
assert_eq!(eval("true && 4 > 2"), Ok(Value::from(true)));
```

And you can use variables and functions in expressions like this:

```rust
use evalexpr::*;

let mut configuration = HashMapConfiguration::new();
configuration.insert_variable("five", 5);
configuration.insert_variable("twelve", 12);
configuration.insert_function("f", Function::new(1 /* argument amount */, Box::new(|arguments| {
    if let Value::Int(int) = arguments[0] {
        Ok(Value::Int(int / 2))
    } else if let Value::Float(float) = arguments[0] {
        Ok(Value::Float(float / 2.0))
    } else {
        Err(Error::expected_number(arguments[0].clone()))
    }
})));

assert_eq!(eval_with_configuration("five + 8 > f(twelve)", &configuration), Ok(Value::from(true)));
```

You can also precompile expressions like this:

```rust
use evalexpr::*;

let precompiled = build_operator_tree("a * b - c > 5").unwrap();

let mut configuration = HashMapConfiguration::new();
configuration.insert_variable("a", 6);
configuration.insert_variable("b", 2);
configuration.insert_variable("c", 3);
assert_eq!(precompiled.eval(&configuration), Ok(Value::from(true)));

configuration.insert_variable("c", 8);
assert_eq!(precompiled.eval(&configuration), Ok(Value::from(false)));
```

## Features

### Operators

Supported binary operators:

| Operator | Precedence | Description |   | Operator | Precedence | Description |
|----------|------------|-------------|---|----------|------------|-------------|
| + | 95 | Sum | | < | 80 | Lower than |
| - | 95 | Difference | | \> | 80 | Greater than |
| * | 100 | Product | | <= | 80 | Lower than or equal |
| / | 100 | Division | | \>= | 80 | Greater than or equal |
| % | 100 | Modulo | | == | 80 | Equal |
| && | 75 | Logical and | | != | 80 | Not equal |
| &#124;&#124; | 70 | Logical or | | | |

Supported unary operators:

| Operator | Precedence | Description |
|----------|------------|-------------|
| - | 110 | Negation |
| ! | 110 | Logical not |

### Values

Operators take values as arguments and produce values as results.
Values can be boolean, integer or floating point numbers.
Strings are supported as well, but there are no operations defined for them yet.
Values are denoted as displayed in the following table.

| Value type | Example |
|------------|---------|
| `Value::Boolean` | `true`, `false` |
| `Value::Int` | `3`, `-9`, `0`, `135412` |
| `Value::Float` | `3.`, `.35`, `1.00`, `0.5`, `123.554` |

Integers are internally represented as `i64`, and floating point numbers are represented as `f64`.
Operators that take numbers as arguments can either take integers or floating point numbers.
If one of the arguments is a floating point number, all others are converted to floating point numbers as well, and the resulting value is a floating point number as well.
Otherwise, the result is an integer.

### Variables

This crate allows to compile parameterizable formulas by using variables.
A variable is a literal in the formula, that does not contain whitespace or can be parsed as value.
The user needs to provide bindings to the variables for evaluation.
This is done with the `Configuration` trait.
Two structs implementing this trait are predefined.
There is `EmptyConfiguration`, that returns `None` for each request, and `HashMapConfiguration`, that stores mappings from literals to variables in a hash map.

Variables do not have fixed types in the expression itself, but aer typed by the configuration.
The `Configuration` trait contains a function that takes a string literal and returns a `Value` enum.
The variant of this enum decides the type on evaluation.

### Functions

This crate also allows to define arbitrary functions to be used in parsed expressions.
A function is defined as a `Function` instance.
It contains two properties, the `argument_amount` and the `function`.
The `function` is a boxed `Fn(&[Value]) -> Result<Value, Error>`.
The `argument_amount` is verified on execution by the crate and does not need to be verified by the `function`.
It determines the length of the slice that is passed to `function`.
See the examples section above for examples on how to construct a function instance.

## License

This crate is primarily distributed under the terms of the MIT license.
See [LICENSE](LICENSE) for details.


<!-- cargo-sync-readme end -->
