
eval
===
[![docs](https://docs.rs/eval/badge.svg?version=0.1.1 "docs")](https://docs.rs/eval)

Eval is a powerful expression evaluator.

## [Document](https://docs.rs/eval)

## Features
Supported operators: `!` `!=` `""` `''` `()` `[]` `,` `>` `<` `>=` `<=` `==`
`+` `-` `*` `/` `%` `&&` `||` `n..m`.

Built-in functions: `min()` `max()` `is_empty()`.

## Where can eval be used?
* Template engine
* ...

## Usage
Add dependency to Cargo.toml

```toml
[dependencies]
eval = "^0.1"
```

In your `main.rs` or `lib.rs`:

```rust
extern crate eval;
```

## Examples

You can do mathematical calculations with supported operators:

```
use eval::{eval, to_value};

assert_eq!(eval("1 + 2 + 3"), Ok(to_value(6)));
assert_eq!(eval("2 * 2 + 3"), Ok(to_value(7)));
assert_eq!(eval("2 / 2 + 3"), Ok(to_value(4.0)));
assert_eq!(eval("2 / 2 + 3 / 3"), Ok(to_value(2.0)));
```

You can eval with context:

```
use eval::{eval_with_context, Context, to_value};

let mut context = Context::new();
context.insert("foo".to_owned(), to_value(true));
context.insert("bar".to_owned(), to_value(true));
assert_eq!(eval_with_context("foo == bar", &context), Ok(to_value(true)));
```

You can eval with functions:

```
use eval::{eval_with_functions, Functions, Function, to_value};

let mut functions = Functions::new();
functions.insert("say_hello".to_owned(), Function::new(|_| Ok(to_value("Hello world!"))));
assert_eq!(eval_with_functions("say_hello()", &functions), Ok(to_value("Hello world!")));
```

You can create an array with `[]`:

```
use eval::{eval, to_value};

assert_eq!(eval("[1, 2, 3, 4, 5]"), Ok(to_value(vec![1, 2, 3, 4, 5])));

```

You can create an integer array with `n..m`:

```
use eval::{eval, to_value};

assert_eq!(eval("0..5"), Ok(to_value(vec![0, 1, 2, 3, 4])));

```

## License
eval is primarily distributed under the terms of the MIT license.
See [LICENSE](LICENSE) for details.
