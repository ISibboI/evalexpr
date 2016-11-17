
use std::str::FromStr;
use std::clone::Clone;
use serde_json::{Value, to_value};
use math::Math;
use operator::Operator;
use node::Node;
use {Context, Functions};
use error::Error;
use ContextsRef;


#[derive(Default)]
pub struct Expression {
    pub raw: String,
    pub pos: Vec<usize>,
    pub operators: Vec<Operator>,
    pub node: Option<Node>
}

impl Expression {
    pub fn new<T: Into<String>>(raw: T) -> Result<Expression, Error> {
        let mut expr = Expression {
            raw: raw.into(),
            ..Default::default()
        };

        expr.parse_pos()?;
        expr.parse_operators()?;
        expr.parse_node()?;
        Ok(expr)
    }

    pub fn parse_pos(&mut self) -> Result<(), Error> {
        let mut found_quote = false;

        for (index, cur) in self.raw.chars().enumerate() {
            match cur {
                '(' | ')' | '+' | '-' | '*' | '/' | ',' | ' ' | '!' | '=' |
                '>' | '<' | '\'' | '[' | ']' | '%' | '&' | '|' => {
                    if ! found_quote {
                        self.pos.push(index);
                        self.pos.push(index + 1);
                    }
                },
                '"' => {
                    found_quote = ! found_quote;
                    self.pos.push(index);
                    self.pos.push(index + 1);
                },
                _  => ()
            }
        }

        self.pos.push(self.raw.len());
        Ok(())
    }

    pub fn parse_operators(&mut self) -> Result<(), Error> {
        let mut operators = Vec::new();
        let mut start;
        let mut end = 0;
        let mut parenthesis = 0;
        let mut square_brackets = 0;
        let mut quote = None;
        let mut prev = String::new();

        for pos in self.pos.clone() {
            if pos == 0 {
                continue;
            } else {
                start = end;
                end = pos;
            }

            let raw = self.raw[start..end].to_owned();

            let operator = Operator::from_str(&raw).unwrap();
            match operator {
                Operator::DoubleQuotes | Operator::SingleQuote => {
                    if quote.is_some() {
                        if quote.as_ref() == Some(&operator) {
                            operators.push(Operator::Value(to_value(&prev)));
                            prev.clear();
                            quote = None;
                            continue;
                        }
                    } else {
                        quote = Some(operator);
                        prev.clear();
                        continue;
                    }
                },
                _ => ()
            };

            if quote.is_some() {
                prev += &raw;
                continue;
            }

            if raw.is_empty() {
                continue;
            }

            if raw == "=" {
                if prev == "!" || prev == ">" || prev == "<" || prev == "=" {
                    prev.push_str("=");
                    operators.push(Operator::from_str(&prev).unwrap());
                    prev.clear();
                } else {
                    prev = raw;
                }
                continue;
            } else if raw == "!" || raw == ">" || raw == "<" {
                if prev == "!" || prev == ">" || prev == "<" {
                    operators.push(Operator::from_str(&prev).unwrap());
                    prev.clear();
                } else {
                    prev = raw;
                }
                continue;
            } else {
                if prev == "!" || prev == ">" || prev == "<" {
                    operators.push(Operator::from_str(&prev).unwrap());
                    prev.clear();
                }
            }

            if (raw == "&" || raw == "|") && (prev == "&" || prev == "|") {
                if raw == prev {
                    prev.push_str(&raw);
                    operators.push(Operator::from_str(&prev).unwrap());
                    prev.clear();
                    continue;
                } else {
                    return Err(Error::UnsupportedOperator(prev));
                }
            } else if raw == "&" || raw == "|" {
                prev = raw;
                continue;
            }

            match operator {
                Operator::LeftSquareBracket => {
                    square_brackets += 1;
                    operators.push(Operator::Function("array".to_owned()));
                    operators.push(operator);
                    continue;
                },
                Operator::LeftParenthesis => {
                    parenthesis += 1;

                    if ! operators.is_empty() {
                        let prev_operator = operators.pop().unwrap();
                        if prev_operator.is_identifier() {
                            operators.push(Operator::Function(prev_operator.get_identifier()));
                            operators.push(operator);
                            continue;
                        } else {
                            operators.push(prev_operator);
                        }
                    }
                },
                Operator::RightParenthesis => parenthesis -= 1,
                Operator::RightSquareBracket => square_brackets -= 1,
                Operator::WhiteSpace => continue,
                _ => ()
            }

            prev = raw;
            operators.push(operator);
        }

        if parenthesis != 0 || square_brackets != 0 {
            Err(Error::UnpairedBrackets)
        } else {
            self.operators = operators;
            Ok(())
        }
    }

    pub fn parse_node(&mut self) -> Result<(), Error> {
        let mut parsing_nodes = Vec::<Node>::new();

        for operator in &self.operators {
            match *operator {
                Operator::Add(priority) | Operator::Sub(priority) |
                Operator::Mul(priority) | Operator::Div(priority) |
                Operator::Not(priority) | Operator::Eq(priority) |
                Operator::Ne(priority) | Operator::Gt(priority) |
                Operator::Lt(priority) | Operator::Ge(priority) |
                Operator::And(priority) | Operator::Or(priority) |
                Operator::Le(priority) | Operator::Rem(priority) => {
                    if ! parsing_nodes.is_empty() {
                        let prev = parsing_nodes.pop().unwrap();
                        if prev.is_value_or_enough() {
                            if prev.operator.get_priority() < priority && ! prev.closed {
                                parsing_nodes.extend_from_slice(&rob_to(prev, operator.to_node()));
                            } else {
                                parsing_nodes.push(operator.children_to_node(vec![prev]));
                            }
                        } else if prev.operator.can_at_beginning() {
                            parsing_nodes.push(prev);
                            parsing_nodes.push(operator.to_node());
                        } else {
                            return Err(Error::DuplicateOperatorNode);
                        }
                    } else if operator.can_at_beginning() {
                        parsing_nodes.push(operator.to_node());
                    } else {
                        return Err(Error::StartWithNonValueOperator);
                    }
                },
                Operator::Function(_) | Operator::LeftParenthesis | Operator::LeftSquareBracket => parsing_nodes.push(operator.to_node()),
                Operator::Comma => close_comma(&mut parsing_nodes)?,
                Operator::RightParenthesis | Operator::RightSquareBracket => close_bracket(&mut parsing_nodes, operator.get_left())?,
                Operator::Value(_) | Operator::Identifier(_) => append_child_to_last_node(&mut parsing_nodes, operator)?,
                _ => ()
            }
        }

        self.node = Some(get_final_node(parsing_nodes)?);
        Ok(())
    }

    pub fn compile(&self) -> Box<Fn(ContextsRef, &Functions, &Functions) -> Result<Value, Error>> {
        let node = self.node.clone().unwrap();

        Box::new(move |contexts, buildin, functions| -> Result<Value, Error> {
            return exec_node(&node, contexts, buildin, functions);

            fn exec_node(node: &Node, contexts: ContextsRef, buildin: &Functions, functions: &Functions) -> Result<Value, Error> {
                match node.operator {
                    Operator::Add(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.add(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Mul(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.mul(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Sub(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.sub(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Div(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.div(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Rem(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.rem(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Eq(_) => Math::eq(&exec_node(&node.get_first_child(), contexts, buildin, functions)?, &exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Ne(_) => Math::ne(&exec_node(&node.get_first_child(), contexts, buildin, functions)?, &exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Gt(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.gt(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Lt(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.lt(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Ge(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.ge(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Le(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.le(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::And(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.and(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Or(_) => exec_node(&node.get_first_child(), contexts, buildin, functions)?.or(&exec_node(&node.get_last_child(), contexts, buildin, functions)?),
                    Operator::Function(ref ident) => {
                        let function_option = if functions.contains_key(ident) {
                            functions.get(ident)
                        } else {
                            buildin.get(ident)
                        };

                        if function_option.is_some() {
                            let function = function_option.unwrap();
                            node.check_function_args(function)?;
                            let mut values = Vec::new();
                            for node in &node.children {
                                values.push(exec_node(node, contexts, buildin, functions)?);
                            }
                            (function.compiled)(values)
                        } else {
                            Err(Error::FunctionNotExists(ident.to_owned()))
                        }
                    },
                    Operator::Value(ref value) => Ok(value.clone()),
                    Operator::Not(_) => {
                        let value = exec_node(&node.get_first_child(), contexts, buildin, functions)?;
                        match value {
                            Value::Bool(boolean) => Ok(Value::Bool(!boolean)),
                            Value::Null => Ok(Value::Bool(true)),
                            _ => Err(Error::NotBoolean(value))
                        }
                    },
                    Operator::Identifier(ref ident) => {
                        let number = parse_number(ident);
                        if number.is_some() {
                            Ok(number.unwrap())
                        } else if is_range(ident) {
                            parse_range(ident)
                        } else {
                            match find(contexts, ident) {
                                Some(value) => Ok(value),
                                None => Ok(Value::Null)
                            }
                        }
                    },
                    _ => Err(Error::CanNotExec(node.operator.clone()))
                }
            }
        })
    }
}

fn append_child_to_last_node(parsing_nodes: &mut Vec<Node>, operator: &Operator) -> Result<(), Error> {
    let mut node = operator.to_node();
    node.closed = true;

    if let Some(mut prev) = parsing_nodes.pop() {
        if prev.is_value_or_enough() {
            return Err(Error::DuplicateValueNode);
        } else if prev.is_enough() {
            parsing_nodes.push(prev);
            parsing_nodes.push(node);
        } else {
            prev.add_child(node);
            parsing_nodes.push(prev);
        }
    } else {
        parsing_nodes.push(node);
    }

    Ok(())
}

fn get_final_node(mut parsing_nodes: Vec<Node>) -> Result<Node, Error> {
    if parsing_nodes.is_empty() {
        return Err(Error::NoFinalNode)
    }

    while parsing_nodes.len() != 1 {
        let last = parsing_nodes.pop().unwrap();
        let mut prev = parsing_nodes.pop().unwrap();
        prev.add_child(last);
        parsing_nodes.push(prev);
    }

    Ok(parsing_nodes.pop().unwrap())
}

fn close_bracket(parsing_nodes: &mut Vec<Node>, bracket: Operator) -> Result<(), Error> {
    loop {
        let mut current = parsing_nodes.pop().unwrap();
        let mut prev = parsing_nodes.pop().unwrap();

        if current.operator == bracket {
            if prev.is_unclosed_function() {
                prev.closed = true;
                parsing_nodes.push(prev);
                break;
            } else {
                return Err(Error::BracketNotWithFunction);
            }
        } else if prev.operator == bracket {
            if ! current.closed {
                current.closed = true;
            }

            if let Some(mut penult) = parsing_nodes.pop() {
                if penult.is_unclosed_function() {
                    penult.closed = true;
                    penult.add_child(current);
                    parsing_nodes.push(penult);
                } else {
                    parsing_nodes.push(penult);
                    parsing_nodes.push(current);
                }
            } else {
                parsing_nodes.push(current);
            }
            break;
        } else {
            if ! prev.closed {
                prev.add_child(current);
                if prev.is_enough() {
                    prev.closed = true;
                }

                if ! parsing_nodes.is_empty() {
                    parsing_nodes.push(prev);
                } else {
                    return Err(Error::StartWithNonValueOperator);
                }
            } else {
                return Err(Error::StartWithNonValueOperator);
            }
        }
    }
    Ok(())
}

fn close_comma(parsing_nodes: &mut Vec<Node>) -> Result<(), Error> {
    if parsing_nodes.len() < 2 {
        return Err(Error::CommaNotWithFunction);
    }

    loop {
        let current = parsing_nodes.pop().unwrap();
        let mut prev = parsing_nodes.pop().unwrap();

        if current.operator == Operator::Comma {
            parsing_nodes.push(prev);
            break;
        } else if current.operator.is_left() {
            parsing_nodes.push(prev);
            parsing_nodes.push(current);
            break;
        } else if prev.operator.is_left() {
            if let Some(mut penult) = parsing_nodes.pop() {
                if penult.is_unclosed_function() {
                    penult.add_child(current);
                    parsing_nodes.push(penult);
                    parsing_nodes.push(prev);
                    break;
                } else {
                    return Err(Error::CommaNotWithFunction);
                }
            } else {
                return Err(Error::CommaNotWithFunction);
            }
        } else {
            if ! prev.closed {
                prev.add_child(current);
                if prev.is_enough() {
                    prev.closed = true;
                }

                if ! parsing_nodes.is_empty() {
                    parsing_nodes.push(prev);
                } else {
                    return Err(Error::StartWithNonValueOperator);
                }
            } else {
                return Err(Error::StartWithNonValueOperator);
            }
        }
    }
    Ok(())
}

fn rob_to(mut was_robed: Node, mut rober: Node) -> Vec<Node> {
    let moveout_node = was_robed.moveout_last_node();
    rober.add_child(moveout_node);
    vec![was_robed, rober]
}

fn find(contexts: ContextsRef, key: &str) -> Option<Value> {
    for context in contexts.iter().rev() {
        let value = get(context, key);
        match value {
            Some(_) => return value,
            None => continue
        }
    }

    None
}

fn get(context: &Context, key: &str) -> Option<Value> {
    let mut keys = key.split('.').collect::<Vec<_>>();
    let context_key = keys.remove(0);
    let context_value_option = context.get(context_key);

    if context_value_option.is_none() {
        None
    } else if ! keys.is_empty() {
        match context_value_option.unwrap().search(&keys.join(".")) {
            Some(value) => Some(value.clone()),
            None => None
        }
    } else {
        Some(context_value_option.unwrap().clone())
    }
}

fn is_range(ident: &str) -> bool {
    ident.contains("..")
}

fn parse_range(ident: &str) -> Result<Value, Error> {
    let segments = ident.split("..").collect::<Vec<_>>();
    if segments.len() != 2 {
        Err(Error::InvalidRange(ident.to_owned()))
    } else {
        let start = segments[0].parse::<i64>();
        let end = segments[1].parse::<i64>();

        if start.is_ok() && end.is_ok() {
            let mut array = Vec::new();
            for n in start.unwrap()..end.unwrap() {
                array.push(n);
            }
            Ok(to_value(array))
        } else {
            Err(Error::InvalidRange(ident.to_owned()))
        }
    }
}

fn parse_number(ident: &str) -> Option<Value> {
    let number = ident.parse::<u64>();
    if number.is_ok() {
        return Some(to_value(number.unwrap()));
    }

    let number = ident.parse::<i64>();
    if number.is_ok() {
        return Some(to_value(number.unwrap()));
    }

    let number = ident.parse::<f64>();
    if number.is_ok() {
        return Some(to_value(number.unwrap()));
    }

    None
}
