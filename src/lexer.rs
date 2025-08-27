#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    
    // Keywords
    Let,
    True,
    False,
    If,
    Else,
    While,
    For,
    Fn,
    Return,
    Break,
    Continue,
    
    // Operators
    Assign,        // =
    Plus,          // +
    Minus,         // -
    Multiply,      // *
    Divide,        // /
    Modulo,        // %
    Not,           // !
    
    // Comparison
    Equal,             // ==
    NotEqual,          // !=
    LessThan,          // <
    GreaterThan,       // >
    LessThanOrEqual,   // <=
    GreaterThanOrEqual,// >=
    
    // Logical
    And,           // &&
    Or,            // ||
    
    // Delimiters
    LeftParen,     // (
    RightParen,    // )
    LeftBrace,     // {
    RightBrace,    // }
    LeftBracket,   // [
    RightBracket,  // ]
    Comma,         // ,
    Semicolon,     // ;
    
    // Special
    Eof,
}

#[derive(Debug, Clone)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct TokenWithPosition {
    pub token: Token,
    pub position: TokenPosition,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    fn current_position(&self) -> TokenPosition {
        TokenPosition {
            line: self.line,
            column: self.column,
        }
    }
    
    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }
    
    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position + 1).copied()
    }
    
    fn advance(&mut self) {
        if self.current_char() == Some('\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.position += 1;
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) -> Result<(), String> {
        if self.current_char() == Some('/') && self.peek_char() == Some('/') {
            // Single-line comment
            while let Some(ch) = self.current_char() {
                self.advance();
                if ch == '\n' {
                    break;
                }
            }
        } else if self.current_char() == Some('/') && self.peek_char() == Some('*') {
            // Multi-line comment
            self.advance(); // skip '/'
            self.advance(); // skip '*'
            
            while let Some(ch) = self.current_char() {
                if ch == '*' && self.peek_char() == Some('/') {
                    self.advance(); // skip '*'
                    self.advance(); // skip '/'
                    return Ok(());
                }
                self.advance();
            }
            return Err(format!("Unterminated multi-line comment at line {}", self.line));
        }
        Ok(())
    }
    
    fn read_string(&mut self) -> Result<String, String> {
        let mut value = String::new();
        let start_line = self.line;
        self.advance(); // skip opening quote
        
        while let Some(ch) = self.current_char() {
            match ch {
                '"' => {
                    self.advance();
                    return Ok(value);
                }
                '\\' => {
                    self.advance();
                    match self.current_char() {
                        Some('n') => value.push('\n'),
                        Some('t') => value.push('\t'),
                        Some('r') => value.push('\r'),
                        Some('\\') => value.push('\\'),
                        Some('"') => value.push('"'),
                        Some('0') => value.push('\0'),
                        Some(c) => {
                            return Err(format!("Invalid escape sequence '\\{}' at line {}, column {}", 
                                c, self.line, self.column));
                        }
                        None => {
                            return Err(format!("Unterminated string starting at line {}", start_line));
                        }
                    }
                    self.advance();
                }
                c => {
                    value.push(c);
                    self.advance();
                }
            }
        }
        
        Err(format!("Unterminated string starting at line {}", start_line))
    }
    
    fn read_number(&mut self) -> Result<f64, String> {
        let start_pos = self.position;
        let mut has_dot = false;
        
        while let Some(ch) = self.current_char() {
            match ch {
                '0'..='9' => self.advance(),
                '.' if !has_dot => {
                    has_dot = true;
                    self.advance();
                }
                _ => break,
            }
        }
        
        let number_str: String = self.input[start_pos..self.position].iter().collect();
        
        // Handle edge cases
        if number_str == "." {
            return Err(format!("Invalid number '.' at line {}, column {}", 
                self.line, self.column - 1));
        }
        
        number_str.parse::<f64>()
            .map_err(|_| format!("Invalid number '{}' at line {}, column {}", 
                number_str, self.line, self.column - number_str.len()))
    }
    
    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        self.input[start_pos..self.position].iter().collect()
    }
    
    fn identifier_to_token(&self, ident: &str) -> Token {
        match ident {
            "let" => Token::Let,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            _ => Token::Identifier(ident.to_string()),
        }
    }
    
    pub fn next_token(&mut self) -> Result<TokenWithPosition, String> {
        self.skip_whitespace();
        
        // Handle comments
        if self.current_char() == Some('/') && 
           (self.peek_char() == Some('/') || self.peek_char() == Some('*')) {
            self.skip_comment()?;
            self.skip_whitespace();
        }
        
        let position = self.current_position();
        
        let token = match self.current_char() {
            None => Token::Eof,
            
            Some(ch) => match ch {
                // String literals
                '"' => Token::String(self.read_string()?),
                
                // Numbers
                '0'..='9' => Token::Number(self.read_number()?),
                
                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let ident = self.read_identifier();
                    return Ok(TokenWithPosition {
                        token: self.identifier_to_token(&ident),
                        position,
                    });
                }
                
                // Two-character operators
                '=' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        Token::Equal
                    } else {
                        Token::Assign
                    }
                }
                
                '!' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        Token::NotEqual
                    } else {
                        Token::Not
                    }
                }
                
                '<' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        Token::LessThanOrEqual
                    } else {
                        Token::LessThan
                    }
                }
                
                '>' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        Token::GreaterThanOrEqual
                    } else {
                        Token::GreaterThan
                    }
                }
                
                '&' => {
                    self.advance();
                    if self.current_char() == Some('&') {
                        self.advance();
                        Token::And
                    } else {
                        return Err(format!("Unexpected character '&' at line {}, column {}. Did you mean '&&'?", 
                            position.line, position.column));
                    }
                }
                
                '|' => {
                    self.advance();
                    if self.current_char() == Some('|') {
                        self.advance();
                        Token::Or
                    } else {
                        return Err(format!("Unexpected character '|' at line {}, column {}. Did you mean '||'?", 
                            position.line, position.column));
                    }
                }
                
                // Single-character tokens
                '+' => { self.advance(); Token::Plus }
                '-' => { self.advance(); Token::Minus }
                '*' => { self.advance(); Token::Multiply }
                '/' => { self.advance(); Token::Divide }
                '%' => { self.advance(); Token::Modulo }
                '(' => { self.advance(); Token::LeftParen }
                ')' => { self.advance(); Token::RightParen }
                '{' => { self.advance(); Token::LeftBrace }
                '}' => { self.advance(); Token::RightBrace }
                '[' => { self.advance(); Token::LeftBracket }
                ']' => { self.advance(); Token::RightBracket }
                ',' => { self.advance(); Token::Comma }
                ';' => { self.advance(); Token::Semicolon }
                
                // Unexpected character
                c => {
                    return Err(format!("Unexpected character '{}' at line {}, column {}", 
                        c, position.line, position.column));
                }
            }
        };
        
        Ok(TokenWithPosition { token, position })
    }
    
    pub fn tokenize(mut self) -> Result<Vec<TokenWithPosition>, String> {
        let mut tokens = Vec::new();
        
        loop {
            let token_with_pos = self.next_token()?;
            let is_eof = matches!(token_with_pos.token, Token::Eof);
            tokens.push(token_with_pos);
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}

// Convenience function for backward compatibility
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let lexer = Lexer::new(input);
    let tokens_with_pos = lexer.tokenize()?;
    Ok(tokens_with_pos.into_iter().map(|twp| twp.token).collect())
}