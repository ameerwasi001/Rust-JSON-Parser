use crate::tokenTypes;
use crate::position;

#[derive(Debug)]
pub struct Token {
    pub tok_type: tokenTypes::TokenTypes,
    pub tok_value: String,
    pub pos_start: position::Position,
    pub pos_end: position::Position
}

impl Token {
    pub fn matches(&self, name : tokenTypes::TokenTypes, value : &str) -> bool {
        return self.tok_type == name && self.tok_value == *value
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.tok_value == "" {
            write!(f, "{}", &self.tok_type.to_string())
        } else {
            let string =
                if self.tok_value == "" { format!("[{:?}]", self.tok_type) }
                else { format!("[{:?} : {}]", self.tok_type, self.tok_value) };
            write!(f, "{}", &string)
        }
    }
}
