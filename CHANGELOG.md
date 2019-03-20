# Change Log

## [Unreleased]

### Added

 * Internal aliases `IntType` and `FloatType` used by the `Value` enum are now public
 * Type alias `TupleType` used to represent tuples was added
 * Error types like `Error::ExpectedInt` for expecting each value type were added
 * Shortcut functions like `eval_int` or `eval_int_with_configuration` to evaluate directly into a value type were added
 * Documentation for the shortcut functions was added

### Removed

 * Integration tests were removed from shipped crate

### Changed

### Fixed

 * Wording of some documentation items was changed to improve readability

### Deprecated

## [1.0.0](https://github.com/ISibboI/evalexpr/tree/1.0.0) - 2019-03-20

 * First stable release