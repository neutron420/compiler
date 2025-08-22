use super::lexer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Number(f64),
    Boolean(bool),
    Identifier(String),
    LetStatement { name: String, value: Box<AstNode> },
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
            Some(Token::LeftBrace) => self.parse_block_statement(),
            _ => self.parse_expression_statement(),
        }
    }

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
            Token::Identifier(name) => Ok(AstNode::Identifier(name)),
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
            _ => 0,
        }
    }
}
