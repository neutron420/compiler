use super::parser::AstNode;
use super::lexer::Token;
use super::object::Object;
use std::collections::HashMap;

pub type Environment = HashMap<String, Object>;

pub fn evaluate(node: &AstNode, env: &mut Environment) -> Result<Object, String> {
    match node {
        AstNode::Program(statements) => {
            let mut result = Object::Boolean(false); // Default result
            for stmt in statements {
                result = evaluate(stmt, env)?;
            }
            Ok(result)
        }
        AstNode::BlockStatement(statements) => {
            // Create a new scope for block statements
            let mut block_env = env.clone();
            let mut result = Object::Boolean(false); // Default result
            for stmt in statements {
                result = evaluate(stmt, &mut block_env)?;
            }
            // Don't merge block_env back to maintain proper scoping
            Ok(result)
        }
        AstNode::LetStatement { name, value } => {
            let val = evaluate(value, env)?;
            env.insert(name.clone(), val);
            Ok(Object::Boolean(false)) // Let statements don't produce a value
        }
        AstNode::Identifier(name) => {
            match env.get(name) {
                Some(obj) => Ok(obj.clone()),
                None => Err(format!("Identifier not found: {}", name)),
            }
        }
        AstNode::Number(n) => Ok(Object::Number(*n)),
        AstNode::Boolean(b) => Ok(Object::Boolean(*b)),
        AstNode::PrefixExpression { op, right } => {
            let right_val = evaluate(right, env)?;
            match op {
                Token::Not => Ok(Object::Boolean(!is_truthy(right_val))),
                Token::Minus => match right_val {
                    Object::Number(n) => Ok(Object::Number(-n)),
                    _ => Err("Cannot negate a non-number".to_string()),
                },
                _ => Err("Unknown prefix operator".to_string()),
            }
        }
        AstNode::InfixExpression { op, left, right } => {
            let left_val = evaluate(left, env)?;
            let right_val = evaluate(right, env)?;
            match (&left_val, &right_val) {
                (Object::Number(l), Object::Number(r)) => {
                    evaluate_number_infix_op(op, *l, *r)
                }
                (Object::Boolean(l), Object::Boolean(r)) => {
                    evaluate_boolean_infix_op(op, *l, *r)
                }
                // Handle equality/inequality between different types
                (_, _) if matches!(op, Token::Equal | Token::NotEqual) => {
                    match op {
                        Token::Equal => Ok(Object::Boolean(false)), // Different types are never equal
                        Token::NotEqual => Ok(Object::Boolean(true)), // Different types are always not equal
                        _ => unreachable!(),
                    }
                }
                _ => Err(format!("Type mismatch: cannot apply {:?} to {} and {}", op, 
                    match left_val { Object::Number(_) => "number", Object::Boolean(_) => "boolean" },
                    match right_val { Object::Number(_) => "number", Object::Boolean(_) => "boolean" }
                )),
            }
        }
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Boolean(b) => b,
        Object::Number(n) => n != 0.0,
    }
}

fn evaluate_number_infix_op(op: &Token, l: f64, r: f64) -> Result<Object, String> {
    match op {
        Token::Plus => Ok(Object::Number(l + r)),
        Token::Minus => Ok(Object::Number(l - r)),
        Token::Multiply => Ok(Object::Number(l * r)),
        Token::Divide => if r == 0.0 { Err("Division by zero".to_string()) } else { Ok(Object::Number(l / r)) },
        Token::Modulo => if r == 0.0 { Err("Modulo by zero".to_string()) } else { Ok(Object::Number(l % r)) },
        Token::Equal => Ok(Object::Boolean(l == r)),
        Token::NotEqual => Ok(Object::Boolean(l != r)),
        Token::LessThan => Ok(Object::Boolean(l < r)),
        Token::GreaterThan => Ok(Object::Boolean(l > r)),
        Token::LessThanOrEqual => Ok(Object::Boolean(l <= r)),
        Token::GreaterThanOrEqual => Ok(Object::Boolean(l >= r)),
        _ => Err(format!("Unknown operator for numbers: {:?}", op)),
    }
}

fn evaluate_boolean_infix_op(op: &Token, l: bool, r: bool) -> Result<Object, String> {
    match op {
        Token::Equal => Ok(Object::Boolean(l == r)),
        Token::NotEqual => Ok(Object::Boolean(l != r)),
        Token::And => Ok(Object::Boolean(l && r)),
        Token::Or => Ok(Object::Boolean(l || r)),
        _ => Err(format!("Unknown operator for booleans: {:?}", op)),
    }
}