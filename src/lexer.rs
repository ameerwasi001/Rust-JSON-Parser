use crate::position;
use crate::token;
use crate::error;
use crate::tokenTypes::TokenTypes::*;
use crate::tokenTypes;

#[derive(Debug)]
pub struct Lexer {
    pub current_index: usize,
    pub chars: Vec<u8>,
    pub position: position::Position
}

impl Lexer {
    fn get_byte(&self) -> Option<&u8> {
        return self.chars.get(self.current_index)
    }

    fn get_char(&self) -> char {
        let byte = if self.get_byte() == None { '\0' } else { self.chars[self.current_index] as char };
        return byte as char
    }

    fn advance(&mut self) {
        self.current_index += 1usize;
        self.position.advance(self.get_char());
    }

    fn is_num(&self) -> bool {
        let current_char = self.get_char();
        return current_char != '\0' && current_char.to_string().parse::<i64>().is_ok() || current_char == '.';
    }

    fn make_number(&mut self) -> token::Token {
        let pos_start = self.position.copy();
        let mut current_char = self.get_char();
        let mut number_str = String::from("");
        let mut dot_count = 0;
        let mut e_count = 0;
        while self.is_num() {
            if current_char == '.' {dot_count+=1};
            if current_char == 'e' {e_count+=1};
            if dot_count == 2 {break};
            if e_count == 2 {break};
            if dot_count == 1 && e_count == 1 {break};
            number_str.push_str(&current_char.to_string());
            self.advance();
            current_char = self.get_char();
        }
        return token::Token{tok_type:
            if dot_count == 0 && e_count == 0 {Int}
            else {Float},
            tok_value: number_str, pos_start: pos_start, pos_end: self.position.copy()}
    }

    fn is_space(&self) -> bool {
        return self.get_char() == ' ' || self.get_char() == '\t' || self.get_char() == '\r' || self.get_char() == '\n';
    }

    fn is_char(&self, chararater: char) -> bool {
        return self.get_char() == chararater;
    }

    fn make_token(&mut self, tok: tokenTypes::TokenTypes) -> token::Token {
        let pos_start = self.position.copy();
        self.advance();
        return token::Token{tok_type: tok, tok_value: String::from(""), pos_start: pos_start, pos_end: self.position.copy()};
    }

    fn make_string(&mut self) -> token::Token {
        let pos_start = self.position.copy();
        let mut chars = String::from("");
        self.advance();
        let mut current_char = self.get_char();
        while !(self.is_char('\0') || self.is_char('"')) {
            chars.push_str(&format!("{}", &current_char));
            self.advance();
            current_char = self.get_char();
        }
        self.advance();
        return token::Token{tok_type: Str, tok_value: chars, pos_start: pos_start, pos_end: self.position.copy()};
    }

    fn is_ident(&self) -> bool {
        let alphabets = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', '_',
            'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
        ];
        return alphabets.contains(&self.get_char())
    }

    fn make_ident(&mut self) -> token::Token {
        let pos_start = self.position.copy();
        let mut chars = String::from(format!("{}", self.get_char()));
        self.advance();
        let mut current_char = self.get_char();
        while self.is_ident() || self.is_num() {
            chars.push_str(&format!("{}", &current_char));
            self.advance();
            current_char = self.get_char();
        }
        return token::Token{tok_type: Keyword, tok_value: chars, pos_start: pos_start, pos_end: self.position.copy()};
    }

    pub fn lex(&mut self) -> Result<Vec<token::Token>, error::Error> {
        let mut tokens: Vec<token::Token> = Vec::new();
        while self.get_char() != '\0' {
            let current_char = self.get_char();
            match current_char {
                _ if self.is_space() => self.advance(),
                _ if self.is_num() => tokens.push(self.make_number()),
                _ if self.is_ident() => tokens.push(self.make_ident()),
                _ if self.is_char('"') => tokens.push(self.make_string()),
                _ if self.is_char('[') => tokens.push(self.make_token(Larray)),
                _ if self.is_char(']') => tokens.push(self.make_token(Rarray)),
                _ if self.is_char('{') => tokens.push(self.make_token(RCurly)),
                _ if self.is_char('}') => tokens.push(self.make_token(LCurly)),
                _ if self.is_char(',') => tokens.push(self.make_token(Comma)),
                _ if self.is_char(':') => tokens.push(self.make_token(Colon)),
                _ => {
                    let pos_start = self.position.copy();
                    self.advance();
                    return Err(error::Error{name: String::from("IllegalCharError"), message: format!("Illegal Chararater '{}'", current_char), pos_start: pos_start, pos_end: self.position.copy()})
                }
            }
        }
        return Ok(tokens)
    }
}
