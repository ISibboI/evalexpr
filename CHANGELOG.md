# Change Log

## Unreleased

### Notes

### Added

 * Builtin functions to check for nan, infinity and subnormality in floats (#101)

### Removed

### Changed

### Fixed

### Deprecated

### Contributors

My warmhearted thanks goes to:

 * [Ophir LOJKINE](https://github.com/lovasoa)

## [7.2.0](https://github.com/ISibboI/evalexpr/compare/7.1.1...7.2.0) - 2022-03-16

### Added

 * The builtin function `if`, which mimics the if-else construct existing in many programming languages.

### Contributors

My warmhearted thanks goes to:

 * [Ophir LOJKINE](https://github.com/lovasoa)

## [7.1.1](https://github.com/ISibboI/evalexpr/compare/7.1.0...7.1.1) - 2022-03-14

### Fixed

 * Set regex minimum version to `1.5.5`, as the previous versions contains a security vulnerability.
   See https://groups.google.com/g/rustlang-security-announcements/c/NcNNL1Jq7Yw?pli=1.
   This vulnerability does not affect this crate as of now, but if we ever allow passing parameters to the regex engine, it might.

## [7.1.0](https://github.com/ISibboI/evalexpr/compare/7.0.1...7.1.0) - 2022-03-13

### Added

 * Bit shift functions `shl` and `shr`, same as Rust's shift functions on `i64`.

### Contributors

My warmhearted thanks goes to

 * [Diane Sparks](https://github.com/FractalDiane)

## [7.0.1](https://github.com/ISibboI/evalexpr/compare/7.0.0...7.0.1) - 2022-02-20

### Changed

 * Updated the optional dependencies, and fixed them to a minimum tested version.
   For simplicity, I fixed them to the newest version, but since I export none of them, this is luckily not breaking.
 * Updated the dev-dependencies.

## [7.0.0](https://github.com/ISibboI/evalexpr/compare/6.6.0...7.0.0) - 2022-01-13

### Changed

 * Made the `EvalexprError` enum `non_exhaustive`.

### Fixed

 * Expressions that have dangling parenthese expressions such as `4(5)` now produce an error.

### Contributors

My warmhearted thanks goes to

 * [dbr/Ben](https://github.com/dbr)

## [6.6.0](https://github.com/ISibboI/evalexpr/compare/6.5.0...6.6.0) - 2021-10-13

### Added

 * Bitwise operators as builtin functions `bitand`, `bitor`, `bitxor`, `bitnot` (#88)
 * Public immutable and mutable accessor functions to the operator and children of a Node.

### Contributors

My warmhearted thanks goes to

 * [Michał Hanusek](https://github.com/hanusek)
 * [Kai Giebeler](https://github.com/kawogi)

## [6.5.0](https://github.com/ISibboI/evalexpr/compare/6.4.0...6.5.0) - 2021-08-16

### Added

 * Make `Function::new` able to accept closures (thanks to Jakub Dąbek)

### Contributors

My warmhearted thanks goes to

 * [Jakub Dąbek](https://github.com/jakubdabek)
 * [LonnonjamesD](https://github.com/LonnonjamesD)

## [6.4.0](https://github.com/ISibboI/evalexpr/compare/6.3.0...6.4.0) - 2021-07-21

### Notes

 * Minimum supported Rust version (MSRV) increased to `1.46.0`
 * Increased test coverage by adding more test and ignoring untestable files

### Added

 * Allow scientific notation in float literals

### Changed
 
 * Made some functions `const`. This increased the MSRV

### Fixed

 * `eval_number` methods returned `EvalexprError::ExpectedFloat` before, now they correctly return `EvalexprError::ExpectedNumber`

### Contributors

My warmhearted thanks goes to

 * [Dennis Marttinen](https://github.com/twelho)

## [6.3.0](https://github.com/ISibboI/evalexpr/compare/6.2.1...6.3.0) - 2021-07-06

### Added

 * Implement more builtin math methods

### Contributors

My warmhearted thanks goes to

 * [Magnus Ulimoen](https://github.com/mulimoen)

## [6.2.0](https://github.com/ISibboI/evalexpr/compare/6.1.1...6.2.0) - 2021-06-24

### Notes

 * Increased test coverage

### Added

 * Implemented `Clone` for `HashMapContext`

### Contributors

My warmhearted thanks goes to

 * [Magnus Ulimoen](https://github.com/mulimoen)

## [6.1.1](https://github.com/ISibboI/evalexpr/compare/6.1.0...6.1.1) - 2021-06-22

### Fixed

 * Improved syntax of documentation

## [6.1.0](https://github.com/ISibboI/evalexpr/compare/6.0.0...6.1.0) - 2021-06-02

### Added

 * Macro `math_consts_context` adding all of Rust's `f64` constants
 * All common math functions implemented by Rust's `f64` are now builtin
 * Continuous integration and test coverage report

### Contributors

My warmhearted thanks goes to

 * [Edwin](https://github.com/olback)

## [6.0.0](https://github.com/ISibboI/evalexpr/compare/5.1.0...6.0.0) - 2021-05-28

### Added

 * `#![forbid(unsafe_code)]`
 * Made `Function` derive `Clone`
 * Ensure that `Function` implements `Send` and `Sync`

### Removed

 * `Cargo.lock`

### Changed

 * Decomposed `Context` into `Context`, `ContextWithMutableVariables` and `ContextWithMutableFunctions`
 * Replaced the `get_function` method of `Context` with a `call_function` method

### Contributors

My warmhearted thanks goes to

 * [dvtomas](https://github.com/dvtomas)

## [5.1.0](https://github.com/ISibboI/evalexpr/compare/5.0.5...5.1.0) - 2021-05-28

### Added

 * Make `Node` cloneable

### Contributors

My warmhearted thanks goes to

 * [dvtomas](https://github.com/dvtomas)

## [5.0.5](https://github.com/ISibboI/evalexpr/compare/5.0.4...5.0.5) - 2019-09-13

### Fixed

 * is-it-maintained badges had wrong repository definitions
 * maintenance status was given wrongly
 * move maintenance status to top

## [5.0.4](https://github.com/ISibboI/evalexpr/compare/5.0.3...5.0.4) - 2019-09-13

### Added

 * maintenance badge
 * is-it-maintained badges

## [5.0.3](https://github.com/ISibboI/evalexpr/compare/5.0.2...5.0.3) - 2019-08-30

### Fixed

 * The `!=` operator was wrongfully parsed as Token::Eq

### Contributors

 * [slientgoat](https://github.com/slientgoat)

## [5.0.2](https://github.com/ISibboI/evalexpr/compare/5.0.1...5.0.2) - 2019-08-30

### Changed

 * Removed target.bench.dev-dependencies completely, as they can be just listed under the normal dev-dependencies

## [5.0.1](https://github.com/ISibboI/evalexpr/compare/5.0.0...5.0.1) - 2019-08-30

### Fixed

 * Bench dependencies are now dev-dependencies so they are not listed on crates.io as normal dependencies anymore

## [5.0.0](https://github.com/ISibboI/evalexpr/compare/4.1.0...5.0.0) - *'Sanity'* - 2019-08-30

### Notes

Finally, 'Sanity' has been released, including a huge bunch of new features.
Notably, and providing a reason for the name of this release, function call and tuple semantics have improved a lot.
Functions now always take exactly one argument, but this can then be a tuple.
It is now possible to construct tuples of tuples, such that mode complex values can be constructed.
As of now there is no way to deconstruct them though.

A lot has been done on string processing, special thanks for that goes to [bittrance](https://github.com/bittrance).
Specifically, under the feature flag `regex_support` two regex functions for strings are hiding now.
Also, the operators `+` and comparison operators have been fitted to support strings.

Thanks to [lovasoa](https://github.com/lovasoa), we now have a nice macro for context creation.

Thanks to [Atul9](https://github.com/Atul9), the crate is now Rust 2018 compliant.

Thanks to [mestachs'](https://github.com/mestachs) request, we now have functions to iterate over identifiers within an expression.

Internally, the structure of the operator tree changed from being `&dyn`-based to being `enum`-based.
Also, we have benchmarks now to observe performance changes in future releases.  

### Added

 * Iterator over all identifiers within an expression, including duplicates
 * Iterators over only variable or only function identifiers within an expression, including duplicates
 * Overload the `+` operator to concatenate strings
 * Overload `<`, `<=`, `>` and `>=` for strings using lexical ordering (Note: `==` and `!=` compare strings as expected)
 * Add `len`, `str::regex_matches`, `str::regex_replace`, `str::to_lowercase`, `str::to_uppercase`, `str::trim` functions for strings
 * Add a macro for more convenient definition of contexts including the direct definition of static contexts
 * Add API for value decomposition
 * Allow using context operations in `eval` calls without context
 * Operator assignment operators for each binary operation (`+=`, `-=`, ...)
 * The `Operator` enum is now public for better error types
 * Benchmarks for observing performance of future releases

### Removed

 * Function arguments are not decomposed anymore.
   The function implementation will receive exactly one argument now.
   This allows the function to be called on a tuple properly. 

### Changed

 * Operators are an enum now instead of trait objects
 * Update to Rust 2018
 * Updated dependencies

### Fixed

 * Allow variable assignments in eval calls without context.
   A `HashMapContext` is created automatically now.
 * The error string for `ExpectedNumber` was wrong
 * Operators panicked when adding a number to a string
 * Some documentation was not updated for the 4.x releases

### Contributors

My warmhearted thanks goes to 

 * [bittrance](https://github.com/bittrance)
 * [lovasoa](https://github.com/lovasoa)
 * [Atul9](https://github.com/Atul9)
 * [mestachs](https://github.com/mestachs)

## [4.1.0](https://github.com/ISibboI/evalexpr/compare/4.0.0...4.1.0) - 2019-03-31

### Added

 * Export `expect_function_argument_amount`

## [4.0.0](https://github.com/ISibboI/evalexpr/compare/3.1.0...4.0.0) - 2019-03-30

### Added

 * String constants

## [3.1.0](https://github.com/ISibboI/evalexpr/compare/3.0.0...3.1.0) - 2019-03-28

### Added

 * Add serde support to `HashMapContext`
 * Make `HashMapContext` derive `Default` and `Debug`

### Changed

 * Changed name of serde feature flag to `serde_support`

## [3.0.0](https://github.com/ISibboI/evalexpr/compare/2.0.0...3.0.0) - 2019-03-28

### Notes

The 3.0.0 update transforms the expression evaluator `evalexpr` to a tiny scripting language.
It allows assignments and chaining of expressions.
Some changes in this update are breaking, hence the major release.

### Added

 * Methods `Node::eval_<type>_with_context_mut` and crate level `eval_<type>_with_context_mut`
 * Empty type and corresponding shortcut methods. The empty type is emitted by empty expressions or empty subexpressions `()`.
 * The assignment operator `=`
 * The expression chaining operator `;`

### Removed

 * Generic arguments from `Context` traits are now static to allow using trait objects of `Context`
 * `EvalexprError::EmptyExpression` is not required anymore since empty expressions now evaluate to the empty type

### Changed

 * Merge `ContextMut` trait into `Context` trait

## [2.0.0](https://github.com/ISibboI/evalexpr/compare/1.2.0...2.0.0) - 2019-03-28

### Notes

The 2.0.0 update is the first step to transform evalexpr to a tiny scripting language with support of at least variable assignment.
The main change for now is that `Configuration` is called `Context`, which seems to be a more proper naming for a set of variables that can not only be read, but also manipulated via expressions.
This update includes further renamings and some inconsistencies in the API were fixed.
For more details, see the following subsections.

### Added

 * Add the `ContextMut` trait, that is a manipulable configuration/context
 * Add `ContextNotManipulable` error variant for the `EmptyContext`
 * Make the `TupleType` alias public
 * Add the `ValueType` enum that represents the type of a value for easier comparisons and matchings
 * Add `EvalexprResult<T>` type that uses the `EvalexprError` type (renamed from `Error`)
 * Add `Node::eval_number` and `Node::eval_number_with_context` to evaluate to int or float and silently converting to float
 * Add `eval_number` and `eval_number_with_context` crate methods to evaluate to int or float and silently converting to float

### Changed

 * Get rid of some unwraps to improve safety
 * Rename `Error` to `EvalexprError`
 * Rename `Configuration` to `Context`
 * Rename `HashMapConfiguration` to `HashMapContext` and `EmptyConfiguration` to `EmptyContext`
 * Rename `Value::as_float` to `Value::as_number` and add new `Value::as_float` that fails if value is an integer

## [1.2.0](https://github.com/ISibboI/evalexpr/compare/1.1.0...1.2.0) - 2019-03-23

### Added

 * Add `serde` feature
 * Implement `serde::de::Deserialize` for `Node`
 * Document `serde` usage
 * Add custom error type with a `String` message

### Changed

 * Highlighting in documentation

## [1.1.0](https://github.com/ISibboI/evalexpr/compare/1.0.0...1.1.0) - 2019-03-20

### Added

 * Internal aliases `IntType` and `FloatType` used by the `Value` enum are now public
 * Type alias `TupleType` used to represent tuples was added
 * Error types like `Error::ExpectedInt` for expecting each value type were added
 * Shortcut functions like `eval_int` or `eval_int_with_configuration` to evaluate directly into a value type were added
 * Documentation for the shortcut functions was added
 * Functions to decompose `Value`s were added and documented

### Removed

 * Integration tests were removed from shipped crate

### Fixed

 * Wording of some documentation items was changed to improve readability

## [1.0.0](https://github.com/ISibboI/evalexpr/tree/1.0.0) - 2019-03-20

 * First stable release
