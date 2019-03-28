use EmptyContext;
use EmptyType;
use FloatType;
use IntType;
use token::Token;
use value::{EMPTY_VALUE, TupleType};

use crate::{
    context::Context,
    error::{EvalexprError, EvalexprResult},
    operator::*,
    value::Value,
};

mod display;

/// A node in the operator tree.
/// The operator tree is created by the crate-level `build_operator_tree` method.
/// It can be evaluated for a given context with the `Node::eval` method.
///
/// The advantage of constructing the operator tree separately from the actual evaluation is that it can be evaluated arbitrarily often with different contexts.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let mut context = HashMapContext::new();
/// context.set_value("alpha".into(), 2.into()).unwrap(); // Do proper error handling here
/// let node = build_operator_tree("1 + alpha").unwrap(); // Do proper error handling here
/// assert_eq!(node.eval_with_context(&context), Ok(Value::from(3)));
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

    /// Evaluates the operator tree rooted at this node with the given context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_with_context(&self, context: &Context) -> EvalexprResult<Value> {
        let mut arguments = Vec::new();
        for child in self.children() {
            arguments.push(child.eval_with_context(context)?);
        }
        self.operator().eval(&arguments, context)
    }

    /// Evaluates the operator tree rooted at this node with the given mutable context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_with_context_mut(&self, context: &mut Context) -> EvalexprResult<Value> {
        let mut arguments = Vec::new();
        for child in self.children() {
            arguments.push(child.eval_with_context_mut(context)?);
        }
        self.operator().eval(&arguments, context)
    }

    /// Evaluates the operator tree rooted at this node with an empty context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval(&self) -> EvalexprResult<Value> {
        self.eval_with_context(&EmptyContext)
    }

    /// Evaluates the operator tree rooted at this node into a string with an the given context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_string_with_context(&self, context: &Context) -> EvalexprResult<String> {
        match self.eval_with_context(context) {
            Ok(Value::String(string)) => Ok(string),
            Ok(value) => Err(EvalexprError::expected_string(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a float with an the given context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_float_with_context(&self, context: &Context) -> EvalexprResult<FloatType> {
        match self.eval_with_context(context) {
            Ok(Value::Float(float)) => Ok(float),
            Ok(value) => Err(EvalexprError::expected_float(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into an integer with an the given context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_int_with_context(&self, context: &Context) -> EvalexprResult<IntType> {
        match self.eval_with_context(context) {
            Ok(Value::Int(int)) => Ok(int),
            Ok(value) => Err(EvalexprError::expected_int(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a float with an the given context.
    /// If the result of the expression is an integer, it is silently converted into a float.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_number_with_context(&self, context: &Context) -> EvalexprResult<FloatType> {
        match self.eval_with_context(context) {
            Ok(Value::Int(int)) => Ok(int as FloatType),
            Ok(Value::Float(float)) => Ok(float),
            Ok(value) => Err(EvalexprError::expected_int(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a boolean with an the given context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_boolean_with_context(&self, context: &Context) -> EvalexprResult<bool> {
        match self.eval_with_context(context) {
            Ok(Value::Boolean(boolean)) => Ok(boolean),
            Ok(value) => Err(EvalexprError::expected_boolean(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a tuple with an the given context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_tuple_with_context(&self, context: &Context) -> EvalexprResult<TupleType> {
        match self.eval_with_context(context) {
            Ok(Value::Tuple(tuple)) => Ok(tuple),
            Ok(value) => Err(EvalexprError::expected_tuple(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into an empty value with an the given context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_empty_with_context(&self, context: &Context) -> EvalexprResult<EmptyType> {
        match self.eval_with_context(context) {
            Ok(Value::Empty) => Ok(EMPTY_VALUE),
            Ok(value) => Err(EvalexprError::expected_empty(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a string with an the given mutable context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_string_with_context_mut(&self, context: &mut Context) -> EvalexprResult<String> {
        match self.eval_with_context_mut(context) {
            Ok(Value::String(string)) => Ok(string),
            Ok(value) => Err(EvalexprError::expected_string(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a float with an the given mutable context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_float_with_context_mut(&self, context: &mut Context) -> EvalexprResult<FloatType> {
        match self.eval_with_context_mut(context) {
            Ok(Value::Float(float)) => Ok(float),
            Ok(value) => Err(EvalexprError::expected_float(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into an integer with an the given mutable context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_int_with_context_mut(&self, context: &mut Context) -> EvalexprResult<IntType> {
        match self.eval_with_context_mut(context) {
            Ok(Value::Int(int)) => Ok(int),
            Ok(value) => Err(EvalexprError::expected_int(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a float with an the given mutable context.
    /// If the result of the expression is an integer, it is silently converted into a float.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_number_with_context_mut(&self, context: &mut Context) -> EvalexprResult<FloatType> {
        match self.eval_with_context_mut(context) {
            Ok(Value::Int(int)) => Ok(int as FloatType),
            Ok(Value::Float(float)) => Ok(float),
            Ok(value) => Err(EvalexprError::expected_int(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a boolean with an the given mutable context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_boolean_with_context_mut(&self, context: &mut Context) -> EvalexprResult<bool> {
        match self.eval_with_context_mut(context) {
            Ok(Value::Boolean(boolean)) => Ok(boolean),
            Ok(value) => Err(EvalexprError::expected_boolean(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a tuple with an the given mutable context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_tuple_with_context_mut(&self, context: &mut Context) -> EvalexprResult<TupleType> {
        match self.eval_with_context_mut(context) {
            Ok(Value::Tuple(tuple)) => Ok(tuple),
            Ok(value) => Err(EvalexprError::expected_tuple(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into an empty value with an the given mutable context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_empty_with_context_mut(&self, context: &mut Context) -> EvalexprResult<EmptyType> {
        match self.eval_with_context_mut(context) {
            Ok(Value::Empty) => Ok(EMPTY_VALUE),
            Ok(value) => Err(EvalexprError::expected_empty(value)),
            Err(error) => Err(error),
        }
    }

    /// Evaluates the operator tree rooted at this node into a string with an empty context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_string(&self) -> EvalexprResult<String> {
        self.eval_string_with_context(&EmptyContext)
    }

    /// Evaluates the operator tree rooted at this node into a float with an empty context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_float(&self) -> EvalexprResult<FloatType> {
        self.eval_float_with_context(&EmptyContext)
    }

    /// Evaluates the operator tree rooted at this node into an integer with an empty context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_int(&self) -> EvalexprResult<IntType> {
        self.eval_int_with_context(&EmptyContext)
    }

    /// Evaluates the operator tree rooted at this node into a float with an empty context.
    /// If the result of the expression is an integer, it is silently converted into a float.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_number(&self) -> EvalexprResult<FloatType> {
        self.eval_number_with_context(&EmptyContext)
    }

    /// Evaluates the operator tree rooted at this node into a boolean with an empty context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_boolean(&self) -> EvalexprResult<bool> {
        self.eval_boolean_with_context(&EmptyContext)
    }

    /// Evaluates the operator tree rooted at this node into a tuple with an empty context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_tuple(&self) -> EvalexprResult<TupleType> {
        self.eval_tuple_with_context(&EmptyContext)
    }

    /// Evaluates the operator tree rooted at this node into an empty value with an empty context.
    ///
    /// Fails, if one of the operators in the expression tree fails.
    pub fn eval_empty(&self) -> EvalexprResult<EmptyType> {
        self.eval_empty_with_context(&EmptyContext)
    }

    fn children(&self) -> &[Node] {
        &self.children
    }

    fn operator(&self) -> &Box<dyn Operator> {
        &self.operator
    }

    fn has_enough_children(&self) -> bool {
        self.children().len() == self.operator().max_argument_amount()
    }

    fn insert_back_prioritized(&mut self, node: Node, is_root_node: bool) -> EvalexprResult<()> {
        if self.operator().precedence() < node.operator().precedence() || is_root_node
            // Right-to-left chaining
            || (self.operator().precedence() == node.operator().precedence() && !self.operator().is_left_to_right() && !node.operator().is_left_to_right())
        {
            if self.operator().is_leaf() {
                Err(EvalexprError::AppendedToLeafNode)
            } else if self.has_enough_children() {
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

pub(crate) fn tokens_to_operator_tree(tokens: Vec<Token>) -> EvalexprResult<Node> {
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
    } else if let Some(root) = root.pop() {
        Ok(root)
    } else {
        Err(EvalexprError::UnmatchedRBrace)
    }
}
