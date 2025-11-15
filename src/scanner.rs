use crate::scanner::TokenType::{
    And, Bang, BangEqual, Class, Comma, Dot, Else, Eof, Equal, EqualEqual, False, For, Fun,
    Greater, GreaterEqual, Identifier, If, LeftBrace, LeftParen, Less, LessEqual, Minus, Nil,
    Number, Or, Plus, Print, Return, RightBrace, RightParen, Semicolon, Slash, Star, String, Super,
    This, True, Var, While,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: i32,
}

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

pub fn init_scanner(source: &str) -> Scanner<'_> {
    Scanner {
        source,
        start: 0,
        current: 0,
        line: 1,
    }
}

impl<'a> Scanner<'a> {
    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(Eof);
        }

        let c = self.advance();

        if self.is_alpha(c) {
            return self.identifier();
        }

        if self.is_digit(c) {
            return self.number();
        }

        match c {
            '(' => self.make_token(LeftParen),
            ')' => self.make_token(RightParen),
            '{' => self.make_token(LeftBrace),
            '}' => self.make_token(RightBrace),
            ';' => self.make_token(Semicolon),
            '.' => self.make_token(Dot),
            ',' => self.make_token(Comma),
            '-' => self.make_token(Minus),
            '+' => self.make_token(Plus),
            '/' => self.make_token(Slash),
            '*' => self.make_token(Star),
            '!' => {
                let token_type = if self.match_ch('=') { BangEqual } else { Bang };
                self.make_token(token_type)
            }
            '=' => {
                let token_type = if self.match_ch('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.make_token(token_type)
            }
            '<' => {
                let token_type = if self.match_ch('=') { LessEqual } else { Less };
                self.make_token(token_type)
            }
            '>' => {
                let token_type = if self.match_ch('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.make_token(token_type)
            }
            '"' => self.string(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn match_ch(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token<'a> {
        Token {
            token_type,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    fn error_token(&self, message: &'static str) -> Token<'static> {
        Token {
            token_type: TokenType::Error,
            lexeme: message,
            line: self.line,
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();

            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance();
        self.make_token(String)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) -> Token<'a> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(Number)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c > 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
    }

    fn identifier(&mut self) -> Token<'a> {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        // Trie-based keyword recognition
        match self.source.as_bytes()[self.start] {
            b'a' => self.check_keyword(1, "nd", And),
            b'c' => self.check_keyword(1, "lass", Class),
            b'e' => self.check_keyword(1, "lse", Else),
            b'f' => {
                if self.current - self.start > 1 {
                    match self.source.as_bytes()[self.start + 1] {
                        b'a' => self.check_keyword(2, "lse", False),
                        b'o' => self.check_keyword(2, "r", For),
                        b'u' => self.check_keyword(2, "n", Fun),
                        _ => Identifier,
                    }
                } else {
                    Identifier
                }
            }
            b'i' => self.check_keyword(1, "f", If),
            b'n' => self.check_keyword(1, "il", Nil),
            b'o' => self.check_keyword(1, "r", Or),
            b'p' => self.check_keyword(1, "rint", Print),
            b'r' => self.check_keyword(1, "eturn", Return),
            b's' => self.check_keyword(1, "uper", Super),
            b't' => {
                if self.current - self.start > 1 {
                    match self.source.as_bytes()[self.start + 1] {
                        b'h' => self.check_keyword(2, "is", This),
                        b'r' => self.check_keyword(2, "ue", True),
                        _ => Identifier,
                    }
                } else {
                    Identifier
                }
            }
            b'v' => self.check_keyword(1, "ar", Var),
            b'w' => self.check_keyword(1, "hile", While),
            _ => Identifier,
        }
    }

    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
        let length = rest.len();
        if self.current - self.start == start + length
            && &self.source[self.start + start..self.start + start + length] == rest
        {
            token_type
        } else {
            Identifier
        }
    }
}
