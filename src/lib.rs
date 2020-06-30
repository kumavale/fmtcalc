//! # fmtcalc
//! This is a simple calculator written in Rust.  
//! It is an alternative to printf for shell command.  
//!
//! # Examples
//!
//! ```bash
//! $ fmtcalc 4+2
//! 6
//!
//! $ fmtcalc "1+2*3 = {}" "1+2*3"
//! 1+2*3 = 7
//!
//! $ fmtcalc "0xFF - 654.321"
//! -399.321
//!
//! $ fmtcalc "3²+4²={}, 5²={}" "3*3+4*4" "5*5"
//! 3² +4² =25, 5² =25
//! ```
//!
//! For better ergonomics I often abbreviate `fmtcalc` as `println` in your shell startup files:
//! ```bash
//! alias println='fmtcalc'
//! ```

mod lexer;
mod token;
mod ast;
mod parser;

/// Returns the calculation result.
///
/// # EBNF
///
/// ```text
/// expr    = mul ('+' mul | '-' mul)*;
/// mul     = unary ('*' unary | '/' unary | '%' unary)*;
/// unary   = ('+' | '-')? primary;
/// primary = num | '(' expr ')';
/// num     = (0x)?[0-9]+(.[0-9]+)?;
/// ```
///
/// # Examples
///
/// ```
/// # use fmtcalc::expr;
/// assert_eq!(expr("5+6*7"),   "47");
/// assert_eq!(expr("5*(9-6)"), "15");
/// assert_eq!(expr("(3+5)/2"),  "4");
/// assert_eq!(expr("0.12+3.4"), "3.52");
/// assert_eq!(expr("0x42"),    "66");
/// ```
pub fn expr(input: &str) -> String {
    let l = lexer::Lexer::new(&input);
    let tokens = lexer::tokenize(l);
    let ast = ast::new(&tokens);
    let mut v = vec![];
    parser::gen(&ast, &mut v);
    v.pop().unwrap().to_string()
}

