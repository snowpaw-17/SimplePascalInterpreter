use std::collections::HashMap;

use crate::pascal_interpreter::{error::RuntimeError, literal::Literal, token::*};

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize,
    current_char: Option<char>,
    line: u32,
    col: u32,
    reserved_keywords: HashMap<&'static str, TokenType>,
    reserved_symbols: HashMap<char, TokenType>,
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
            reserved_symbols: Lexer::init_reserved_symbols(),
        }
    }

     // Lexical analyzer (tokenizer)
    pub fn get_next_token(&mut self) -> Result<Token, RuntimeError> {
        while let Some(ch) = self.current_char {
            match ch {
                _ if ch.is_whitespace() => self.skip_whitespace(),
                '{' => self.skip_comment(),
                _ if self.reserved_symbols.contains_key(&ch) => return Ok(self.get_reserved_symbol_token()),
                _ if ch.is_alphabetic() => return Ok(self.get_reserved_keyword_or_identifier()),
                _ if ch.is_digit(10) => return Ok(self.get_number()),
                _ => return Err(RuntimeError::UnexpectedChar(ch)),
            }
        }
        Ok(Token::new(TokenType::EOF, Literal::from_str(String::new()), self.line, self.col))
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
            self.col += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(_) = self.current_char.filter(|c| c.is_whitespace()){
            self.advance();
        }
    }

    fn skip_comment(&mut self) {
        while let Some(_) = self.current_char.filter(|&c| c != '}') {
            self.advance();
        }
        self.advance();
    }

    /// Return an integer consumed from input
    fn get_number(&mut self) -> Token {
        let (line, col) = (self.line, self.col);
        let start_pos = self.pos;
        while let Some(_) = self.current_char.filter(|c| c.is_digit(10)) {
            self.advance();
        }

        if let Some('.') = self.current_char {
            self.advance();

            while let Some(_) = self.current_char.filter(|c| c.is_digit(10)) {
                self.advance();
            }
            let num_as_str = &self.text[start_pos..self.pos];
            let value = Literal::from_float(num_as_str.parse::<f64>().unwrap());
            return Token::new(TokenType::FloatConst, value, line, col);
        }

        let num_as_str = &self.text[start_pos..self.pos];
        let value = Literal::from_int(num_as_str.parse::<i64>().unwrap());
        Token::new(TokenType::IntegerConst, value, self.line, self.col)
    }

    fn get_identifier(&mut self) -> &'a str {
        let start_pos = self.pos;
        while let Some(_) = self.current_char.filter(|c| c.is_alphanumeric() || c == &'_')
        {
            self.advance();
        }
        return &self.text[start_pos..self.pos];
    }

    pub fn get_current_char(&self) -> Option<char> {
        self.current_char
    }

     fn get_reserved_symbol_token(&mut self) -> Token {
        let (line, col) = (self.line, self.col);
        let token_type = self.reserved_symbols[&self.current_char.unwrap()];
        self.advance();
        Token::new(token_type, Literal::from_str(String::new()), line, col)
    }

    fn get_reserved_keyword_or_identifier(&mut self) -> Token {
        let (line, col) = (self.line, self.col);
        let id = self.get_identifier();
        let (id, token_type) = self.reserved_keywords
                            .get_key_value(id.to_uppercase().as_str())
                            .unwrap_or((&id, &TokenType::Identifier));
        Token::new(*token_type, Literal::from_str(id.to_string()), line, col)
    }
    
    fn init_reserved_keywords() -> HashMap<&'static str, TokenType> {
        let mut reserved_keywords: HashMap<&str, TokenType> = HashMap::new();
        reserved_keywords.insert("BEGIN", TokenType::Begin);
        reserved_keywords.insert("END", TokenType::End);
        reserved_keywords.insert("PROGRAM", TokenType::Program);
        reserved_keywords.insert("VAR", TokenType::Var);
        reserved_keywords.insert("DIV", TokenType::IntegerDivision);
        reserved_keywords.insert("INTEGER", TokenType::IntegerType);
        reserved_keywords.insert("REAL", TokenType::FloatType);
        reserved_keywords.insert("PROCEDURE", TokenType::Procedure);
        reserved_keywords
    }

    fn init_reserved_symbols() -> HashMap<char, TokenType> {
        let mut reserved_symbols: HashMap<char, TokenType> = HashMap::new();
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
