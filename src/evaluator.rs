use super::parser::AstNode;
use super::lexer::Token;
use super::object::{Object, get_builtins};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::{self, Write};

pub type Environment = HashMap<String, Object>;

// Thread-safe output capture
lazy_static::lazy_static! {
    static ref OUTPUT_BUFFER: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

pub fn clear_output() {
    if let Ok(mut buffer) = OUTPUT_BUFFER.lock() {
        buffer.clear();
    }
}

pub fn get_output() -> String {
    if let Ok(buffer) = OUTPUT_BUFFER.lock() {
        buffer.join("")
    } else {
        String::new()
    }
}

pub fn add_output(text: &str) {
    if let Ok(mut buffer) = OUTPUT_BUFFER.lock() {
        buffer.push(text.to_string());
    }
}

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
    // Clear previous output
    clear_output();
    
    // Add builtins to environment if not present
    for (name, builtin) in get_builtins() {
        env.entry(name).or_insert(builtin);
    }
    
    match evaluate_internal(node, env)? {
        EvalResult::Value(obj) => {
            // If there was printed output, return it; otherwise return the final value
            let output = get_output();
            if !output.is_empty() {
                Ok(Object::String(output.trim_end().to_string()))
            } else if matches!(obj, Object::Null) {
                Ok(Object::String(String::new()))
            } else {
                Ok(obj)
            }
        },
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
        
        AstNode::WhileStatement { condition, body } => {
            let mut result = Object::Null;
            let mut iterations = 0;
            const MAX_ITERATIONS: usize = 10000; // Prevent infinite loops
            
            loop {
                iterations += 1;
                if iterations > MAX_ITERATIONS {
                    return Err("Loop exceeded maximum iterations (possible infinite loop)".to_string());
                }
                
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
        
        AstNode::ForStatement { init, condition, increment, body } => {
            // Create new scope for for loop
            let mut loop_env = env.clone();
            
            // Initialize
            evaluate_internal(init, &mut loop_env)?;
            
            let mut result = Object::Null;
            let mut iterations = 0;
            const MAX_ITERATIONS: usize = 10000;
            
            loop {
                iterations += 1;
                if iterations > MAX_ITERATIONS {
                    return Err("Loop exceeded maximum iterations (possible infinite loop)".to_string());
                }
                
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
        
        AstNode::FunctionDefinition { name, parameters, body } => {
            let function = Object::Function {
                parameters: parameters.clone(),
                body: (**body).clone(),
                closure: env.clone(),
            };
            env.insert(name.clone(), function.clone());
            Ok(EvalResult::Value(function))
        }
        
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
        
        AstNode::ReturnStatement { value } => {
            let return_value = match value {
                Some(expr) => evaluate_internal(expr, env)?.unwrap_value(),
                None => Object::Null,
            };
            Ok(EvalResult::Return(return_value))
        }
        
        AstNode::BreakStatement => Ok(EvalResult::Break),
        AstNode::ContinueStatement => Ok(EvalResult::Continue),
        
        AstNode::Array(elements) => {
            let values: Result<Vec<Object>, String> = elements.iter()
                .map(|elem| evaluate_internal(elem, env).map(|r| r.unwrap_value()))
                .collect();
            Ok(EvalResult::Value(Object::Array(values?)))
        }
        
        AstNode::ArrayAccess { array, index } => {
            let array_obj = evaluate_internal(array, env)?.unwrap_value();
            let index_obj = evaluate_internal(index, env)?.unwrap_value();
            
            match (&array_obj, &index_obj) {
                (Object::Array(arr), Object::Number(i)) => {
                    let idx = *i as i32;
                    if idx < 0 {
                        return Err("Array index cannot be negative".to_string());
                    }
                    let idx = idx as usize;
                    if idx >= arr.len() {
                        return Err(format!("Array index {} out of bounds (length {})", idx, arr.len()));
                    }
                    Ok(EvalResult::Value(arr[idx].clone()))
                }
                (Object::String(s), Object::Number(i)) => {
                    let idx = *i as i32;
                    if idx < 0 {
                        return Err("String index cannot be negative".to_string());
                    }
                    let idx = idx as usize;
                    let chars: Vec<char> = s.chars().collect();
                    if idx >= chars.len() {
                        return Err(format!("String index {} out of bounds (length {})", idx, chars.len()));
                    }
                    Ok(EvalResult::Value(Object::String(chars[idx].to_string())))
                }
                (Object::Array(_), _) => Err("Array index must be a number".to_string()),
                (Object::String(_), _) => Err("String index must be a number".to_string()),
                _ => Err(format!("Cannot index into {}", array_obj.type_name())),
            }
        }
        
        AstNode::Identifier(name) => {
            match env.get(name) {
                Some(obj) => Ok(EvalResult::Value(obj.clone())),
                None => Err(format!("Identifier not found: {}", name)),
            }
        }
        
        AstNode::Number(n) => {
            if n.is_infinite() || n.is_nan() {
                return Err("Invalid number: infinity or NaN".to_string());
            }
            Ok(EvalResult::Value(Object::Number(*n)))
        },
        AstNode::Boolean(b) => Ok(EvalResult::Value(Object::Boolean(*b))),
        AstNode::String(s) => Ok(EvalResult::Value(Object::String(s.clone()))),
        
        AstNode::PrefixExpression { op, right } => {
            let right_val = evaluate_internal(right, env)?.unwrap_value();
            match op {
                Token::Not => Ok(EvalResult::Value(Object::Boolean(!right_val.is_truthy()))),
                Token::Minus => match right_val {
                    Object::Number(n) => Ok(EvalResult::Value(Object::Number(-n))),
                    _ => Err(format!("Cannot negate {}", right_val.type_name())),
                },
                _ => Err(format!("Unknown prefix operator: {:?}", op)),
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

fn objects_equal(left: &Object, right: &Object) -> bool {
    match (left, right) {
        (Object::Number(l), Object::Number(r)) => (l - r).abs() < f64::EPSILON,
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
    // Check for invalid numbers
    if l.is_infinite() || l.is_nan() || r.is_infinite() || r.is_nan() {
        return Err("Cannot perform operations with infinity or NaN".to_string());
    }
    
    match op {
        Token::Plus => {
            let result = l + r;
            if result.is_infinite() {
                return Err("Arithmetic overflow".to_string());
            }
            Ok(EvalResult::Value(Object::Number(result)))
        },
        Token::Minus => {
            let result = l - r;
            if result.is_infinite() {
                return Err("Arithmetic overflow".to_string());
            }
            Ok(EvalResult::Value(Object::Number(result)))
        },
        Token::Multiply => {
            let result = l * r;
            if result.is_infinite() {
                return Err("Arithmetic overflow".to_string());
            }
            Ok(EvalResult::Value(Object::Number(result)))
        },
        Token::Divide => {
            if r == 0.0 {
                return Err("Division by zero".to_string());
            }
            let result = l / r;
            if result.is_infinite() || result.is_nan() {
                return Err("Division resulted in infinity or NaN".to_string());
            }
            Ok(EvalResult::Value(Object::Number(result)))
        },
        Token::Modulo => {
            if r == 0.0 {
                return Err("Modulo by zero".to_string());
            }
            let result = l % r;
            Ok(EvalResult::Value(Object::Number(result)))
        },
        Token::Equal => Ok(EvalResult::Value(Object::Boolean((l - r).abs() < f64::EPSILON))),
        Token::NotEqual => Ok(EvalResult::Value(Object::Boolean((l - r).abs() >= f64::EPSILON))),
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