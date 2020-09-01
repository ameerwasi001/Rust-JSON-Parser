use std::collections::HashMap;
use crate::token;
use crate::error;
use crate::tokenTypes::TokenTypes::*;
use crate::nodes;
use crate::position;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<token::Token>,
    pub index: usize
}

impl Parser {
    fn eof_token(&self) -> token::Token {
        token::Token{
            tok_type: Eof,
            tok_value: String::from(""),
            pos_start: position::Position{filename: String::from("Yoi"), ftext: String::from(""), index: 0u64, ln: 1u64, cn: 1u64},
            pos_end: position::Position{filename: String::from("Yoi"), ftext: String::from(""), index: 0u64, ln: 1u64, cn: 1u64}
        }
    }

    fn current_tok(&mut self) -> token::Token {
        match self.tokens.get(self.index) {
            None => self.eof_token(),
            Some(_) => self.tokens.remove(self.index)
        }
    }

    pub fn parse(&mut self) -> Result<nodes::Node, error::Error> {
        let res = self.models(None);
        match res {
            Ok(obj) => {
                let (pos_start, pos_end) = obj.get_pos();
                if self.current_tok().tok_type != Eof {
                    return Err(
                        error::Error{
                            name: String::from("ParseError"),
                            message: String::from(format!("Inappropriate ending")),
                            pos_start,
                            pos_end
                        }
                    )
                } else {
                    return Ok(obj)
                }
            },
            Err(err) => Err(err)
        }
    }

    fn sequence<T>(
        &mut self,
        elem: fn(&mut Parser) -> Result<T, error::Error>,
        create_sequence: fn(Vec<T>, position::Position, position::Position) -> nodes::Node,
        tok: Option<token::Token>,
        should_start: fn(&token::Token) -> bool,
        should_end: fn(&token::Token) -> bool,
        is_sep: fn(&token::Token) -> bool
    ) -> Result<nodes::Node, error::Error> {
        let mut elems : Vec<T> = Vec::new();
        let tok = match tok {
            Some(t) => t,
            None => self.current_tok()
        };
        if !(should_start(&tok)) {
            return Err(
                error::Error{
                    name: String::from("ParseError"),
                    message: String::from("Inappropriate starting"),
                    pos_start: tok.pos_start.copy(),
                    pos_end: tok.pos_end.copy()
                }
            )
        }
        match elem(self) {
            Ok(elem) => elems.push(elem),
            Err(err) => return Err(err)
        };
        let mut sep = self.current_tok();
        while is_sep(&sep) {
            match elem(self) {
                Ok(elem) => elems.push(elem),
                Err(err) => return Err(err)
            };
            sep = self.current_tok();
        }
        if !(should_end(&sep)) {
            return Err(
                error::Error{
                    name: String::from("ParseError"),
                    message: String::from(format!("Inappropriate ending")),
                    pos_start: tok.pos_start.copy(),
                    pos_end: tok.pos_end.copy()
                }
            )
        }
        return Ok(create_sequence(elems, tok.pos_start, sep.pos_end))
    }

    fn models(&mut self, tok: Option<token::Token>) -> Result<nodes::Node, error::Error> {
        fn create_object(pairs: Vec<(String, nodes::Node)>, pos_start: position::Position, pos_end: position::Position) -> nodes::Node {
            let map: HashMap<String, nodes::Node> = pairs.into_iter().collect();
            return nodes::Node::ObjectNode{pairs: map, pos_start, pos_end}
        }

        return self.sequence(
            |this: &mut Parser| {this.model()},
            create_object,
            tok,
            |t: &token::Token| t.tok_type == RCurly,
            |t: &token::Token| t.tok_type == LCurly,
            |t: &token::Token| t.tok_type == Comma
        )
    }

    fn array(&mut self, tok: Option<token::Token>) -> Result<nodes::Node, error::Error> {
        return self.sequence(
            |this: &mut Parser| this.atom(),
            |array: Vec<nodes::Node>, pos_start: position::Position, pos_end: position::Position| nodes::Node::ArrayNode{array, pos_start, pos_end},
            tok,
            |t: &token::Token| t.tok_type == Larray,
            |t: &token::Token| t.tok_type == Rarray,
            |t: &token::Token| t.tok_type == Comma
        )
    }

    fn model(&mut self) -> Result<(String, nodes::Node), error::Error> {
        let key = self.current_tok();
        if key.tok_type != Str {
            return Err(
                error::Error {
                    name: String::from("ParseError"),
                    message: String::from("Key must be a string"),
                    pos_start: key.pos_start.copy(),
                    pos_end: key.pos_end.copy()
                }
            )
        };
        let colon = self.current_tok();
        if colon.tok_type != Colon {
            return Err(
                error::Error {
                    name: String::from("ParseError"),
                    message: String::from("Expected colon"),
                    pos_start: key.pos_start.copy(),
                    pos_end: key.pos_end.copy()
                }
            )
        };
        let value = self.atom();
        return match value {
            Ok(a) => Ok((key.tok_value, a)),
            Err(e) => return Err(e)
        }
    }

    fn atom(&mut self) -> Result<nodes::Node, error::Error> {
        let tok = self.current_tok();
        let value = match tok {
            _ if tok.matches(Keyword, "null") => {
                let value = nodes::Node::NullNode{pos_start: tok.pos_start, pos_end: tok.pos_end};
                Ok(value)
            },
            _ if tok.matches(Keyword, "false") => {
                let value = nodes::Node::BoolNode{boolean: false, pos_start: tok.pos_start, pos_end: tok.pos_end};
                Ok(value)
            },
            _ if tok.matches(Keyword, "true") => {
                let value = nodes::Node::BoolNode{boolean: true, pos_start: tok.pos_start, pos_end: tok.pos_end};
                Ok(value)
            },
            _ if tok.tok_type == RCurly => {
                match self.models(Some(tok)) {
                    Ok(a) => Ok(a),
                    Err(e) => return Err(e)
                }
            },
            _ if tok.tok_type == Larray => {
                match self.array(Some(tok)) {
                    Ok(a) => Ok(a),
                    Err(e) => return Err(e)
                }
            },
            _ if tok.tok_type == Int => {
                let value = nodes::Node::IntNode{int: tok.tok_value.parse::<i128>().unwrap(), pos_start: tok.pos_start, pos_end: tok.pos_end};
                Ok(value)
            },
            _ if tok.tok_type == Float => {
                let value = nodes::Node::FloatNode{float: tok.tok_value.parse::<f64>().unwrap(), pos_start: tok.pos_start, pos_end: tok.pos_end};
                Ok(value)
            },
            _ if tok.tok_type == Str => {
                let value = nodes::Node::StrNode{string: tok.tok_value, pos_start: tok.pos_start, pos_end: tok.pos_end};
                Ok(value)
            },
            _ => {
                return Err(
                    error::Error {
                        name: String::from("ParseError"),
                        message: String::from("Expected number, true, object, or false"),
                        pos_start: tok.pos_start.copy(),
                        pos_end: tok.pos_end.copy()
                    }
                )
            }
        };
        return value
    }
}
