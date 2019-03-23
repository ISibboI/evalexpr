# Change Log

## Unreleased

### Added

 * Add `serde` feature

### Removed

### Changed

 * Highlighing in documentation

### Fixed

### Deprecated

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