use std::collections::HashMap;

use crate::pascal_interpreter::{
    literal::Literal,
    token::*,
    error::RuntimeError,
};

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
    current_char: Option<char>,
    line: u32,
    col: u32,
    reserved_keywords: HashMap<String, TokenType>,
    reserved_symbols: HashMap<char, TokenType>
}

impl<'a> Lexer<'a> {
    pub fn from(text: &'a str) -> Lexer<'a> {
        if text.is_empty() {
            panic!("Missing input");
        }

        Lexer {
            text: &text,
            pos: 0,
            current_char: text.chars().next(),
            line: 1u32,
            col: 1u32,
            reserved_keywords: Lexer::init_reserved_keywords(),
            reserved_symbols: Lexer::init_reserved_symbols()
        }
    }

    // Advance the `pos` pointer and set the `current_char`
    fn advance(&mut self) {
        if self.current_char.unwrap() == '\n' {
            self.line += 1u32;
            self.col = 0u32;
        }

        self.pos += 1;
        
        if self.pos >= self.text.len() {
            self.current_char = None;
        } else {
            self.current_char = self.text.chars().nth(self.pos);
            self.col+= 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.filter(|c| c.is_whitespace()).is_some() {
            self.advance();
        }
    }

    fn skip_comment(&mut self) {    
        while self.current_char.filter(|&c| c != '}').is_some() {
            self.advance();
        }
        self.advance();
}

    /// Return an integer consumed from input
    fn get_number(&mut self) -> Token {
        let mut result = String::new();
        while self.current_char.filter(|c| c.is_digit(10)).is_some() {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        if self.current_char.filter(|&c| c == '.').is_some() {
            result.push(self.current_char.unwrap());
            self.advance();

            while self.current_char.filter(|c| c.is_digit(10)).is_some() {
                result.push(self.current_char.unwrap());
                self.advance();
            }
            let value = Literal::from_float(result.parse::<f64>().unwrap());
            return Token::new(TokenType::FloatConst, value, self.line, self.col);
        }

        let value = Literal::from_int(result.parse::<i64>().unwrap());
        Token::new(TokenType::IntegerConst, value, self.line, self.col)
    }

    fn get_identifier(&mut self) -> String {
        let mut result = String::new();
        while self.current_char.filter(|c| c.is_alphanumeric()).is_some() {
            result.push(self.current_char.unwrap());
            self.advance();
        }
        result
    }

    // Lexical analyzer (tokenizer)
    pub fn get_next_token(&mut self) -> Result<Token, RuntimeError> {
        while self.current_char.is_some() {
            let ch = self.current_char.unwrap();
            if ch.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if ch == '{' {
                self.skip_comment();
                continue;
            }

            if ch.is_digit(10) {
                return Ok(self.get_number());
            }

            let entry = self.reserved_symbols.get(&ch); 
            if entry.is_some() {
                let result = entry.unwrap().to_owned();
                self.advance();
                return Ok(Token::new(result, Literal::from_str(ch.to_string()), self.line, self.col));
            } 
            
            if ch.is_alphabetic() {
                let identifier = self.get_identifier();
                let entry = self.reserved_keywords.get(&identifier); 
                let token_type = entry.map(|x| x.to_owned()).unwrap_or(TokenType::Identifier);
                return Ok(Token::new(token_type, Literal::from_str(identifier), self.line, self.col));   
            }
            return Err(RuntimeError::UnexpectedChar(ch));
        }
        Ok(Token::new(TokenType::EOF, Literal::from_str(String::new()), self.line, self.col))
    }

    pub fn get_current_char(&self) -> Option<char> {
        self.current_char
    }

    fn init_reserved_keywords() -> HashMap<String, TokenType> {
        let mut reserved_keywords :  HashMap<String, TokenType> = HashMap::new();
        reserved_keywords.insert(String::from("BEGIN"), TokenType::Begin);
        reserved_keywords.insert(String::from("END"), TokenType::End);
        reserved_keywords.insert(String::from("PROGRAM"), TokenType::Program);
        reserved_keywords.insert(String::from("VAR"), TokenType::Var);
        reserved_keywords.insert(String::from("DIV"), TokenType::IntegerDivision);
        reserved_keywords.insert(String::from("INTEGER"), TokenType::IntegerType);
        reserved_keywords.insert(String::from("REAL"), TokenType::FloatType);
        reserved_keywords.insert(String::from("PROCEDURE"), TokenType::Procedure);
        
        reserved_keywords
    }  

    fn init_reserved_symbols() -> HashMap<char, TokenType> {
        let mut reserved_symbols :  HashMap<char, TokenType> = HashMap::new();
        reserved_symbols.insert('+', TokenType::Plus);
        reserved_symbols.insert('-', TokenType::Minus);
        reserved_symbols.insert('*', TokenType::Multiply);
        reserved_symbols.insert('/', TokenType::Division);
        reserved_symbols.insert('%', TokenType::Modulus);
        reserved_symbols.insert('=', TokenType::Assignment);
        reserved_symbols.insert('(', TokenType::Lparen);
        reserved_symbols.insert(')', TokenType::Rparen);
        reserved_symbols.insert(';', TokenType::Semi);
        reserved_symbols.insert('.', TokenType::Dot);
        reserved_symbols.insert(':', TokenType::Colon);
        reserved_symbols.insert(',', TokenType::Comma);
        
        reserved_symbols
    }
}