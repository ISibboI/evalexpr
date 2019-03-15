use crate::{configuration::Configuration, error::Error, operator::*, value::Value};
use token::Token;

pub struct Node {
    children: Vec<Node>,
    operator: Box<dyn Operator>,
}

impl Node {
    fn new(operator: Box<dyn Operator>) -> Self {
        Self {
            children: Vec::new(),
            operator,
        }
    }

    fn root_node() -> Self {
        Self {
            children: Vec::new(),
            operator: Box::new(RootNode),
        }
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

    fn insert_back_prioritized(&mut self, operator: Box<dyn Operator>) -> Result<(), Error> {
        if self.operator().precedence() < operator.precedence() {
            if self.operator().is_leaf() {
                Err(Error::AppendedToLeafNode)
            } else if self.has_correct_amount_of_children() {
                if self.children.last_mut().unwrap().operator().precedence() < operator.precedence()
                {
                    self.children
                        .last_mut()
                        .unwrap()
                        .insert_back_prioritized(operator)
                } else {
                    let new_node = Node::new(operator);
                    let last_child = self.children.pop().unwrap();
                    self.children.push(new_node);
                    let new_node = self.children.last_mut().unwrap();

                    if new_node.operator().is_leaf() {
                        Err(Error::AppendedToLeafNode)
                    } else {
                        new_node.children.push(last_child);
                        Ok(())
                    }
                }
            } else {
                self.children.push(Node::new(operator));
                Ok(())
            }
        } else {
            Err(Error::PrecedenceViolation)
        }
    }
}

pub fn tokens_to_operator_tree(tokens: Vec<Token>) -> Result<Node, Error> {
    let mut root = Node::root_node();

    for token in tokens {
        let operator: Option<Box<dyn Operator>> = match token {
            Token::Plus => Some(Box::new(Add)),
            Token::Minus => Some(Box::new(Sub)),
            Token::Star => Some(Box::new(Mul)),
            Token::Slash => Some(Box::new(Div)),
            Token::Whitespace => None,
            Token::Identifier(identifier) => Some(Box::new(Identifier::new(identifier))),
            Token::Float(number) => Some(Box::new(Const::new(Value::Float(number)))),
            Token::Int(number) => Some(Box::new(Const::new(Value::Int(number)))),
            Token::Boolean(boolean) => Some(Box::new(Const::new(Value::Boolean(boolean)))),
        };

        if let Some(operator) = operator {
            root.insert_back_prioritized(operator)?;
        }
    }

    if root.children().len() == 1 {
        Ok(root.children.pop().unwrap())
    } else {
        Err(Error::EmptyExpression)
    }
}
