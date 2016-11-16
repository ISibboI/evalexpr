
use serde_json::Value;
use operator::Operator;


quick_error! {
    /// Expression parsing error
    #[derive(Debug, PartialEq)]
    pub enum Error {
        /// Unsupported operator yet.
        UnsupportedOperator(operator: String) {
            display("Unsupported operator: {:?}", operator)
        }
        /// This operator does not support execution.
        CanNotExec(operator: Operator) {
            display("This operator does not support execution: {:?}", operator)
        }
        /// Your expression may start with non-value operator like ( + * )
        StartWithNonValueOperator {
            display("Your expression may start with non-value operator like ( + * ).")
        }
        /// Unpaired brackets, left brackets count does not equal right brackets count
        UnpairedBrackets {
            display("Unpaired brackets, left brackets count does not equal right brackets count.")
        }
        /// Duplicate values node, you may have (2 3) but there is no operators between them
        DuplicateValueNode {
            display("Duplicate values node, you may have (2 3) but there is no operators between them.")
        }
        /// Duplicate operators node, you may have (+ +) but there is no values between them
        DuplicateOperatorNode {
            display("Duplicate operators node, you may have (+ +) but there is no values between them.")
        }
        /// You have a comma(,) , but there is no function in front of it.
        CommaNotWithFunction {
            display("You have a comma(,) , but there is no function in front of it.")
        }
        /// You have empty brackets () , but there is no function in front of it.
        BracketNotWithFunction {
            display("You have empty brackets () , but there is no function in front of it.")
        }
        /// Function not exists.
        FunctionNotExists(ident: String) {
            display("Function not exists: {}", ident)
        }
        /// Expected boolean type but the given value isn't.
        NotBoolean(value: Value) {
            display("Expected boolean type, found: {}", value)
        }
        /// Failed to parse, no final expression.
        NoFinalNode {
            display("Failed to parse, no final expression.")
        }
        /// The number of arguments is greater than the maximum limit.
        ArgumentsGreater(max: usize) {
            display("The number of arguments is greater than the maximum limit: {}", max)
        }
        /// The number of arguments is less than the minimum limit.
        ArgumentsLess(min: usize) {
            display("The number of arguments is less than the minimum limit: {}", min)
        }
        /// This two value types are different or do not support mathematical calculations.
        UnsupportedTypes(a: String, b: String) {
            display("This two value types are different or do not support mathematical calculations: {}, {}", a, b)
        }
        /// Invalid array expression like `1..2..3`
        InvalidArray(ident: String) {
            display("Invalid array expression: {}", ident)
        }
        /// Custom error.
        Custom(detail: String) {
            display("{}", detail)
        }
    }
}
