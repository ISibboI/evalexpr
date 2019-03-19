# evalexpr

[![docs](https://docs.rs/evalexpr/badge.svg?version=0.4.4 "docs")](https://docs.rs/evalexpr)

Evalexpr is a powerful arithmetic and boolean expression evaluator.

## [Documentation](https://docs.rs/evalexpr)

<!-- cargo-sync-readme start -->


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

 Integers are internally represented as `i64`, and floating point numbers are represented as `f64`.
 Operators that take numbers as arguments can either take integers or floating point numbers.
 If one of the arguments is a floating point number, all others are converted to floating point numbers as well, and the resulting value is a floating point number as well.
 Otherwise, the result is an integer.

 ### Variables





 
Supported binary operators: `!` `!=` `""` `''` `()` `[]` `,` `>` `<` `>=` `<=` `==`
`+` unary/binary `-` `*` `/` `%` `&&` `||` `n..m`.

Supported unary operators: ``

Built-in functions: `min()` `max()` `len()` `is_empty()` `array()` `converge()`.
See the `builtin` module for a detailed description of each.

Where can eval be used?
-----------------------

* Template engine
* Scripting language
* ...

Usage
-----

Add dependency to Cargo.toml

```toml
[dependencies]
evalexpr = "0.4"
```

In your `main.rs` or `lib.rs`:

```rust
extern crate evalexpr as eval;
```

Examples
--------

You can do mathematical calculations with supported operators:

```rust
use eval::{eval, to_value};

assert_eq!(eval("1 + 2 + 3"), Ok(to_value(6)));
assert_eq!(eval("2 * 2 + 3"), Ok(to_value(7)));
assert_eq!(eval("2 / 2 + 3"), Ok(to_value(4.0)));
assert_eq!(eval("2 / 2 + 3 / 3"), Ok(to_value(2.0)));
```

You can eval with context:

```rust
use eval::{Expr, to_value};

assert_eq!(Expr::new("foo == bar")
               .value("foo", true)
               .value("bar", true)
               .exec(),
           Ok(to_value(true)));
```

You can access data like javascript by using `.` and `[]`. `[]` supports expression.

```rust
use eval::{Expr, to_value};
use std::collections::HashMap;

let mut object = HashMap::new();
object.insert("foos", vec!["Hello", "world", "!"]);

assert_eq!(Expr::new("object.foos[1-1] == 'Hello'")
               .value("object", object)
               .exec(),
           Ok(to_value(true)));
```

You can eval with function:

```rust
use eval::{Expr, to_value};

assert_eq!(Expr::new("say_hello()")
               .function("say_hello", |_| Ok(to_value("Hello world!")))
               .exec(),
           Ok(to_value("Hello world!")));
```

You can create an array with `array()`:

```rust
use eval::{eval, to_value};

assert_eq!(eval("array(1, 2, 3, 4, 5)"), Ok(to_value(vec![1, 2, 3, 4, 5])));
```

You can create an integer array with `n..m`:

```rust
use eval::{eval, to_value};

assert_eq!(eval("0..5"), Ok(to_value(vec![0, 1, 2, 3, 4])));
```

License
-------

evalexpr is primarily distributed under the terms of the MIT license.
See [LICENSE](LICENSE) for details. 


<!-- cargo-sync-readme end -->
