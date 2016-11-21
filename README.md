
eval
====
[![docs](https://docs.rs/eval/badge.svg?version=0.3.1 "docs")](https://docs.rs/eval)

Eval is a powerful expression evaluator.

## [Document](https://docs.rs/eval)

## Features
Supported operators: `!` `!=` `""` `''` `()` `[]` `,` `>` `<` `>=` `<=` `==`
`+` `-` `*` `/` `%` `&&` `||` `n..m`.

Built-in functions: `min()` `max()` `len()` `is_empty()`.

## Where can eval be used?
* Template engine
* ...

## Usage
Add dependency to Cargo.toml

```toml
[dependencies]
eval = "^0.3"
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
use eval::{Expr, to_value};

assert_eq!(Expr::new("foo == bar")
               .value("foo", true)
               .value("bar", true)
               .exec(),
           Ok(to_value(true)));
```


You can access data like javascript by using `.` and `[]`. `[]` supports expression.

```
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

```
use eval::{Expr, to_value};

assert_eq!(Expr::new("say_hello()")
               .function("say_hello", |_| Ok(to_value("Hello world!")))
               .exec(),
           Ok(to_value("Hello world!")));
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
