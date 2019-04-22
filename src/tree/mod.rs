use token::Token;
use value::{TupleType, EMPTY_VALUE};
use EmptyContext;
use EmptyType;
use FloatType;
use IntType;

use crate::{
    context::Context,
    error::{EvalexprError, EvalexprResult},
    operator::*,
    value::Value,
};
use std::mem;

mod display;
mod iter;

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
#[derive(Debug, PartialEq)]
pub struct Node {
    operator: Operator,
    children: Vec<Node>,
}

impl Node {
    fn new(operator: Operator) -> Self {
        Self {
            children: Vec::new(),
            operator,
        }
    }

    fn root_node() -> Self {
        Self::new(Operator::RootNode)
    }

    /// Returns an iterator over all identifiers in this expression.
    /// Each occurrence of an identifier is returned separately.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use evalexpr::*;
    ///
    /// let tree = build_operator_tree("a + b + c * f()").unwrap(); // Do proper error handling here
    /// let mut iter = tree.iter_identifiers();
    /// assert_eq!(iter.next(), Some("a"));
    /// assert_eq!(iter.next(), Some("b"));
    /// assert_eq!(iter.next(), Some("c"));
    /// assert_eq!(iter.next(), Some("f"));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_identifiers(&self) -> impl Iterator<Item = &str> {
        self.iter().filter_map(|node| match node.operator() {
            Operator::VariableIdentifier { identifier }
            | Operator::FunctionIdentifier { identifier } => Some(identifier.as_str()),
            _ => None,
        })
    }

    /// Returns an iterator over all variable identifiers in this expression.
    /// Each occurrence of a variable identifier is returned separately.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use evalexpr::*;
    ///
    /// let tree = build_operator_tree("a + f(b + c)").unwrap(); // Do proper error handling here
    /// let mut iter = tree.iter_variable_identifiers();
    /// assert_eq!(iter.next(), Some("a"));
    /// assert_eq!(iter.next(), Some("b"));
    /// assert_eq!(iter.next(), Some("c"));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_variable_identifiers(&self) -> impl Iterator<Item = &str> {
        self.iter().filter_map(|node| match node.operator() {
            Operator::VariableIdentifier { identifier } => Some(identifier.as_str()),
            _ => None,
        })
    }

    /// Returns an iterator over all function identifiers in this expression.
    /// Each occurrence of a function identifier is returned separately.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use evalexpr::*;
    ///
    /// let tree = build_operator_tree("a + f(b + c)").unwrap(); // Do proper error handling here
    /// let mut iter = tree.iter_function_identifiers();
    /// assert_eq!(iter.next(), Some("f"));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_function_identifiers(&self) -> impl Iterator<Item = &str> {
        self.iter().filter_map(|node| match node.operator() {
            Operator::FunctionIdentifier { identifier } => Some(identifier.as_str()),
            _ => None,
        })
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
        self.operator().eval_mut(&arguments, context)
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

    fn operator(&self) -> &Operator {
        &self.operator
    }

    fn has_enough_children(&self) -> bool {
        Some(self.children().len()) == self.operator().max_argument_amount()
    }

    fn insert_back_prioritized(&mut self, node: Node, is_root_node: bool) -> EvalexprResult<()> {
        println!("Inserting {:?} into {:?}", node.operator, self.operator());
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
                    println!("Recursing into {:?}", self.children.last().unwrap().operator());
                    self.children
                        .last_mut()
                        .unwrap()
                        .insert_back_prioritized(node, false)
                } else {
                    println!("Rotating");
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
                println!("Inserting as specified");
                self.children.push(node);
                Ok(())
            }
        } else {
            Err(EvalexprError::PrecedenceViolation)
        }
    }
}

fn collapse_root_stack_to(root_stack: &mut Vec<Node>, mut root: Node, collapse_goal: &Node) -> EvalexprResult<Node> {
    loop {
        if let Some(mut potential_higher_root) = root_stack.pop() {
            // TODO I'm not sure about this >, as I have no example for different sequence operators with the same precedence
            if potential_higher_root.operator().precedence() > collapse_goal.operator().precedence() {
                potential_higher_root.children.push(root);
                root = potential_higher_root;
            } else {
                root_stack.push(potential_higher_root);
                break;
            }
        } else {
            // This is the only way the topmost root node could have been removed
            return Err(EvalexprError::UnmatchedRBrace);
        }
    }

    Ok(root)
}

fn collapse_all_sequences(root_stack: &mut Vec<Node>) -> EvalexprResult<()> {
    println!("Collapsing all sequences");
    println!("Initial root stack is: {:?}", root_stack);
    let mut root = if let Some(root) = root_stack.pop() {
        root
    } else {
        return Err(EvalexprError::UnmatchedRBrace);
    };

    loop {
        println!("Root is: {:?}", root);
        if root.operator() == &Operator::RootNode {
            root_stack.push(root);
            break;
        }

        if let Some(mut potential_higher_root) = root_stack.pop() {
            if root.operator().is_sequence() {
                potential_higher_root.children.push(root);
                root = potential_higher_root;
            } else {
                root_stack.push(potential_higher_root);
                root_stack.push(root);
                break;
            }
        } else {
            // This is the only way the topmost root node could have been removed
            return Err(EvalexprError::UnmatchedRBrace);
        }
    }

    println!("Root stack after collapsing all sequences is: {:?}", root_stack);
    Ok(())
}

pub(crate) fn tokens_to_operator_tree(tokens: Vec<Token>) -> EvalexprResult<Node> {
    let mut root_stack = vec![Node::root_node()];
    let mut last_token_is_rightsided_value = false;
    let mut token_iter = tokens.iter().peekable();

    while let Some(token) = token_iter.next().cloned() {
        let next = token_iter.peek().cloned();

        let node = match token.clone() {
            Token::Plus => Some(Node::new(Operator::Add)),
            Token::Minus => {
                if last_token_is_rightsided_value {
                    Some(Node::new(Operator::Sub))
                } else {
                    Some(Node::new(Operator::Neg))
                }
            }
            Token::Star => Some(Node::new(Operator::Mul)),
            Token::Slash => Some(Node::new(Operator::Div)),
            Token::Percent => Some(Node::new(Operator::Mod)),
            Token::Hat => Some(Node::new(Operator::Exp)),

            Token::Eq => Some(Node::new(Operator::Eq)),
            Token::Neq => Some(Node::new(Operator::Neq)),
            Token::Gt => Some(Node::new(Operator::Gt)),
            Token::Lt => Some(Node::new(Operator::Lt)),
            Token::Geq => Some(Node::new(Operator::Geq)),
            Token::Leq => Some(Node::new(Operator::Leq)),
            Token::And => Some(Node::new(Operator::And)),
            Token::Or => Some(Node::new(Operator::Or)),
            Token::Not => Some(Node::new(Operator::Not)),

            Token::LBrace => {
                root_stack.push(Node::root_node());
                None
            }
            Token::RBrace => {
                if root_stack.len() <= 1 {
                    return Err(EvalexprError::UnmatchedRBrace);
                } else {
                    collapse_all_sequences(&mut root_stack)?;
                    root_stack.pop()
                }
            }

            Token::Comma => Some(Node::new(Operator::Tuple)),
            Token::Assign => Some(Node::new(Operator::Assign)),
            Token::Semicolon => Some(Node::new(Operator::Chain)),

            Token::Identifier(identifier) => {
                let mut result = Some(Node::new(Operator::variable_identifier(identifier.clone())));
                if let Some(next) = next {
                    if next == &Token::Assign {
                        result = Some(Node::new(Operator::value(identifier.clone().into())));
                    } else if next.is_leftsided_value() {
                        result = Some(Node::new(Operator::function_identifier(identifier)));
                    }
                }
                result
            }
            Token::Float(float) => Some(Node::new(Operator::value(Value::Float(float)))),
            Token::Int(int) => Some(Node::new(Operator::value(Value::Int(int)))),
            Token::Boolean(boolean) => Some(Node::new(Operator::value(Value::Boolean(boolean)))),
            Token::String(string) => Some(Node::new(Operator::value(Value::String(string)))),
        };

        if let Some(mut node) = node {
            // Need to pop and then repush here, because Rust 1.33.0 cannot release the mutable borrow of root_stack before the end of this complete if-statement
            if let Some(mut root) = root_stack.pop() {
                if node.operator().is_sequence() {
                    println!("Found a sequence operator");
                    println!("Stack before sequence operation: {:?}, {:?}", root_stack, root);
                    // If root.operator() and node.operator() are of the same variant, ...
                    if mem::discriminant(root.operator()) == mem::discriminant(node.operator()) {
                        // ... we create a new root node for the next expression in the sequence
                        root.children.push(Node::root_node());
                        root_stack.push(root);
                    } else if root.operator() == &Operator::RootNode {
                        // If the current root is an actual root node, we start a new sequence
                        node.children.push(root);
                        node.children.push(Node::root_node());
                        root_stack.push(Node::root_node());
                        root_stack.push(node);
                    } else {
                        // Otherwise, we combine the sequences based on their precedences
                        // TODO I'm not sure about this <, as I have no example for different sequence operators with the same precedence
                        if root.operator().precedence() < node.operator().precedence() {
                            // If the new sequence has a higher precedence, it is part of the last element of the current root sequence
                            if let Some(last_root_child) = root.children.pop() {
                                node.children.push(last_root_child);
                                node.children.push(Node::root_node());
                                root_stack.push(root);
                                root_stack.push(node);
                            } else {
                                // Once a sequence has been pushed on top of the stack, it also gets a child
                                unreachable!()
                            }
                        } else {
                            // If the new sequence doesn't have a higher precedence, then all sequences with a higher precedence are collapsed below this one
                            root = collapse_root_stack_to(&mut root_stack, root, &node)?;
                            node.children.push(root);
                            root_stack.push(node);
                        }
                    }
                    println!("Stack after sequence operation: {:?}", root_stack);
                } else if root.operator().is_sequence() {
                    if let Some(mut last_root_child) = root.children.pop() {
                        last_root_child.insert_back_prioritized(node, true)?;
                        root.children.push(last_root_child);
                        root_stack.push(root);
                    } else {
                        // Once a sequence has been pushed on top of the stack, it also gets a child
                        unreachable!()
                    }
                } else {
                    root.insert_back_prioritized(node, true)?;
                    root_stack.push(root);
                }
            } else {
                return Err(EvalexprError::UnmatchedRBrace);
            }
        }

        last_token_is_rightsided_value = token.is_rightsided_value();
    }

    // In the end, all sequences are implicitly terminated
    collapse_all_sequences(&mut root_stack)?;

    if root_stack.len() > 1 {
        Err(EvalexprError::UnmatchedLBrace)
    } else if let Some(root) = root_stack.pop() {
        Ok(root)
    } else {
        Err(EvalexprError::UnmatchedRBrace)
    }
}
