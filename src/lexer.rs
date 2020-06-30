use super::token::TokenKind;

pub struct Lexer<'a> {
    input: &'a str,
    pos:   usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
        }
    }

    fn eof(&self) -> bool {
        self.input.len() <= self.pos
    }

    fn next_char(&self) -> Result<char, ()> {
        self.input[self.pos..].chars().next().ok_or(())
    }

    fn skip_whitespace(&mut self) -> Result<(), ()> {
        self.consume_while(char::is_whitespace).and(Ok(()))
    }

    fn consume_while<F>(&mut self, f: F) -> Result<String, ()>
        where
            F: Fn(char) -> bool,
    {
        let mut s = String::new();
        while !self.eof() && f(self.next_char()?) {
            s.push(self.consume_char()?);
        }
        Ok(s)
    }

    fn consume_char(&mut self) -> Result<char, ()> {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().ok_or(())?;
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        Ok(cur_char)
    }

    fn next_token(&mut self) -> TokenKind {
        self.skip_whitespace().unwrap();
        match self.consume_char() {
            Ok('+') => TokenKind::PLUS,
            Ok('-') => TokenKind::MINUS,
            Ok('*') => TokenKind::ASTERISK,
            Ok('/') => TokenKind::SLASH,
            Ok('%') => TokenKind::PERCENT,
            Ok('(') => TokenKind::LPAREN,
            Ok(')') => TokenKind::RPAREN,
            Ok(other) =>
                if let Some(num) = other.to_digit(10) {
                    let mut radix = 10;
                    let mut num: f64 = num.into();
                    while let Ok(c) = self.next_char() {
                        if let Some(n) = c.to_digit(radix) {
                            num = num * (radix as f64) + (n as f64);
                            self.consume_char().unwrap();
                        } else if num == 0.0 && c == 'x' {
                            radix = 16;
                            self.consume_char().unwrap();
                        } else {
                            break;
                        }
                    }
                    if let Ok('.') = self.next_char() {
                        self.consume_char().unwrap();
                        let mut fractional = String::new();
                        while let Ok(c) = self.next_char() {
                            if c.is_digit(10) {
                                fractional.push(c);
                                self.consume_char().unwrap();
                            } else {
                                break;
                            }
                        }
                        num = format!("{}.{}", num, fractional).parse::<f64>().unwrap();
                    }
                    TokenKind::NUM(num)
                } else {
                    TokenKind::ILLEGAL(other.to_string())
                },
            Err(_) => TokenKind::EOF,
        }
    }
}

pub fn tokenize(mut l: Lexer) -> Vec<TokenKind> {
    let mut tok = Vec::new();
    while !l.eof() {
        tok.push(l.next_token());
    }
    tok
}

