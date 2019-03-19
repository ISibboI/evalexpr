# evalexpr

[![docs](https://docs.rs/evalexpr/badge.svg?version=0.4.4 "docs")](https://docs.rs/evalexpr)

Evalexpr is a powerful arithmetic and boolean expression evaluator.

## [Documentation](https://docs.rs/evalexpr)

<!-- cargo-sync-readme start -->


 ## Features

 Supported binary operators:

 | Operator | Description |
 |----------|-------------|
 | + | Sum |
 | - | Difference |
 | * | Product |
 | / | Division |
 | % | Modulo |
 | < | Lower than |
 | \> | Greater than |
 | <= | Lower than or equal |
 | \>= | Greater than or equal |
 | == | Equal |
 | != | Not equal |
 | `&&` | Logical and |
 | `||` | Logical or |
 
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
