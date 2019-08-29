# evalexpr

[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)
[![](http://meritbadge.herokuapp.com/evalexpr)](https://crates.io/crates/evalexpr)
[![](https://docs.rs/evalexpr/badge.svg)](https://docs.rs/evalexpr)

Evalexpr is an expression evaluator and tiny scripting language in Rust.
It has a small and easy to use interface and can be easily integrated into any application.
It is very lightweight and comes with no further dependencies.
Evalexpr is [available on crates.io](https://crates.io/crates/evalexpr), and its [API Documentation is available on docs.rs](https://docs.rs/evalexpr).

<!-- cargo-sync-readme start -->


## Quickstart

Add `evalexpr` as dependency to your `Cargo.toml`:

```toml
[dependencies]
evalexpr = "5"
```

Add the `extern crate` definition to your `main.rs` or `lib.rs`:

```rust
extern crate evalexpr;
```

Then you can use `evalexpr` to **evaluate expressions** like this:

```rust
use evalexpr::*;

assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
// `eval` returns a variant of the `Value` enum,
// while `eval_[type]` returns the respective type directly.
// Both can be used interchangeably.
assert_eq!(eval_int("1 + 2 + 3"), Ok(6));
assert_eq!(eval("1 - 2 * 3"), Ok(Value::from(-5)));
assert_eq!(eval("1.0 + 2 * 3"), Ok(Value::from(7.0)));
assert_eq!(eval("true && 4 > 2"), Ok(Value::from(true)));
```

You can **chain** expressions and **assign** to variables like this:

```rust
use evalexpr::*;

let mut context = HashMapContext::new();
// Assign 5 to a like this
assert_eq!(eval_empty_with_context_mut("a = 5", &mut context), Ok(EMPTY_VALUE));
// The HashMapContext is type safe, so this will fail now
assert_eq!(eval_empty_with_context_mut("a = 5.0", &mut context), Err(EvalexprError::expected_int(Value::from(5.0))));
// We can check which value the context stores for a like this
assert_eq!(context.get_value("a"), Some(&Value::from(5)));
// And use the value in another expression like this
assert_eq!(eval_int_with_context_mut("a = a + 2; a", &mut context), Ok(7));
```

And you can use **variables** and **functions** in expressions like this:

```rust
use evalexpr::*;

let context = context_map! {
    "five" => 5,
    "twelve" => 12,
    "f" => Function::new(Box::new(|argument| {
        if let Ok(int) = argument.as_int() {
            Ok(Value::Int(int / 2))
        } else if let Ok(float) = argument.as_float() {
            Ok(Value::Float(float / 2.0))
        } else {
            Err(EvalexprError::expected_number(argument.clone()))
        }
    })),
    "avg" => Function::new(Box::new(|argument| {
        let arguments = argument.as_tuple()?;

        if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
            Ok(Value::Int((a + b) / 2))
        } else {
            Ok(Value::Float((arguments[0].as_number()? + arguments[1].as_number()?) / 2.0))
        }
    }))
}.unwrap(); // Do proper error handling here

assert_eq!(eval_with_context("five + 8 > f(twelve)", &context), Ok(Value::from(true)));
// `eval_with_context` returns a variant of the `Value` enum,
// while `eval_[type]_with_context` returns the respective type directly.
// Both can be used interchangeably.
assert_eq!(eval_boolean_with_context("five + 8 > f(twelve)", &context), Ok(true));
assert_eq!(eval_with_context("avg(2, 4) == 3", &context), Ok(Value::from(true)));
```

You can also **precompile** expressions like this:

```rust
use evalexpr::*;

let precompiled = build_operator_tree("a * b - c > 5").unwrap(); // Do proper error handling here

let mut context = context_map! {
    "a" => 6,
    "b" => 2,
    "c" => 3
}.unwrap(); // Do proper error handling here
assert_eq!(precompiled.eval_with_context(&context), Ok(Value::from(true)));

context.set_value("c".into(), 8.into()).unwrap(); // Do proper error handling here
assert_eq!(precompiled.eval_with_context(&context), Ok(Value::from(false)));
// `Node::eval_with_context` returns a variant of the `Value` enum,
// while `Node::eval_[type]_with_context` returns the respective type directly.
// Both can be used interchangeably.
assert_eq!(precompiled.eval_boolean_with_context(&context), Ok(false));
```

## Features

### Operators

This crate offers a set of binary and unary operators for building expressions.
Operators have a precedence to determine their order of evaluation.
The precedence should resemble that of most common programming languages, especially Rust.
The precedence of variables and values is 200, and the precedence of function literals is 190.

Supported binary operators:

| Operator | Precedence | Description |
|----------|------------|-------------|
| ^ | 120 | Exponentiation |
| * | 100 | Product |
| / | 100 | Division |
| % | 100 | Modulo |
| + | 95 | Sum or String Concatenation |
| - | 95 | Difference |
| < | 80 | Lower than |
| \> | 80 | Greater than |
| <= | 80 | Lower than or equal |
| \>= | 80 | Greater than or equal |
| == | 80 | Equal |
| != | 80 | Not equal |
| && | 75 | Logical and |
| &#124;&#124; | 70 | Logical or |
| = | 50 | Assignment |
| , | 40 | Aggregation |
| ; | 0 | Expression Chaining |

Supported unary operators:

| Operator | Precedence | Description |
|----------|------------|-------------|
| - | 110 | Negation |
| ! | 110 | Logical not |

#### The Aggregation Operator

The aggregation operator aggregates two values into a tuple.
If one of the values is a tuple already, the resulting tuple will be flattened.
Example:

```rust
use evalexpr::*;

assert_eq!(eval("1, 2, 3"), Ok(Value::from(vec![Value::from(1), Value::from(2), Value::from(3)])));
```

#### The Assignment Operator

This crate features the assignment operator, that allows expressions to store their result in a variable in the expression context.
If an expression uses the assignment operator, it must be evaluated with a mutable context.
Note that assignments are type safe, meaning if an identifier is assigned a value of a type once, it cannot be assigned a value of another type.

```rust
use evalexpr::*;

let mut context = HashMapContext::new();
assert_eq!(eval_with_context("a = 5", &context), Err(EvalexprError::ContextNotManipulable));
assert_eq!(eval_empty_with_context_mut("a = 5", &mut context), Ok(EMPTY_VALUE));
assert_eq!(eval_empty_with_context_mut("a = 5.0", &mut context), Err(EvalexprError::expected_int(5.0.into())));
assert_eq!(eval_int_with_context("a", &context), Ok(5));
assert_eq!(context.get_value("a"), Some(5.into()).as_ref());
```

#### The Expression Chaining Operator

The expression chaining operator works as one would expect from programming languages that use the semicolon to end statements, like `Rust`, `C` or `Java`.
It has the special feature that it returns the value of the last expression in the expression chain.
If the last expression is terminated by a semicolon as well, then `Value::Empty` is returned.
Expression chaining is useful together with assignment to create small scripts.

```rust
use evalexpr::*;

let mut context = HashMapContext::new();
assert_eq!(eval("1;2;3;4;"), Ok(Value::Empty));
assert_eq!(eval("1;2;3;4"), Ok(4.into()));

// Initialization of variables via script.
assert_eq!(eval_empty_with_context_mut("hp = 1; max_hp = 5; heal_amount = 3;", &mut context), Ok(EMPTY_VALUE));
// Precompile healing script.
let healing_script = build_operator_tree("hp = min(hp + heal_amount, max_hp); hp").unwrap(); // Do proper error handling here
// Execute precompiled healing script.
assert_eq!(healing_script.eval_int_with_context_mut(&mut context), Ok(4));
assert_eq!(healing_script.eval_int_with_context_mut(&mut context), Ok(5));
```

### Builtin Functions

This crate offers a set of builtin functions.

| Identifier | Argument Amount | Argument Types | Description |
|------------|-----------------|----------------|-------------|
| `min` | >= 1 | Numeric | Returns the minimum of the arguments |
| `max` | >= 1 | Numeric | Returns the maximum of the arguments |
| `len` | 1 | String | Returns the character length of a string |
| `str::regex_matches` | 2 | String, String | Returns true if the first argument matches the regex in the second argument |
| `str::regex_replace` | 3 | String, String, String | Returns the first argument with all matches of the regex in the second argument replaced by the third argument |
| `str::to_lowercase` | 1 | String | Returns the lower-case version of the string |
| `str::to_uppercase` | 1 | String | Returns the upper-case version of the string |
| `str::trim` | 1 | String | Strips whitespace from the start and the end of the string |

The `min` and `max` functions can deal with a mixture of integer and floating point arguments.
If the maximum or minimum is an integer, then an integer is returned.
Otherwise, a float is returned.

The regex functions require the feature flag `regex_support`.

### Values

Operators take values as arguments and produce values as results.
Values can be boolean, integer or floating point numbers, strings, tuples or the empty type.
Values are denoted as displayed in the following table.

| Value type | Example |
|------------|---------|
| `Value::String` | `"abc"`, `""`, `"a\"b\\c"` |
| `Value::Boolean` | `true`, `false` |
| `Value::Int` | `3`, `-9`, `0`, `135412` |
| `Value::Float` | `3.`, `.35`, `1.00`, `0.5`, `123.554` |
| `Value::Tuple` | `(3, 55.0, false, ())`, `(1, 2)` |
| `Value::Empty` | `()` |

Integers are internally represented as `i64`, and floating point numbers are represented as `f64`.
Tuples are represented as `Vec<Value>` and empty values are not stored, but represented by rust's unit type `()` where necessary.

There exist type aliases for some of the types.
They include `IntType`, `FloatType`, `TupleType` and `EmptyType`.

Values can be constructed either directly or using the `From` trait.
Values can be decomposed using the `Value::as_[type]` methods.
The type of a value can be checked using the `Value::is_[type]` methods.

**Examples for constructing a value:**

| Code | Result |
|------|--------|
| `Value::from(4)` | `Value::Int(4)` |
| `Value::from(4.4)` | `Value::Float(4.4)` |
| `Value::from(true)` | `Value::Boolean(true)` |
| `Value::from(vec![Value::from(3)])` | `Value::Tuple(vec![Value::Int(3)])` |

**Examples for deconstructing a value:**

| Code | Result |
|------|--------|
| `Value::from(4).as_int()` | `Ok(4)` |
| `Value::from(4.4).as_float()` | `Ok(4.4)` |
| `Value::from(true).as_int()` | `Err(Error::ExpectedInt {actual: Value::Boolean(true)})` |

Operators that take numbers as arguments can either take integers or floating point numbers.
If one of the arguments is a floating point number, all others are converted to floating point numbers as well, and the resulting value is a floating point number as well.
Otherwise, the result is an integer.
An exception to this is the exponentiation operator that always returns a floating point number.

Values have a precedence of 200.

### Variables

This crate allows to compile parameterizable formulas by using variables.
A variable is a literal in the formula, that does not contain whitespace or can be parsed as value.
The user needs to provide bindings to the variables for evaluation.
This is done with the `Context` trait.
Two structs implementing this trait are predefined.
There is `EmptyContext`, that returns `None` for each request, and `HashMapContext`, that stores mappings from literals to variables in a hash map.

Variables do not have fixed types in the expression itself, but are typed by the context.
The `Context` trait contains a function that takes a string literal and returns a `Value` enum.
The variant of this enum decides the type on evaluation.

Variables have a precedence of 200.

### User-Defined Functions

This crate also allows to define arbitrary functions to be used in parsed expressions.
A function is defined as a `Function` instance.
It contains two properties, the `argument_amount` and the `function`.
The `function` is a boxed `Fn(&[Value]) -> EvalexprResult<Value, Error>`.
The `argument_amount` determines the length of the slice that is passed to `function` if it is `Some(_)`, otherwise the function is defined to take an arbitrary amount of arguments.
It is verified on execution by the crate and does not need to be verified by the `function`.

Functions with no arguments are not allowed.
Use variables instead.

Be aware that functions need to verify the types of values that are passed to them.
The `error` module contains some shortcuts for verification, and error types for passing a wrong value type.
Also, most numeric functions need to differentiate between being called with integers or floating point numbers, and act accordingly.

Functions are identified by literals, like variables as well.
A literal identifies a function, if it is followed by an opening brace `(`, another literal, or a value.

Same as variables, function bindings are provided by the user via a `Context`.
Functions have a precedence of 190.

### Examplary variables and functions in expressions:

| Expression | Valid? | Explanation |
|------------|--------|-------------|
| `a` | yes | |
| `abc` | yes | |
| `a<b` | no | Expression is interpreted as variable `a`, operator `<` and variable `b` |
| `a b` | no | Expression is interpreted as function `a` applied to argument `b` |
| `123` | no | Expression is interpreted as `Value::Int` |
| `true` | no | Expression is interpreted as `Value::Bool` |
| `.34` | no | Expression is interpreted as `Value::Float` |

### [Serde](https://serde.rs)

To use this crate with serde, the `serde_support` feature flag has to be set.
This can be done like this in the `Cargo.toml`:

```toml
[dependencies]
evalexpr = {version = "5", features = ["serde_support"]}
```

This crate implements `serde::de::Deserialize` for its type `Node` that represents a parsed expression tree.
The implementation expects a [serde `string`](https://serde.rs/data-model.html) as input.
Example parsing with [ron format](docs.rs/ron):

```rust
extern crate ron;
use evalexpr::*;

let mut context = context_map!{
    "five" => 5
}.unwrap(); // Do proper error handling here

// In ron format, strings are surrounded by "
let serialized_free = "\"five * five\"";
match ron::de::from_str::<Node>(serialized_free) {
    Ok(free) => assert_eq!(free.eval_with_context(&context), Ok(Value::from(25))),
    Err(error) => {
        () // Handle error
    }
}
```

With `serde`, expressions can be integrated into arbitrarily complex data.

The crate also implements `Serialize` and `Deserialize` for the `HashMapContext`.
But note that only the variables get serialized, not the functions.

## License

This crate is primarily distributed under the terms of the MIT license.
See [LICENSE](LICENSE) for details.


<!-- cargo-sync-readme end -->

## No Panicking

This crate makes extensive use of the `Result` pattern and is intended to never panic.
The *exception* are panics caused by *failed allocations*.
But unfortunately, Rust does not provide any features to prove this behavior.
The developer of this crate has not found a good solution to ensure no-panic behavior in any way.
Please report a panic in this crate immediately as issue on [github](https://github.com/ISibboI/evalexpr/issues).

Even if the crate itself is panic free, it allows the user to define custom functions that are executed by the crate.
The user needs to ensure that the function he provides to the crate never panic.

## Contribution

If you have any ideas for features or see any problems in the code, architecture, interface, algorithmics or documentation, please open an issue on github.
If there is already an issue describing what you want to say, please add a thumbs up or whatever emoji you think fits to the issue, so I know which ones I should prioritize.

**Notes for contributors:**

 * This crate uses the [`sync-readme`](https://github.com/phaazon/cargo-sync-readme) cargo subcommand to keep the documentation in `src/lib.rs` and `README.md` in sync.
   The subcommand only syncs from the documentation in `src/lib.rs` to `README.md`.
   So please alter the documentation in the `src/lib.rs` rather than altering anything in between `<!-- cargo-sync-readme start -->` and `<!-- cargo-sync-readme end -->` in the `README.md`.