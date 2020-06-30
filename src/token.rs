#[derive(Debug, PartialEq)]
pub enum TokenKind {
    PLUS,      // +
    MINUS,     // -
    ASTERISK,  // *
    SLASH,     // /
    PERCENT,   // %
    LPAREN,    // (
    RPAREN,    // )
    NUM(f64),  // [0-9][0-9]*.[0-9][0-9]*

    ILLEGAL(String),
    EOF,
}

