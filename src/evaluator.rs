use super::parser::AstNode;
use super::lexer::Token;
use super::object::{Object, get_builtins};
use std::collections::HashMap;

pub type Environment = HashMap<String, Object>;

#[derive(Debug)]
pub enum EvalResult {
    Value(Object),
    Return(Object),
    Break,
    Continue,
}

impl EvalResult {
    fn unwrap_value(self) -> Object {
        match self {
            EvalResult::Value(obj) => obj,
            EvalResult::Return(obj) => obj,
            _ => Object::Null,
        }
    }
}

pub fn evaluate(node: &AstNode, env: &mut Environment) -> Result<Object, String> {
    // Add builtins to environment if not present
    for (name, builtin) in get_builtins() {
        env.entry(name).or_insert(builtin);
    }
    
    match evaluate_internal(node, env)? {
        EvalResult::Value(obj) => Ok(obj),
        EvalResult::Return(obj) => Ok(obj),
        EvalResult::Break => Err("break statement outside of loop".to_string()),
        EvalResult::Continue => Err("continue statement outside of loop".to_string()),
    }
}

fn evaluate_internal(node: &AstNode, env: &mut Environment) -> Result<EvalResult, String> {
    match node {
        AstNode::Program(statements) => {
            let mut result = Object::Null;
            for stmt in statements {
                match evaluate_internal(stmt, env)? {
                    EvalResult::Value(obj) => result = obj,
                    EvalResult::Return(obj) => return Ok(EvalResult::Return(obj)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                }
            }
            Ok(EvalResult::Value(result))
        }
        
        AstNode::BlockStatement(statements) => {
            // Create new scope for block
            let mut block_env = env.clone();
            let mut result = Object::Null;
            
            for stmt in statements {
                match evaluate_internal(stmt, &mut block_env)? {
                    EvalResult::Value(obj) => result = obj,
                    EvalResult::Return(obj) => return Ok(EvalResult::Return(obj)),
                    EvalResult::Break => return Ok(EvalResult::Break),
                    EvalResult::Continue => return Ok(EvalResult::Continue),
                }
            }
            
            // Copy back any function definitions to parent scope
            for (key, value) in block_env.iter() {
                if matches!(value, Object::Function { .. }) && !env.contains_key(key) {
                    env.insert(key.clone(), value.clone());
                }
            }
            
            Ok(EvalResult::Value(result))
        }
        
        AstNode::LetStatement { name, value } => {
            let val = evaluate_internal(value, env)?.unwrap_value();
            env.insert(name.clone(), val);
            Ok(EvalResult::Value(Object::Null))
        }
        
        // NEW: If statement evaluation
        AstNode::IfStatement { condition, then_branch, else_branch } => {
            let condition_val = evaluate_internal(condition, env)?.unwrap_value();
            
            if condition_val.is_truthy() {
                evaluate_internal(then_branch, env)
            } else if let Some(else_stmt) = else_branch {
                evaluate_internal(else_stmt, env)
            } else {
                Ok(EvalResult::Value(Object::Null))
            }
        }
        
        // NEW: While loop evaluation
        AstNode::WhileStatement { condition, body } => {
            let mut result = Object::Null;
            
            loop {
                let condition_val = evaluate_internal(condition, env)?.unwrap_value();
                if !condition_val.is_truthy() {
                    break;
                }
                
                match evaluate_internal(body, env)? {
                    EvalResult::Value(obj) => result = obj,
                    EvalResult::Return(obj) => return Ok(EvalResult::Return(obj)),
                    EvalResult::Break => break,
                    EvalResult::Continue => continue,
                }
            }
            
            Ok(EvalResult::Value(result))
        }
        
        // NEW: For loop evaluation
        AstNode::ForStatement { init, condition, increment, body } => {
            // Create new scope for for loop
            let mut loop_env = env.clone();
            
            // Initialize
            evaluate_internal(init, &mut loop_env)?;
            
            let mut result = Object::Null;
            
            loop {
                let condition_val = evaluate_internal(condition, &mut loop_env)?.unwrap_value();
                if !condition_val.is_truthy() {
                    break;
                }
                
                // Execute body
                match evaluate_internal(body, &mut loop_env)? {
                    EvalResult::Value(obj) => result = obj,
                    EvalResult::Return(obj) => return Ok(EvalResult::Return(obj)),
                    EvalResult::Break => break,
                    EvalResult::Continue => {
                        // Execute increment and continue
                        evaluate_internal(increment, &mut loop_env)?;
                        continue;
                    },
                }
                
                // Execute increment
                evaluate_internal(increment, &mut loop_env)?;
            }
            
            Ok(EvalResult::Value(result))
        }
        
        // NEW: Function definition
        AstNode::FunctionDefinition { name, parameters, body } => {
            let function = Object::Function {
                parameters: parameters.clone(),
                body: (**body).clone(),
                closure: env.clone(),
            };
            env.insert(name.clone(), function.clone());
            Ok(EvalResult::Value(function))
        }
        
        // NEW: Function call
        AstNode::FunctionCall { name, arguments } => {
            let function = match env.get(name) {
                Some(obj) => obj.clone(),
                None => return Err(format!("Function not found: {}", name)),
            };
            
            let args: Result<Vec<Object>, String> = arguments.iter()
                .map(|arg| evaluate_internal(arg, env).map(|r| r.unwrap_value()))
                .collect();
            let args = args?;
            
            match function {
                Object::Function { parameters, body, mut closure } => {
                    if parameters.len() != args.len() {
                        return Err(format!("Function {} expects {} arguments, got {}", 
                            name, parameters.len(), args.len()));
                    }
                    
                    // Bind arguments to parameters
                    for (param, arg) in parameters.iter().zip(args.iter()) {
                        closure.insert(param.clone(), arg.clone());
                    }
                    
                    match evaluate_internal(&body, &mut closure)? {
                        EvalResult::Return(obj) => Ok(EvalResult::Value(obj)),
                        EvalResult::Value(obj) => Ok(EvalResult::Value(obj)),
                        other => Ok(other),
                    }
                }
                Object::BuiltinFunction(func) => {
                    let result = func(&args)?;
                    Ok(EvalResult::Value(result))
                }
                _ => Err(format!("{} is not a function", name)),
            }
        }
        
        // NEW: Return statement
        AstNode::ReturnStatement { value } => {
            let return_value = match value {
                Some(expr) => evaluate_internal(expr, env)?.unwrap_value(),
                None => Object::Null,
            };
            Ok(EvalResult::Return(return_value))
        }
        
        // NEW: Break and Continue
        AstNode::BreakStatement => Ok(EvalResult::Break),
        AstNode::ContinueStatement => Ok(EvalResult::Continue),
        
        // NEW: Array literal
        AstNode::Array(elements) => {
            let values: Result<Vec<Object>, String> = elements.iter()
                .map(|elem| evaluate_internal(elem, env).map(|r| r.unwrap_value()))
                .collect();
            Ok(EvalResult::Value(Object::Array(values?)))
        }
        
        // NEW: Array access
        AstNode::ArrayAccess { array, index } => {
            let array_obj = evaluate_internal(array, env)?.unwrap_value();
            let index_obj = evaluate_internal(index, env)?.unwrap_value();
            
            match (&array_obj, &index_obj) {
                (Object::Array(arr), Object::Number(i)) => {
                    let idx = *i as usize;
                    if idx >= arr.len() {
                        return Err("Array index out of bounds".to_string());
                    }
                    Ok(EvalResult::Value(arr[idx].clone()))
                }
                (Object::String(s), Object::Number(i)) => {
                    let idx = *i as usize;
                    if idx >= s.len() {
                        return Err("String index out of bounds".to_string());
                    }
                    let char = s.chars().nth(idx).unwrap();
                    Ok(EvalResult::Value(Object::String(char.to_string())))
                }
                _ => Err("Invalid array/string access".to_string()),
            }
        }
        
        AstNode::Identifier(name) => {
            match env.get(name) {
                Some(obj) => Ok(EvalResult::Value(obj.clone())),
                None => Err(format!("Identifier not found: {}", name)),
            }
        }
        
        AstNode::Number(n) => Ok(EvalResult::Value(Object::Number(*n))),
        AstNode::Boolean(b) => Ok(EvalResult::Value(Object::Boolean(*b))),
        AstNode::String(s) => Ok(EvalResult::Value(Object::String(s.clone()))), // NEW
        
        AstNode::PrefixExpression { op, right } => {
            let right_val = evaluate_internal(right, env)?.unwrap_value();
            match op {
                Token::Not => Ok(EvalResult::Value(Object::Boolean(!right_val.is_truthy()))),
                Token::Minus => match right_val {
                    Object::Number(n) => Ok(EvalResult::Value(Object::Number(-n))),
                    _ => Err("Cannot negate a non-number".to_string()),
                },
                _ => Err("Unknown prefix operator".to_string()),
            }
        }
        
        AstNode::InfixExpression { op, left, right } => {
            let left_val = evaluate_internal(left, env)?.unwrap_value();
            let right_val = evaluate_internal(right, env)?.unwrap_value();
            
            match (&left_val, &right_val) {
                (Object::Number(l), Object::Number(r)) => {
                    evaluate_number_infix_op(op, *l, *r)
                }
                (Object::Boolean(l), Object::Boolean(r)) => {
                    evaluate_boolean_infix_op(op, *l, *r)
                }
                (Object::String(l), Object::String(r)) => {
                    evaluate_string_infix_op(op, l, r)
                }
                // Mixed type comparisons
                (_, _) if matches!(op, Token::Equal | Token::NotEqual) => {
                    match op {
                        Token::Equal => Ok(EvalResult::Value(Object::Boolean(objects_equal(&left_val, &right_val)))),
                        Token::NotEqual => Ok(EvalResult::Value(Object::Boolean(!objects_equal(&left_val, &right_val)))),
                        _ => unreachable!(),
                    }
                }
                _ => Err(format!("Type mismatch: cannot apply {:?} to {} and {}", 
                    op, left_val.type_name(), right_val.type_name())),
            }
        }
    }
}

// NEW: String operations
fn evaluate_string_infix_op(op: &Token, l: &str, r: &str) -> Result<EvalResult, String> {
    match op {
        Token::Plus => Ok(EvalResult::Value(Object::String(format!("{}{}", l, r)))),
        Token::Equal => Ok(EvalResult::Value(Object::Boolean(l == r))),
        Token::NotEqual => Ok(EvalResult::Value(Object::Boolean(l != r))),
        Token::LessThan => Ok(EvalResult::Value(Object::Boolean(l < r))),
        Token::GreaterThan => Ok(EvalResult::Value(Object::Boolean(l > r))),
        Token::LessThanOrEqual => Ok(EvalResult::Value(Object::Boolean(l <= r))),
        Token::GreaterThanOrEqual => Ok(EvalResult::Value(Object::Boolean(l >= r))),
        _ => Err(format!("Unknown operator for strings: {:?}", op)),
    }
}

// NEW: Object equality comparison
fn objects_equal(left: &Object, right: &Object) -> bool {
    match (left, right) {
        (Object::Number(l), Object::Number(r)) => l == r,
        (Object::Boolean(l), Object::Boolean(r)) => l == r,
        (Object::String(l), Object::String(r)) => l == r,
        (Object::Null, Object::Null) => true,
        (Object::Array(l), Object::Array(r)) => {
            l.len() == r.len() && l.iter().zip(r.iter()).all(|(a, b)| objects_equal(a, b))
        },
        _ => false,
    }
}

fn evaluate_number_infix_op(op: &Token, l: f64, r: f64) -> Result<EvalResult, String> {
    match op {
        Token::Plus => Ok(EvalResult::Value(Object::Number(l + r))),
        Token::Minus => Ok(EvalResult::Value(Object::Number(l - r))),
        Token::Multiply => Ok(EvalResult::Value(Object::Number(l * r))),
        Token::Divide => if r == 0.0 { 
            Err("Division by zero".to_string()) 
        } else { 
            Ok(EvalResult::Value(Object::Number(l / r))) 
        },
        Token::Modulo => if r == 0.0 { 
            Err("Modulo by zero".to_string()) 
        } else { 
            Ok(EvalResult::Value(Object::Number(l % r))) 
        },
        Token::Equal => Ok(EvalResult::Value(Object::Boolean(l == r))),
        Token::NotEqual => Ok(EvalResult::Value(Object::Boolean(l != r))),
        Token::LessThan => Ok(EvalResult::Value(Object::Boolean(l < r))),
        Token::GreaterThan => Ok(EvalResult::Value(Object::Boolean(l > r))),
        Token::LessThanOrEqual => Ok(EvalResult::Value(Object::Boolean(l <= r))),
        Token::GreaterThanOrEqual => Ok(EvalResult::Value(Object::Boolean(l >= r))),
        _ => Err(format!("Unknown operator for numbers: {:?}", op)),
    }
}

fn evaluate_boolean_infix_op(op: &Token, l: bool, r: bool) -> Result<EvalResult, String> {
    match op {
        Token::Equal => Ok(EvalResult::Value(Object::Boolean(l == r))),
        Token::NotEqual => Ok(EvalResult::Value(Object::Boolean(l != r))),
        Token::And => Ok(EvalResult::Value(Object::Boolean(l && r))),
        Token::Or => Ok(EvalResult::Value(Object::Boolean(l || r))),
        _ => Err(format!("Unknown operator for booleans: {:?}", op)),
    }
}