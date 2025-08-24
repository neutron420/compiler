// Add these new tokens to your existing Token enum in lexer.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Existing tokens...
    Number(f64),
    Identifier(String),
    Let,
    True,
    False,
    
    // NEW: Control flow tokens
    If,
    Else,
    While,
    For,
    Return,
    Break,
    Continue,
    
    // NEW: Function tokens  
    Fn,
    
    // NEW: Data structure tokens
    LeftBracket,    // [
    RightBracket,   // ]
    Comma,          // ,
    
    // NEW: String token
    String(String),
    
    // Existing operators...
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Not,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    
    // Existing delimiters...
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
}

// Add these cases to your tokenize function
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            // String literals
            '"' => {
                let mut string_val = String::new();
                while let Some(next_ch) = chars.next() {
                    if next_ch == '"' {
                        break;
                    }
                    if next_ch == '\\' {
                        // Handle escape sequences
                        match chars.next() {
                            Some('n') => string_val.push('\n'),
                            Some('t') => string_val.push('\t'),
                            Some('\\') => string_val.push('\\'),
                            Some('"') => string_val.push('"'),
                            Some(c) => string_val.push(c),
                            None => return Err("Unterminated string".to_string()),
                        }
                    } else {
                        string_val.push(next_ch);
                    }
                }
                tokens.push(Token::String(string_val));
            }
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                ident.push(ch);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(match ident.as_str() {
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    // NEW KEYWORDS
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "fn" => Token::Fn,
                    "return" => Token::Return,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    _ => Token::Identifier(ident),
                });
            }
            
            // NEW: Array brackets and comma
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            ',' => tokens.push(Token::Comma),
            
            // ... rest of your existing tokenization logic
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                num_str.push(ch);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_digit(10) || next_ch == '.' {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match num_str.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => return Err(format!("Invalid number: {}", num_str)),
                }
            }
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::Equal);
                } else {
                    tokens.push(Token::Assign);
                }
            }
            '!' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::NotEqual);
                } else {
                    tokens.push(Token::Not);
                }
            }
            '<' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::LessThanOrEqual);
                } else {
                    tokens.push(Token::LessThan);
                }
            }
            '>' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::GreaterThanOrEqual);
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }
            '&' => {
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(Token::And);
                } else {
                    return Err("Expected '&&'".to_string());
                }
            }
            '|' => {
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(Token::Or);
                } else {
                    return Err("Expected '||'".to_string());
                }
            }
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '%' => tokens.push(Token::Modulo),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ';' => tokens.push(Token::Semicolon),
            ' ' | '\t' | '\n' => (), 
            _ => return Err(format!("Unexpected character: {}", ch)),
        }
    }
    Ok(tokens)
}