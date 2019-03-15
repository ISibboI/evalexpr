use crate::{configuration::Configuration, error::Error, operator::*, value::Value};
use token::Token;

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

    pub fn eval(&self, configuration: &Configuration) -> Result<Value, Error> {
        let mut arguments = Vec::new();
        for child in self.children() {
            arguments.push(child.eval(configuration)?);
        }
        self.operator().eval(&arguments, configuration)
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }

    pub fn operator(&self) -> &Box<dyn Operator> {
        &self.operator
    }

    fn has_correct_amount_of_children(&self) -> bool {
        self.children().len() == self.operator().argument_amount()
    }

    fn insert_back_prioritized(&mut self, node: Node, is_root_node: bool) -> Result<(), Error> {
        if self.operator().precedence() < node.operator().precedence() || is_root_node {
            if self.operator().is_leaf() {
                Err(Error::AppendedToLeafNode)
            } else if self.has_correct_amount_of_children() {
                if self.children.last_mut().unwrap().operator().precedence() < node.operator().precedence()
                {
                    self.children
                        .last_mut()
                        .unwrap()
                        .insert_back_prioritized(node, false)
                } else {
                    let last_child = self.children.pop().unwrap();
                    self.children.push(node);
                    let node = self.children.last_mut().unwrap();

                    if node.operator().is_leaf() {
                        Err(Error::AppendedToLeafNode)
                    } else {
                        node.children.push(last_child);
                        Ok(())
                    }
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

pub fn tokens_to_operator_tree(tokens: Vec<Token>) -> Result<Node, Error> {
    let mut root = vec![Node::root_node()];
    let mut last_non_whitespace_token_is_value = false;

    for token in tokens {
        let node = match token.clone() {
            Token::Plus => Some(Node::new(Add)),
            Token::Minus => {
                if last_non_whitespace_token_is_value {
                    Some(Node::new(Sub))
                } else {
                    Some(Node::new(Neg))
                }
            }
            Token::Star => Some(Node::new(Mul)),
            Token::Slash => Some(Node::new(Div)),
            Token::Percent => Some(Node::new(Mod)),

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
            }
            Token::Whitespace => None,
            
            Token::Identifier(identifier) => Some(Node::new(Identifier::new(identifier))),
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

        if token != Token::Whitespace {
            last_non_whitespace_token_is_value = token.is_value();
        }
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
