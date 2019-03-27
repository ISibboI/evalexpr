use EmptyConfiguration;
use FloatType;
use IntType;
use token::Token;
use value::TupleType;

use crate::{configuration::Configuration, error::EvalexprError, operator::*, value::Value};

mod display;

/// A node in the operator tree.
/// The operator tree is created by the crate-level `build_operator_tree` method.
/// It can be evaluated for a given configuration with the `Node::eval` method.
///
/// The advantage of constructing the operator tree separately from the actual evaluation is that it can be evaluated arbitrarily often with different configurations.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut configuration = HashMapConfiguration::new();
/// configuration.insert_variable("alpha", 2);
/// let node = build_operator_tree("1 + alpha").unwrap(); // Do proper error handling here
/// assert_eq!(node.eval_with_configuration(&configuration), Ok(Value::from(3)));
/// ```
///
#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    operator: Box<dyn Operator>,
}

impl Node {
    fn new<T: Operator + 'static>(operator: T) -> Self {
        Self {
            children: Vec::new(),
            operator: Box::new(operator),
        }
    }

    fn root_node() -> Self {
        Self::new(RootNode)
    }

    /// Evaluates the operator tree rooted at this node with the given configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_with_configuration(&self, configuration: &Configuration) -> Result<Value, EvalexprError> {
        let mut arguments = Vec::new();
        for child in self.children() {
            arguments.push(child.eval_with_configuration(configuration)?);
        }
        self.operator().eval(&arguments, configuration)
    }

    /// Evaluates the operator tree rooted at this node with an empty configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval(&self) -> Result<Value, EvalexprError> {
        self.eval_with_configuration(&EmptyConfiguration)
    }

    /// Evaluates the operator tree rooted at this node into a string with an the given configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_string_with_configuration(
        &self,
        configuration: &Configuration,
    ) -> Result<String, EvalexprError> {
        match self.eval_with_configuration(configuration) {
            Ok(Value::String(string)) => Ok(string),
            Ok(value) => Err(EvalexprError::expected_string(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a float with an the given configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_float_with_configuration(
        &self,
        configuration: &Configuration,
    ) -> Result<FloatType, EvalexprError> {
        match self.eval_with_configuration(configuration) {
            Ok(Value::Float(float)) => Ok(float),
            Ok(value) => Err(EvalexprError::expected_float(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into an integer with an the given configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_int_with_configuration(
        &self,
        configuration: &Configuration,
    ) -> Result<IntType, EvalexprError> {
        match self.eval_with_configuration(configuration) {
            Ok(Value::Int(int)) => Ok(int),
            Ok(value) => Err(EvalexprError::expected_int(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a boolean with an the given configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_boolean_with_configuration(
        &self,
        configuration: &Configuration,
    ) -> Result<bool, EvalexprError> {
        match self.eval_with_configuration(configuration) {
            Ok(Value::Boolean(boolean)) => Ok(boolean),
            Ok(value) => Err(EvalexprError::expected_boolean(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a tuple with an the given configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_tuple_with_configuration(
        &self,
        configuration: &Configuration,
    ) -> Result<TupleType, EvalexprError> {
        match self.eval_with_configuration(configuration) {
            Ok(Value::Tuple(tuple)) => Ok(tuple),
            Ok(value) => Err(EvalexprError::expected_tuple(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a string with an empty configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_string(&self) -> Result<String, EvalexprError> {
        self.eval_string_with_configuration(&EmptyConfiguration)
    }

    /// Evaluates the operator tree rooted at this node into a float with an empty configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_float(&self) -> Result<FloatType, EvalexprError> {
        self.eval_float_with_configuration(&EmptyConfiguration)
    }

    /// Evaluates the operator tree rooted at this node into an integer with an empty configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_int(&self) -> Result<IntType, EvalexprError> {
        self.eval_int_with_configuration(&EmptyConfiguration)
    }

    /// Evaluates the operator tree rooted at this node into a boolean with an empty configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_boolean(&self) -> Result<bool, EvalexprError> {
        self.eval_boolean_with_configuration(&EmptyConfiguration)
    }

    /// Evaluates the operator tree rooted at this node into a tuple with an empty configuration.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_tuple(&self) -> Result<TupleType, EvalexprError> {
        self.eval_tuple_with_configuration(&EmptyConfiguration)
    }

    fn children(&self) -> &[Node] {
        &self.children
    }

    fn operator(&self) -> &Box<dyn Operator> {
        &self.operator
    }

    fn has_correct_amount_of_children(&self) -> bool {
        self.children().len() == self.operator().argument_amount()
    }

    fn insert_back_prioritized(&mut self, node: Node, is_root_node: bool) -> Result<(), EvalexprError> {
        if self.operator().precedence() < node.operator().precedence() || is_root_node
            // Right-to-left chaining
            || (self.operator().precedence() == node.operator().precedence() && !self.operator().is_left_to_right() && !node.operator().is_left_to_right())
        {
            if self.operator().is_leaf() {
                Err(EvalexprError::AppendedToLeafNode)
            } else if self.has_correct_amount_of_children() {
                if self.children.last().unwrap().operator().precedence()
                    < node.operator().precedence()
                    // Right-to-left chaining
                    || (self.children.last().unwrap().operator().precedence()
                    == node.operator().precedence() && !self.children.last().unwrap().operator().is_left_to_right() && !node.operator().is_left_to_right())
                {
                    self.children
                        .last_mut()
                        .unwrap()
                        .insert_back_prioritized(node, false)
                } else {
                    if node.operator().is_leaf() {
                        return Err(EvalexprError::AppendedToLeafNode);
                    }

                    let last_child = self.children.pop().unwrap();
                    self.children.push(node);
                    let node = self.children.last_mut().unwrap();

                    node.children.push(last_child);
                    Ok(())
                }
            } else {
                self.children.push(node);
                Ok(())
            }
        } else {
            Err(EvalexprError::PrecedenceViolation)
        }
    }
}

pub(crate) fn tokens_to_operator_tree(tokens: Vec<Token>) -> Result<Node, EvalexprError> {
    let mut root = vec![Node::root_node()];
    let mut last_token_is_rightsided_value = false;
    let mut token_iter = tokens.iter().peekable();

    while let Some(token) = token_iter.next().cloned() {
        let next = token_iter.peek().cloned();

        let node = match token.clone() {
            Token::Plus => Some(Node::new(Add)),
            Token::Minus => {
                if last_token_is_rightsided_value {
                    Some(Node::new(Sub))
                } else {
                    Some(Node::new(Neg))
                }
            },
            Token::Star => Some(Node::new(Mul)),
            Token::Slash => Some(Node::new(Div)),
            Token::Percent => Some(Node::new(Mod)),
            Token::Hat => Some(Node::new(Exp)),

            Token::Eq => Some(Node::new(Eq)),
            Token::Neq => Some(Node::new(Neq)),
            Token::Gt => Some(Node::new(Gt)),
            Token::Lt => Some(Node::new(Lt)),
            Token::Geq => Some(Node::new(Geq)),
            Token::Leq => Some(Node::new(Leq)),
            Token::And => Some(Node::new(And)),
            Token::Or => Some(Node::new(Or)),
            Token::Not => Some(Node::new(Not)),

            Token::LBrace => {
                root.push(Node::root_node());
                None
            },
            Token::RBrace => {
                if root.len() < 2 {
                    return Err(EvalexprError::UnmatchedRBrace);
                } else {
                    root.pop()
                }
            },

            Token::Comma => Some(Node::new(Tuple)),

            Token::Identifier(identifier) => {
                let mut result = Some(Node::new(VariableIdentifier::new(identifier.clone())));
                if let Some(next) = next {
                    if next.is_leftsided_value() {
                        result = Some(Node::new(FunctionIdentifier::new(identifier)));
                    }
                }
                result
            },
            Token::Float(number) => Some(Node::new(Const::new(Value::Float(number)))),
            Token::Int(number) => Some(Node::new(Const::new(Value::Int(number)))),
            Token::Boolean(boolean) => Some(Node::new(Const::new(Value::Boolean(boolean)))),
        };

        if let Some(node) = node {
            if let Some(root) = root.last_mut() {
                root.insert_back_prioritized(node, true)?;
            } else {
                return Err(EvalexprError::UnmatchedRBrace);
            }
        }

        last_token_is_rightsided_value = token.is_rightsided_value();
    }

    if root.len() > 1 {
        Err(EvalexprError::UnmatchedLBrace)
    } else if let Some(mut root) = root.pop() {
        if root.children().len() > 1 {
            Err(EvalexprError::wrong_operator_argument_amount(
                root.children().len(),
                1,
            ))
        } else if let Some(child) = root.children.pop() {
            Ok(child)
        } else {
            Err(EvalexprError::EmptyExpression)
        }
    } else {
        Err(EvalexprError::UnmatchedRBrace)
    }
}
