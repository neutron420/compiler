use std::fmt;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Function {
        parameters: Vec<String>,
        body: super::parser::AstNode,
        closure: HashMap<String, Object>,
    },
    BuiltinFunction(fn(&[Object]) -> Result<Object, String>),
    Null,
}

// Manual PartialEq implementation (avoiding function pointer comparison)
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Number(a), Object::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Array(a), Object::Array(b)) => a == b,
            (Object::Function { parameters: pa, body: ba, closure: ca },
             Object::Function { parameters: pb, body: bb, closure: cb }) =>
                pa == pb && ba == bb && ca == cb,
            (Object::Null, Object::Null) => true,
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
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            },
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "{}", s), // Don't add quotes for display
            Object::Array(elements) => {
                let elements_str: Vec<String> = elements.iter()
                    .map(|e| match e {
                        Object::String(s) => format!("\"{}\"", s),
                        other => other.to_string(),
                    })
                    .collect();
                write!(f, "[{}]", elements_str.join(", "))
            },
            Object::Function { parameters, .. } => {
                write!(f, "function({})", parameters.join(", "))
            },
            Object::BuiltinFunction(_) => write!(f, "builtin function"),
            Object::Null => write!(f, "null"),
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
        }
    }
}

// Built-in functions
pub fn get_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    
    // I/O functions
    builtins.insert("print".to_string(), Object::BuiltinFunction(builtin_print));
    builtins.insert("println".to_string(), Object::BuiltinFunction(builtin_println));
    
    // Collection functions
    builtins.insert("len".to_string(), Object::BuiltinFunction(builtin_len));
    builtins.insert("push".to_string(), Object::BuiltinFunction(builtin_push));
    builtins.insert("pop".to_string(), Object::BuiltinFunction(builtin_pop));
    builtins.insert("first".to_string(), Object::BuiltinFunction(builtin_first));
    builtins.insert("last".to_string(), Object::BuiltinFunction(builtin_last));
    builtins.insert("rest".to_string(), Object::BuiltinFunction(builtin_rest));
    
    // Mathematical functions
    builtins.insert("abs".to_string(), Object::BuiltinFunction(builtin_abs));
    builtins.insert("sqrt".to_string(), Object::BuiltinFunction(builtin_sqrt));
    builtins.insert("pow".to_string(), Object::BuiltinFunction(builtin_pow));
    builtins.insert("floor".to_string(), Object::BuiltinFunction(builtin_floor));
    builtins.insert("ceil".to_string(), Object::BuiltinFunction(builtin_ceil));
    builtins.insert("round".to_string(), Object::BuiltinFunction(builtin_round));
    builtins.insert("min".to_string(), Object::BuiltinFunction(builtin_min));
    builtins.insert("max".to_string(), Object::BuiltinFunction(builtin_max));
    builtins.insert("sin".to_string(), Object::BuiltinFunction(builtin_sin));
    builtins.insert("cos".to_string(), Object::BuiltinFunction(builtin_cos));
    builtins.insert("tan".to_string(), Object::BuiltinFunction(builtin_tan));
    
    // String functions
    builtins.insert("substr".to_string(), Object::BuiltinFunction(builtin_substr));
    builtins.insert("upper".to_string(), Object::BuiltinFunction(builtin_upper));
    builtins.insert("lower".to_string(), Object::BuiltinFunction(builtin_lower));
    builtins.insert("trim".to_string(), Object::BuiltinFunction(builtin_trim));
    builtins.insert("split".to_string(), Object::BuiltinFunction(builtin_split));
    builtins.insert("join".to_string(), Object::BuiltinFunction(builtin_join));
    
    // Type checking and conversion functions
    builtins.insert("type".to_string(), Object::BuiltinFunction(builtin_type));
    builtins.insert("to_string".to_string(), Object::BuiltinFunction(builtin_to_string));
    builtins.insert("to_number".to_string(), Object::BuiltinFunction(builtin_to_number));
    
    builtins
}

// I/O Functions
fn builtin_print(args: &[Object]) -> Result<Object, String> {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            super::evaluator::add_output(" ");
        }
        super::evaluator::add_output(&arg.to_string());
    }
    Ok(Object::Null)
}

fn builtin_println(args: &[Object]) -> Result<Object, String> {
    builtin_print(args)?;
    super::evaluator::add_output("\n");
    Ok(Object::Null)
}

// Collection Functions
fn builtin_len(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("len() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::String(s) => Ok(Object::Number(s.chars().count() as f64)),
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

fn builtin_first(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("first() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                Ok(Object::Null)
            } else {
                Ok(arr[0].clone())
            }
        },
        other => Err(format!("first() not supported for {}", other.type_name())),
    }
}

fn builtin_last(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("last() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Array(arr) => {
            if arr.is_empty() {
                Ok(Object::Null)
            } else {
                Ok(arr[arr.len() - 1].clone())
            }
        },
        other => Err(format!("last() not supported for {}", other.type_name())),
    }
}

fn builtin_rest(args: &[Object]) -> Result<Object, String> {
    if args.len() != 1 {
        return Err(format!("rest() takes exactly 1 argument, got {}", args.len()));
    }
    
    match &args[0] {
        Object::Array(arr) => {
            if arr.len() <= 1 {
                Ok(Object::Array(vec![]))
            } else {
                Ok(Object::Array(arr[1..].to_vec()))
            }
        },
        other => Err(format!("rest() not supported for {}", other.type_name())),
    }
}
fn builtin_abs(args: &[Object]) -> Result<Object, String