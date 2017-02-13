
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
        /// Expected a boolean but the given value isn't.
        ExpectedBoolean(value: Value) {
            display("Expected a boolean, found: {}", value)
        }
        /// Expected ident.
        ExpectedIdentifier {
            display("Expected ident.")
        }
        /// Expected array.
        ExpectedArray {
            display("Expected array.")
        }
        /// Expected object.
        ExpectedObject {
            display("Expected object.")
        }
        /// Expect number.
        ExpectedNumber {
            display("Expected number.")
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
        /// Invalid range expression like `1..2..3`
        InvalidRange(ident: String) {
            display("Invalid range expression: {}", ident)
        }
        /// Can not add child node.
        CanNotAddChild {
            display("Can not add child node.")
        }
        /// Custom error.
        Custom(detail: String) {
            display("{}", detail)
        }
    }
}
