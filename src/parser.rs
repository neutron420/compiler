use super::lexer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    // Existing nodes
    Number(f64),
    Boolean(bool),
    String(String),  // NEW
    Identifier(String),
    
    // NEW: Array support
    Array(Vec<AstNode>),
    ArrayAccess { array: Box<AstNode>, index: Box<AstNode> },
    
    // Existing statements
    LetStatement { name: String, value: Box<AstNode> },
    
    // NEW: Control flow
    IfStatement { 
        condition: Box<AstNode>, 
        then_branch: Box<AstNode>, 
        else_branch: Option<Box<AstNode>> 
    },
    WhileStatement { condition: Box<AstNode>, body: Box<AstNode> },
    ForStatement { 
        init: Box<AstNode>, 
        condition: Box<AstNode>, 
        increment: Box<AstNode>, 
        body: Box<AstNode> 
    },
    
    // NEW: Functions
    FunctionDefinition { 
        name: String, 
        parameters: Vec<String>, 
        body: Box<AstNode> 
    },
    FunctionCall { name: String, arguments: Vec<AstNode> },
    ReturnStatement { value: Option<Box<AstNode>> },
    
    // NEW: Control statements
    BreakStatement,
    ContinueStatement,
    
    // Existing expressions
    InfixExpression { op: Token, left: Box<AstNode>, right: Box<AstNode> },
    PrefixExpression { op: Token, right: Box<AstNode> },
    BlockStatement(Vec<AstNode>),
    Program(Vec<AstNode>),
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens: tokens.into_iter().peekable() }
    }

    pub fn parse_program(&mut self) -> Result<AstNode, String> {
        let mut statements = Vec::new();
        while self.tokens.peek().is_some() {
            statements.push(self.parse_statement()?);
        }
        Ok(AstNode::Program(statements))
    }

    fn parse_statement(&mut self) -> Result<AstNode, String> {
        match self.tokens.peek() {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::While) => self.parse_while_statement(),
            Some(Token::For) => self.parse_for_statement(),
            Some(Token::Fn) => self.parse_function_definition(),
            Some(Token::Return) => self.parse_return_statement(),
            Some(Token::Break) => {
                self.tokens.next();
                if self.tokens.peek() == Some(&Token::Semicolon) {
                    self.tokens.next();
                }
                Ok(AstNode::BreakStatement)
            },
            Some(Token::Continue) => {
                self.tokens.next();
                if self.tokens.peek() == Some(&Token::Semicolon) {
                    self.tokens.next();
                }
                Ok(AstNode::ContinueStatement)
            },
            Some(Token::LeftBrace) => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    // NEW: If statement parsing
    fn parse_if_statement(&mut self) -> Result<AstNode, String> {
        self.tokens.next(); // consume 'if'
        
        let condition = self.parse_expression(0)?;
        let then_branch = self.parse_statement()?;
        
        let else_branch = if self.tokens.peek() == Some(&Token::Else) {
            self.tokens.next(); // consume 'else'
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        
        Ok(AstNode::IfStatement {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    // NEW: While statement parsing
    fn parse_while_statement(&mut self) -> Result<AstNode, String> {
        self.tokens.next(); // consume 'while'
        
        let condition = self.parse_expression(0)?;
        let body = self.parse_statement()?;
        
        Ok(AstNode::WhileStatement {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    // NEW: For statement parsing
    fn parse_for_statement(&mut self) -> Result<AstNode, String> {
        self.tokens.next(); // consume 'for'
        
        match self.tokens.next() {
            Some(Token::LeftParen) => (),
            _ => return Err("Expected '(' after 'for'".to_string()),
        };
        
        let init = self.parse_statement()?;
        let condition = self.parse_expression(0)?;
        
        match self.tokens.next() {
            Some(Token::Semicolon) => (),
            _ => return Err("Expected ';' after condition in for loop".to_string()),
        };
        
        let increment = self.parse_expression(0)?;
        
        match self.tokens.next() {
            Some(Token::RightParen) => (),
            _ => return Err("Expected ')' in for loop".to_string()),
        };
        
        let body = self.parse_statement()?;
        
        Ok(AstNode::ForStatement {
            init: Box::new(init),
            condition: Box::new(condition),
            increment: Box::new(increment),
            body: Box::new(body),
        })
    }

    // NEW: Function definition parsing
    fn parse_function_definition(&mut self) -> Result<AstNode, String> {
        self.tokens.next(); // consume 'fn'
        
        let name = match self.tokens.next() {
            Some(Token::Identifier(name)) => name,
            _ => return Err("Expected function name".to_string()),
        };
        
        match self.tokens.next() {
            Some(Token::LeftParen) => (),
            _ => return Err("Expected '(' after function name".to_string()),
        };
        
        let mut parameters = Vec::new();
        while self.tokens.peek() != Some(&Token::RightParen) {
            match self.tokens.next() {
                Some(Token::Identifier(param)) => parameters.push(param),
                _ => return Err("Expected parameter name".to_string()),
            };
            
            if self.tokens.peek() == Some(&Token::Comma) {
                self.tokens.next(); // consume comma
            } else if self.tokens.peek() != Some(&Token::RightParen) {
                return Err("Expected ',' or ')' in parameter list".to_string());
            }
        }
        
        match self.tokens.next() {
            Some(Token::RightParen) => (),
            _ => return Err("Expected ')' after parameters".to_string()),
        };
        
        let body = self.parse_statement()?;
        
        Ok(AstNode::FunctionDefinition {
            name,
            parameters,
            body: Box::new(body),
        })
    }

    // NEW: Return statement parsing
    fn parse_return_statement(&mut self) -> Result<AstNode, String> {
        self.tokens.next(); // consume 'return'
        
        let value = if self.tokens.peek() == Some(&Token::Semicolon) {
            None
        } else {
            Some(Box::new(self.parse_expression(0)?))
        };
        
        if self.tokens.peek() == Some(&Token::Semicolon) {
            self.tokens.next();
        }
        
        Ok(AstNode::ReturnStatement { value })
    }

    // Existing methods with enhancements...
    fn parse_let_statement(&mut self) -> Result<AstNode, String> {
        self.tokens.next(); 
        let name = match self.tokens.next() {
            Some(Token::Identifier(name)) => name,
            _ => return Err("Expected identifier after 'let'".to_string()),
        };
        match self.tokens.next() {
            Some(Token::Assign) => (),
            _ => return Err("Expected '=' after identifier".to_string()),
        };
        let value = self.parse_expression(0)?;
        if self.tokens.peek() == Some(&Token::Semicolon) {
            self.tokens.next();
        }
        Ok(AstNode::LetStatement { name, value: Box::new(value) })
    }
    
    fn parse_block_statement(&mut self) -> Result<AstNode, String> {
        self.tokens.next();
        let mut statements = Vec::new();
        while self.tokens.peek().is_some() && self.tokens.peek() != Some(&Token::RightBrace) {
            statements.push(self.parse_statement()?);
        }
        match self.tokens.next() {
            Some(Token::RightBrace) => Ok(AstNode::BlockStatement(statements)),
            _ => Err("Expected '}' to close block".to_string()),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<AstNode, String> {
        let expr = self.parse_expression(0)?;
        if self.tokens.peek() == Some(&Token::Semicolon) {
            self.tokens.next();
        }
        Ok(expr)
    }

    fn parse_expression(&mut self, min_precedence: u8) -> Result<AstNode, String> {
        let mut left = self.parse_prefix()?;
        
        loop {
            let op = match self.tokens.peek() {
                Some(token) => token.clone(),
                None => break,
            };

            let precedence = self.get_infix_precedence(&op);
            if precedence < min_precedence {
                break;
            }

            // Handle array access
            if op == Token::LeftBracket {
                self.tokens.next(); // consume '['
                let index = self.parse_expression(0)?;
                match self.tokens.next() {
                    Some(Token::RightBracket) => (),
                    _ => return Err("Expected ']'".to_string()),
                };
                left = AstNode::ArrayAccess { 
                    array: Box::new(left), 
                    index: Box::new(index) 
                };
                continue;
            }

            let op_token = self.tokens.next().unwrap();
            let right = self.parse_expression(precedence + 1)?;
            left = AstNode::InfixExpression { op: op_token, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<AstNode, String> {
        let token = match self.tokens.next() {
            Some(t) => t,
            None => return Err("Unexpected end of input while parsing prefix".to_string()),
        };

        match token {
            Token::Number(n) => Ok(AstNode::Number(n)),
            Token::True => Ok(AstNode::Boolean(true)),
            Token::False => Ok(AstNode::Boolean(false)),
            Token::String(s) => Ok(AstNode::String(s)),  // NEW
            Token::Identifier(name) => {
                // Check for function call
                if self.tokens.peek() == Some(&Token::LeftParen) {
                    self.tokens.next(); // consume '('
                    let mut arguments = Vec::new();
                    
                    while self.tokens.peek() != Some(&Token::RightParen) {
                        arguments.push(self.parse_expression(0)?);
                        if self.tokens.peek() == Some(&Token::Comma) {
                            self.tokens.next(); // consume comma
                        } else if self.tokens.peek() != Some(&Token::RightParen) {
                            return Err("Expected ',' or ')' in function call".to_string());
                        }
                    }
                    
                    match self.tokens.next() {
                        Some(Token::RightParen) => (),
                        _ => return Err("Expected ')' after arguments".to_string()),
                    };
                    
                    Ok(AstNode::FunctionCall { name, arguments })
                } else {
                    Ok(AstNode::Identifier(name))
                }
            },
            // NEW: Array literal parsing
            Token::LeftBracket => {
                let mut elements = Vec::new();
                
                while self.tokens.peek() != Some(&Token::RightBracket) {
                    elements.push(self.parse_expression(0)?);
                    if self.tokens.peek() == Some(&Token::Comma) {
                        self.tokens.next(); // consume comma
                    } else if self.tokens.peek() != Some(&Token::RightBracket) {
                        return Err("Expected ',' or ']' in array".to_string());
                    }
                }
                
                match self.tokens.next() {
                    Some(Token::RightBracket) => Ok(AstNode::Array(elements)),
                    _ => return Err("Expected ']' to close array".to_string()),
                }
            },
            op @ (Token::Minus | Token::Not) => {
                let right = self.parse_expression(6)?;
                Ok(AstNode::PrefixExpression { op, right: Box::new(right) })
            }
            Token::LeftParen => {
                let expr = self.parse_expression(0)?;
                match self.tokens.next() {
                    Some(Token::RightParen) => Ok(expr),
                    _ => Err("Expected ')'".to_string()),
                }
            }
            Token::LeftBrace => self.parse_block_statement(),
            t => Err(format!("Unexpected token for prefix expression: {:?}", t)),
        }
    }

    fn get_infix_precedence(&self, token: &Token) -> u8 {
        match token {
            Token::Or => 1,
            Token::And => 2,
            Token::Equal | Token::NotEqual | Token::LessThan | Token::GreaterThan | Token::LessThanOrEqual | Token::GreaterThanOrEqual => 3,
            Token::Plus | Token::Minus => 4,
            Token::Multiply | Token::Divide | Token::Modulo => 5,
            Token::LeftBracket => 7,  // Array access has high precedence
            _ => 0,
        }
    }
}