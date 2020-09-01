mod position;
mod token;
mod lexer;
mod tokenTypes;
mod parser;
mod error;
mod nodes;
use tokenTypes::TokenTypes::*;

fn main() {
    let input =
        String::from(
            "
                {
                    \"a\": true,
                    \"x\": {\"arr\": [0.78, 11]},
                    \"b\": 1,
                    \"d\": \"Ameer\"
                }
            "
        );
    let position = position::Position{filename: String::from("Yoi"), ftext: input, index: 0u64, ln: 1u64, cn: 1u64};
    let mut lexer = lexer::Lexer{current_index: 0usize, chars: position.ftext.as_bytes().to_vec(), position: position};
    let toks = match lexer.lex() {
        Ok(a) => a,
        Err(e) => panic!(format!("{}", e))
    };

    let mut parser = parser::Parser{tokens: toks, index: 0usize};
    let _ast = match parser.parse() {
        Ok(a) => println!("{}", a),
        Err(e) => panic!(format!("{}", e))
    };
}
