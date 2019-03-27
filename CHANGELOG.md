# Change Log

## Unreleased

### Added

 * Add the `ContextMut` trait, that is a manipulable configuration/context
 * Add `ContextNotManipulable` error variant for the `EmptyContext`
 * Make the `TupleType` alias public
 * Add the `ValueType` enum that represents the type of a value for easier comparisons and matchings
 * Add `EvalexprResult<T>` type that uses the `EvalexprError` type (renamed from `Error`)
 * Add `Node::eval_number` and `Node::eval_number_with_context` to evaluate to int or float and silently converting to float

### Removed

### Changed

 * Get rid of some unwraps to improve safety
 * Rename `Error` to `EvalexprError`
 * Rename `Configuration` to `Context`
 * Rename `HashMapConfiguration` to `HashMapContext` and `EmptyConfiguration` to `EmptyContext`
 * Rename `Value::as_float` to `Value::as_number` and add new `Value::as_float` that fails if value is an integer

### Fixed

### Deprecated

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