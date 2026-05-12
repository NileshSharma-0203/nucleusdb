use crate::sql::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> i64 {
        let start = self.position;

        while let Some(c) = self.current_char() {
            if c.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.position]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn read_string(&mut self) -> String {
        self.advance();

        let start = self.position;

        while let Some(c) = self.current_char() {
            if c == '\'' {
                break;
            }

            self.advance();
        }

        let value: String = self.input[start..self.position].iter().collect();

        if self.current_char() == Some('\'') {
            self.advance();
        }

        value
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char() {
            Some('*') => {
                self.advance();
                Token::Star
            }

            Some(',') => {
                self.advance();
                Token::Comma
            }

            Some(';') => {
                self.advance();
                Token::Semicolon
            }

            Some('(') => {
                self.advance();
                Token::LeftParen
            }

            Some(')') => {
                self.advance();
                Token::RightParen
            }

            Some('=') => {
                self.advance();
                Token::Equals
            }

            Some('\'') => Token::String(self.read_string()),

            Some(c) if c.is_alphabetic() => {
                let ident = self.read_identifier();

                match ident.to_uppercase().as_str() {
                    "SELECT" => Token::Select,
                    "FROM" => Token::From,
                    "WHERE" => Token::Where,

                    "INSERT" => Token::Insert,
                    "INTO" => Token::Into,
                    "VALUES" => Token::Values,

                    "CREATE" => Token::Create,
                    "TABLE" => Token::Table,

                    "INT" => Token::IntType,
                    "TEXT" => Token::TextType,
                    "BOOL" => Token::BoolType,
                    "FLOAT" => Token::FloatType,
                    "EXPLAIN" => Token::Explain,
                    "BEGIN" => Token::Begin,
                    "COMMIT" => Token::Commit,
                    "ROLLBACK" => Token::Rollback,

                    _ => Token::Identifier(ident),
                }
            }

            Some(c) if c.is_numeric() => Token::Integer(self.read_number()),

            None => Token::EOF,

            _ => {
                self.advance();
                self.next_token()
            }
        }
    }
}