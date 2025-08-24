use std::fmt;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Boolean(bool),
    String(String),                                    // NEW
    Array(Vec<Object>),                               // NEW
    Function {                                        // NEW
        parameters: Vec<String>,
        body: super::parser::AstNode,
        closure: HashMap<String, Object>,
    },
    BuiltinFunction(fn(&[Object]) -> Result<Object, String>),  // NEW
    Null,                                            // NEW
    ReturnValue(Box<Object>),                        // NEW - for early returns
}

// Manual PartialEq implementation (avoiding function pointer comparison)
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Array(a), Object::Array(b)) => a == b,
            (Object::Function { parameters: pa, body: ba, closure: ca },
             Object::Function { parameters: pb, body: bb, closure: cb }) =>
                pa == pb && ba == bb && ca == cb,
            (Object::Null, Object::Null) => true,
            (Object::ReturnValue(a), Object::ReturnValue(b)) => a == b,
            // Do not compare BuiltinFunction by pointer
            (Object::BuiltinFunction(_), Object::BuiltinFunction(_)) => false,
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            },
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Array(elements) => {
                let elements_str: Vec<String> = elements.iter()
                    .map(|e| e.to_string())
                    .collect();
                write!(f, "[{}]", elements_str.join(", "))
            },
            Object::Function { parameters, .. } => {
                write!(f, "function({})", parameters.join(", "))
            },
            Object::BuiltinFunction(_) => write!(f, "builtin function"),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Boolean(b) => *b,
            Object::Number(n) => *n != 0.0,
            Object::String(s) => !s.is_empty(),
            Object::Array(arr) => !arr.is_empty(),
            Object::Null => false,
            Object::Function { .. } => true,
            Object::BuiltinFunction(_) => true,
            Object::ReturnValue(obj) => obj.is_truthy(),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Object::Number(_) => "number",
            Object::Boolean(_) => "boolean", 
            Object::String(_) => "string",
            Object::Array(_) => "array",
            Object::Function { .. } => "function",
            Object::BuiltinFunction(_) => "builtin",
            Object::Null => "null",
            Object::ReturnValue(obj) => obj.type_name(),
        }
    }
}

// Built-in functions
pub fn get_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    
    // print function
    builtins.insert("print".to_string(), Object::BuiltinFunction(builtin_print));
    
    // len function  
    builtins.insert("len".to_string(), Object::BuiltinFunction(builtin_len));
    
    // push function for arrays
    builtins.insert("push".to_string(), Object::BuiltinFunction(builtin_push));
    
    // pop function for arrays
    builtins.insert("pop".to_string(), Object::BuiltinFunction(builtin_pop));
    
    // Mathematical functions
    builtins.insert("abs".to_string(), Object::BuiltinFunction(builtin_abs));
    builtins.insert("sqrt".to_string(), Object::BuiltinFunction(builtin_sqrt));
    builtins.insert("pow".to_string(), Object::BuiltinFunction(builtin_pow));
    builtins.insert("floor".to_string(), Object::BuiltinFunction(builtin_floor));
    builtins.insert("ceil".to_string(), Object::BuiltinFunction(builtin_ceil));
    builtins.insert("round".to_string(), Object::BuiltinFunction(builtin_round));
    builtins.insert("min".to_string(), Object::BuiltinFunction(builtin_min));
    builtins.insert("max".to_string(), Object::BuiltinFunction(builtin_max));
    
    // String functions
    builtins.insert("substr".to_string(), Object::BuiltinFunction(builtin_substr));
    builtins.insert("upper".to_string(), Object::BuiltinFunction(builtin_upper));
    builtins.insert("lower".to_string(), Object::BuiltinFunction(builtin_lower));
    
    // Type checking functions
    builtins.insert("type".to_string(), Object::BuiltinFunction(builtin_type));
    
    builtins
}

fn builtin_print(args: &[Object]) -> Result<Object, String> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        match arg {
            Object::String(s) => print!("{}", s),
            other => print!("{}", other),
        }
    }
    println!();
    Ok(Object::Null)
}

fn builtin_len(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("len() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::String(s) => Ok(Object::Number(s.len() as f64)),
        Object::Array(arr) => Ok(Object::Number(arr.len() as f64)),
        other => Err(format!("len() not supported for {}", other.type_name())),
    }
}

fn builtin_push(args: &[Object]) -> Result<Object, String> {
    if args.len() != 2 {
        return Err(format!("push() takes exactly 2 arguments, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Ok(Object::Array(new_arr))
        },
        other => Err(format!("push() not supported for {}", other.type_name())),
    }
}

fn builtin_pop(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("pop() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                return Err("Cannot pop from empty array".to_string());
            }
            let mut new_arr = arr.clone();
            let popped = new_arr.pop().unwrap();
            Ok(popped)
        },
        other => Err(format!("pop() not supported for {}", other.type_name())),
    }
}

fn builtin_abs(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("abs() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Number(n) => Ok(Object::Number(n.abs())),
        other => Err(format!("abs() not supported for {}", other.type_name())),
    }
}

fn builtin_sqrt(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("sqrt() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Number(n) => {
            if *n < 0.0 {
                return Err("sqrt() of negative number".to_string());
            }
            Ok(Object::Number(n.sqrt()))
        },
        other => Err(format!("sqrt() not supported for {}", other.type_name())),
    }
}

fn builtin_pow(args: &[Object]) -> Result<Object, String> {
    if args.len() != 2 {
        return Err(format!("pow() takes exactly 2 arguments, got {}", args.len()));
    }
    
    match (&args[0], &args[1]) {
        (Object::Number(base), Object::Number(exp)) => {
            Ok(Object::Number(base.powf(*exp)))
        },
        _ => Err("pow() requires two numbers".to_string()),
    }
}

fn builtin_floor(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("floor() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Number(n) => Ok(Object::Number(n.floor())),
        other => Err(format!("floor() not supported for {}", other.type_name())),
    }
}

fn builtin_ceil(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("ceil() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Number(n) => Ok(Object::Number(n.ceil())),
        other => Err(format!("ceil() not supported for {}", other.type_name())),
    }
}

fn builtin_round(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("round() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Number(n) => Ok(Object::Number(n.round())),
        other => Err(format!("round() not supported for {}", other.type_name())),
    }
}

fn builtin_min(args: &[Object]) -> Result<Object, String> {
    if args.is_empty() {
        return Err("min() requires at least 1 argument".to_string());
    }
    
    let mut min_val = match &args[0] {
        Object::Number(n) => *n,
        other => return Err(format!("min() not supported for {}", other.type_name())),
    };
    
    for arg in &args[1..] {
        match arg {
            Object::Number(n) => {
                if *n < min_val {
                    min_val = *n;
                }
            },
            other => return Err(format!("min() not supported for {}", other.type_name())),
        }
    }
    
    Ok(Object::Number(min_val))
}

fn builtin_max(args: &[Object]) -> Result<Object, String> {
    if args.is_empty() {
        return Err("max() requires at least 1 argument".to_string());
    }
    
    let mut max_val = match &args[0] {
        Object::Number(n) => *n,
        other => return Err(format!("max() not supported for {}", other.type_name())),
    };
    
    for arg in &args[1..] {
        match arg {
            Object::Number(n) => {
                if *n > max_val {
                    max_val = *n;
                }
            },
            other => return Err(format!("max() not supported for {}", other.type_name())),
        }
    }
    
    Ok(Object::Number(max_val))
}

fn builtin_substr(args: &[Object]) -> Result<Object, String> {
    if args.len() < 2 || args.len() > 3 {
        return Err(format!("substr() takes 2 or 3 arguments, got {}", args.len()));
    }
    
    match (&args[0], &args[1]) {
        (Object::String(s), Object::Number(start)) => {
            let start_idx = *start as usize;
            if start_idx >= s.len() {
                return Ok(Object::String("".to_string()));
            }
            
            let result = if args.len() == 3 {
                match &args[2] {
                    Object::Number(len) => {
                        let len_val = *len as usize;
                        let end_idx = std::cmp::min(start_idx + len_val, s.len());
                        s.chars().skip(start_idx).take(end_idx - start_idx).collect()
                    },
                    _ => return Err("substr() length must be a number".to_string()),
                }
            } else {
                s.chars().skip(start_idx).collect()
            };
            Ok(Object::String(result))
        },
        _ => Err("substr() requires string and number".to_string()),
    }
}

fn builtin_upper(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("upper() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::String(s) => Ok(Object::String(s.to_uppercase())),
        other => Err(format!("upper() not supported for {}", other.type_name())),
    }
}

fn builtin_lower(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("lower() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::String(s) => Ok(Object::String(s.to_lowercase())),
        other => Err(format!("lower() not supported for {}", other.type_name())),
    }
}

fn builtin_type(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("type() takes exactly 1 argument, got {}", args.len()));
    }
    
    Ok(Object::String(args[0].type_name().to_string()))
}