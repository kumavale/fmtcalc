//! EBNF
//! expr    = mul ('+' mul | '-' mul)*;
//! mul     = unary ('*' unary | '/' unary | '%' unary)*;
//! unary   = ('+' | '-')? primary;
//! primary = num | '(' expr ')';
//! num     = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';

use super::token::TokenKind;

#[derive(Debug)]
pub enum AstKind {
    ADD,      // +
    SUB,      // -
    MUL,      // *
    DIV,      // /
    REM,      // %
    NUM(f64), // [0-9][0-9]*.[0-9][0-9]*
}

#[derive(Debug)]
pub struct Ast {
    pub kind:  AstKind,
    pub lnode: Option<Box<Ast>>,
    pub rnode: Option<Box<Ast>>,
}

#[derive(Debug)]
struct Tokens<'a> {
    tokens: &'a[TokenKind],
    idx:    usize,
}

impl<'a> Tokens<'a> {
    fn new(tokens: &'a[TokenKind]) -> Self {
        Self {
            tokens,
            idx: 0
        }
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.tokens[self.idx] == kind {
            self.idx += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) {
        if self.tokens[self.idx] == kind {
            self.idx += 1;
        } else {
            panic!("expected {:?}, but got {:?}", kind, self.tokens[self.idx]);
        }
    }

    fn expect_number(&mut self) -> f64 {
        match self.tokens[self.idx] {
            TokenKind::NUM(num) => {
                self.idx += 1;
                num
            },
            _ => panic!("expected NUM(f64). but got {:?}", self.tokens[self.idx]),
        }
    }

    fn check_illegal(&self) {
        if let TokenKind::ILLEGAL(_) = self.tokens[self.idx] {
            panic!("illegal TokenKind {:?}", self.tokens[self.idx]);
        }
    }

    fn eof(&self) -> bool {
        self.tokens.len() <= self.idx
    }
}

pub fn new(tokens: &[TokenKind]) -> Option<Box<Ast>> {
    let mut tokens = Tokens::new(&tokens);
    expr(&mut tokens)
}

fn new_node(kind: AstKind, lnode: Option<Box<Ast>>, rnode: Option<Box<Ast>>) -> Option<Box<Ast>> {
    Some(Box::new(Ast { kind, lnode, rnode }))
}

fn expr(mut tokens: &mut Tokens) -> Option<Box<Ast>> {
    let mut node = mul(&mut tokens);

    while !tokens.eof() {
        tokens.check_illegal();
        if tokens.consume(TokenKind::PLUS) {
            node = new_node(AstKind::ADD, node, mul(&mut tokens));
        } else if tokens.consume(TokenKind::MINUS) {
            node = new_node(AstKind::SUB, node, mul(&mut tokens));
        } else {
            return node;
        }
    }
    node
}

fn mul(mut tokens: &mut Tokens) -> Option<Box<Ast>> {
    let mut node = unary(&mut tokens);

    while !tokens.eof() {
        tokens.check_illegal();
        if tokens.consume(TokenKind::ASTERISK) {
            node = new_node(AstKind::MUL, node, unary(&mut tokens));
        } else if tokens.consume(TokenKind::SLASH) {
            node = new_node(AstKind::DIV, node, unary(&mut tokens));
        } else if tokens.consume(TokenKind::PERCENT) {
            node = new_node(AstKind::REM, node, unary(&mut tokens));
        } else {
            return node;
        }
    }
    node
}

fn unary(mut tokens: &mut Tokens) -> Option<Box<Ast>> {
    if tokens.consume(TokenKind::PLUS) {
        return primary(&mut tokens);
    }
    if tokens.consume(TokenKind::MINUS) {
        return new_node(AstKind::SUB, new_node(AstKind::NUM(0.0), None, None), primary(&mut tokens));
    }
    primary(&mut tokens)
}

fn primary(mut tokens: &mut Tokens) -> Option<Box<Ast>> {
    if tokens.consume(TokenKind::LPAREN) {
        let node = expr(&mut tokens);
        tokens.expect(TokenKind::RPAREN);
        return node;
    }

    match tokens.tokens[tokens.idx] {
        TokenKind::NUM(_) => new_node(AstKind::NUM(tokens.expect_number()), None, None),
        _ => None,
    }
}

