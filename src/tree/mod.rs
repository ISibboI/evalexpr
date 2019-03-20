use crate::{configuration::Configuration, error::Error, operator::*, value::Value};
use token::Token;

mod display;

/// A node in the operator tree.
/// The operator tree is created by the crate-level `build_operator_tree` method.
/// It can be evaluated for a given configuration with the `Node::eval` method.
///
/// # Examples
///
/// ```rust
/// use evalexpr::*;
///
/// let node = build_operator_tree("1 + 2").unwrap();
/// assert_eq!(node.eval(&EmptyConfiguration), Ok(Value::from(3)));
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

    /// Evaluates the operator tree rooted at this node.
    ///
    /// Fails, if an operator is used with a wrong number of arguments or a wrong type.
    pub fn eval(&self, configuration: &Configuration) -> Result<Value, Error> {
        let mut arguments = Vec::new();
        for child in self.children() {
            arguments.push(child.eval(configuration)?);
        }
        self.operator().eval(&arguments, configuration)
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

    fn insert_back_prioritized(&mut self, node: Node, is_root_node: bool) -> Result<(), Error> {
        if self.operator().precedence() < node.operator().precedence() || is_root_node
            // Right-to-left chaining
            || (self.operator().precedence() == node.operator().precedence() && !self.operator().is_left_to_right() && !node.operator().is_left_to_right())
        {
            if self.operator().is_leaf() {
                Err(Error::AppendedToLeafNode)
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
                        return Err(Error::AppendedToLeafNode);
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
            Err(Error::PrecedenceViolation)
        }
    }
}

pub(crate) fn tokens_to_operator_tree(tokens: Vec<Token>) -> Result<Node, Error> {
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
                    return Err(Error::UnmatchedRBrace);
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
                return Err(Error::UnmatchedRBrace);
            }
        }

        last_token_is_rightsided_value = token.is_rightsided_value();
    }

    if root.len() > 1 {
        Err(Error::UnmatchedLBrace)
    } else if root.len() == 0 {
        Err(Error::UnmatchedRBrace)
    } else {
        let mut root = root.pop().unwrap();
        if root.children().len() == 1 {
            Ok(root.children.pop().unwrap())
        } else {
            Err(Error::EmptyExpression)
        }
    }
}
