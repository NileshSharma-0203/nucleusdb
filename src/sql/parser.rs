use crate::sql::ast::{ColumnDefinition, DataType, Expression, Statement, Value};
use crate::sql::lexer::Lexer;
use crate::sql::token::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn parse_explain(&mut self) -> Result<Statement, String> {
        self.advance();
    
        let statement = self.parse_statement()?;
    
        Ok(Statement::Explain {
            statement: Box::new(statement),
        })
    }
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();

        Self {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token.clone() {
            Token::Select => self.parse_select(),
            Token::Create => self.parse_create_table(),
            Token::Insert => self.parse_insert(),
            Token::Explain => self.parse_explain(),
            Token::Begin => {
                self.advance();
                Ok(Statement::Begin)
            }
            
            Token::Commit => {
                self.advance();
                Ok(Statement::Commit)
            }
            
            Token::Rollback => {
                self.advance();
                Ok(Statement::Rollback)
            }
            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }

    fn parse_select(&mut self) -> Result<Statement, String> {
        self.advance();
    
        if self.current_token != Token::Star {
            return Err("Expected * after SELECT".to_string());
        }
    
        self.advance();
    
        if self.current_token != Token::From {
            return Err("Expected FROM after *".to_string());
        }
    
        self.advance();
    
        let table = match self.current_token.clone() {
            Token::Identifier(name) => name,
            other => return Err(format!("Expected table name, found {:?}", other)),
        };
    
        self.advance();
    
        let filter = if self.current_token == Token::Where {
            self.advance();
    
            let column = match self.current_token.clone() {
                Token::Identifier(name) => name,
                other => return Err(format!("Expected column name after WHERE, found {:?}", other)),
            };
    
            self.advance();
    
            if self.current_token != Token::Equals {
                return Err("Expected = after WHERE column".to_string());
            }
    
            self.advance();
    
            let value = match self.current_token.clone() {
                Token::Integer(n) => Value::Int(n),
                Token::String(s) => Value::Text(s),
                other => return Err(format!("Expected value after =, found {:?}", other)),
            };
    
            Some(Expression::Equals { column, value })
        } else {
            None
        };
    
        Ok(Statement::Select { table, filter })
    }
    fn parse_create_table(&mut self) -> Result<Statement, String> {
        self.advance();

        if self.current_token != Token::Table {
            return Err("Expected TABLE after CREATE".to_string());
        }

        self.advance();

        let table = match self.current_token.clone() {
            Token::Identifier(name) => name,
            other => return Err(format!("Expected table name, found {:?}", other)),
        };

        self.advance();

        if self.current_token != Token::LeftParen {
            return Err("Expected ( after table name".to_string());
        }

        self.advance();

        let mut columns = Vec::new();

        loop {
            let column_name = match self.current_token.clone() {
                Token::Identifier(name) => name,
                other => return Err(format!("Expected column name, found {:?}", other)),
            };

            self.advance();

            let data_type = match self.current_token.clone() {
                Token::IntType => DataType::Int,
                Token::TextType => DataType::Text,
                Token::BoolType => DataType::Bool,
                Token::FloatType => DataType::Float,
                other => return Err(format!("Expected data type, found {:?}", other)),
            };

            columns.push(ColumnDefinition {
                name: column_name,
                data_type,
            });

            self.advance();

            match self.current_token {
                Token::Comma => {
                    self.advance();
                }
                Token::RightParen => {
                    break;
                }
                ref other => return Err(format!("Expected comma or ), found {:?}", other)),
            }
        }

        Ok(Statement::CreateTable { table, columns })
    }

    fn parse_insert(&mut self) -> Result<Statement, String> {
        self.advance();

        if self.current_token != Token::Into {
            return Err("Expected INTO after INSERT".to_string());
        }

        self.advance();

        let table = match self.current_token.clone() {
            Token::Identifier(name) => name,
            other => return Err(format!("Expected table name, found {:?}", other)),
        };

        self.advance();

        if self.current_token != Token::Values {
            return Err("Expected VALUES after table name".to_string());
        }

        self.advance();

        if self.current_token != Token::LeftParen {
            return Err("Expected ( after VALUES".to_string());
        }

        self.advance();

        let mut values = Vec::new();

        loop {
            let value = match self.current_token.clone() {
                Token::Integer(n) => Value::Int(n),
                Token::String(s) => Value::Text(s),
                other => return Err(format!("Expected value, found {:?}", other)),
            };

            values.push(value);

            self.advance();

            match self.current_token {
                Token::Comma => {
                    self.advance();
                }
                Token::RightParen => {
                    break;
                }
                ref other => return Err(format!("Expected comma or ), found {:?}", other)),
            }
        }

        Ok(Statement::Insert { table, values })
    }
}