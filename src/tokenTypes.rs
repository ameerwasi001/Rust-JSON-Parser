#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenTypes {
    Str,
    Int,
    Keyword,
    Float,
    Larray,
    Comma,
    Rarray,
    LCurly,
    RCurly,
    Colon,
    Eof
}

impl std::fmt::Display for TokenTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
