use std::collections::HashMap;
use crate::position;

#[derive(Debug)]
pub enum Node {
    ObjectNode{pairs: HashMap<String, Node>, pos_start: position::Position, pos_end: position::Position},
    NullNode{pos_start: position::Position, pos_end: position::Position},
    BoolNode{boolean: bool, pos_start: position::Position, pos_end: position::Position},
    IntNode{int: i128, pos_start: position::Position, pos_end: position::Position},
    FloatNode{float: f64, pos_start: position::Position, pos_end: position::Position},
    StrNode{string: String, pos_start: position::Position, pos_end: position::Position},
    ArrayNode{array: Vec<Node>, pos_start: position::Position, pos_end: position::Position}
}

impl Node {
    pub fn get_pos(&self) -> (position::Position, position::Position) {
        match self {
            Node::ObjectNode{pairs: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::NullNode{pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::BoolNode{boolean: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::IntNode{int: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::FloatNode{float: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::StrNode{string: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy()),
            Node::ArrayNode{array: _, pos_start, pos_end} => (pos_start.copy(), pos_end.copy())
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Node::ObjectNode{pairs, pos_start: _, pos_end: _} => {
                let mut mods = Vec::new();
                for (key, node) in pairs {
                    let m = format!("{}: {}", key, node);
                    mods.push(m);
                };
                write!(f, "{}{}{}", "{", mods.join(", "), "}")
            },

            Node::NullNode{pos_start: _, pos_end: _} => write!(f, "null"),
            Node::BoolNode{boolean, pos_start: _, pos_end: _} => write!(f, "{}", if *boolean {"true"} else {"false"}),
            Node::IntNode{int, pos_start: _, pos_end: _} => write!(f, "{}", int),
            Node::FloatNode{float, pos_start: _, pos_end: _} => write!(f, "{}", float),
            Node::StrNode{string, pos_start: _, pos_end: _} => write!(f, "{}", string),
            Node::ArrayNode{array, pos_start: _, pos_end: _} => {
                write!(f, "{}{}{}", "[", array.iter().map(|n| format!("{}", n)).collect::<Vec<String>>().join(", "), "]")
            }
        }
    }
}
