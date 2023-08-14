#[cfg(test)]
mod tests;

use anyhow::Result;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenType {
    // Single-char tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // one or two char tokens.
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,
    // literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

fn error(line: usize, msg: &str) {
    report(line, "", msg);
}

fn report(line: usize, info: &str, msg: &str) {
    println!("[line {}] Error {}: {}", line, info, msg);
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>, // TODO
    line: usize,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

struct Scanner {
    source: String,
    tokens: RefCell<Vec<Token>>,
    start: RefCell<usize>,
    current: RefCell<usize>,
    line: RefCell<usize>,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new().into(),
            start: 0.into(),
            current: 0.into(),
            line: 1.into(),
        }
    }

    fn scan_tokens(&self) -> Result<()> {
        while !self.is_end() {
            *self.start.borrow_mut() = *self.current.borrow();
            self.scan_token()?;
        }

        self.tokens.borrow_mut().push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: *self.line.borrow(),
        }); //todo
        Ok(())
    }

    fn scan_token(&self) -> Result<()> {
        let c: char = self.advance();
        use TokenType::*;
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(COMMA),
            '.' => self.add_token(DOT),
            '-' => self.add_token(MINUS),
            '+' => self.add_token(PLUS),
            ';' => self.add_token(SEMICOLON),
            '*' => self.add_token(STAR),
            '!' => {
                let token = if self.is_match('=') { BangEqual } else { BANG };
                self.add_token(token);
            }
            '=' => {
                let token = if self.is_match('=') {
                    EqualEqual
                } else {
                    EQUAL
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.is_match('=') { LessEqual } else { LESS };
                self.add_token(token);
            }
            '>' => {
                let token = if self.is_match('=') {
                    GreaterEqual
                } else {
                    GREATER
                };
                self.add_token(token);
            }
            '/' => {
                if self.is_match('/') {
                    // comments
                    let iter = self.source.chars();
                    for c in iter {
                        if c == '\n' || self.is_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    // slash
                    self.add_token(SLASH);
                }
            }
            ' ' | '\r' | '\t' => {
                // ignore
            }
            '\n' => *self.line.borrow_mut() += 1,
            '"' => self.deal_string(),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.deal_number(),

            _ => {
                if Self::is_alpha_underline(self.peek(0)) {
                    self.deal_identifier();
                }
            }
        }
        Ok(())
    }

    fn deal_string(&self) {
        while self.peek(0) != '"' && !self.is_end() {
            if self.peek(0) == '\n' {
                *self.line.borrow_mut() += 1;
            }
            self.advance();
        }

        if self.is_end() {
            todo!();
        }

        // The '"'
        self.advance();

        // trim quotes.
        let value = &self.source[*self.start.borrow() + 1..*self.current.borrow() - 1];

        self.add_literal_token(TokenType::STRING, value);
    }

    fn deal_number(&self) {
        while Self::is_digit(self.peek(0)) {
            self.advance();
        }
        // fractional part.
        if self.peek(0) == '.' && Self::is_digit(self.peek(1)) {
            // consume the "."
            self.advance();
            while Self::is_digit(self.peek(0)) {
                self.advance();
            }
        }

        let value = self.get_current_value();
        self.add_literal_token(TokenType::NUMBER, value)
    }

    fn deal_identifier(&self) {
        while Self::is_alpha_underline_num(self.peek(0)) {
            self.advance();
        }

        self.add_token(TokenType::IDENTIFIER)
    }

    fn get_current_value(&self) -> &str {
        &self.source[*self.start.borrow()..*self.current.borrow()]
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha_underline(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_underline_num(c: char) -> bool {
        Self::is_alpha_underline(c) || Self::is_digit(c)
    }

    fn is_match(&self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }

        let current_char = self.source.chars().nth(*self.current.borrow()).unwrap();
        if current_char != expected {
            return false;
        }

        // ok
        *self.current.borrow_mut() += 1; // maybe TODO. not update current??
        return true;
    }

    fn peek(&self, offset: usize) -> char {
        let nth = *self.current.borrow() + offset;
        if self.is_end() || nth >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(nth).unwrap();
    }

    fn add_literal_token(&self, token_type: TokenType, literal: &str) {
        let text = self.get_current_value();
        self.tokens.borrow_mut().push(Token {
            token_type,
            lexeme: text.to_string(),
            literal: Some(literal.to_string()), // TODO
            line: *self.line.borrow(),
        })
    }

    fn add_token(&self, token_type: TokenType) {
        let text = self.get_current_value();
        self.tokens.borrow_mut().push(Token {
            token_type,
            lexeme: text.to_string(),
            literal: None,
            line: *self.line.borrow(),
        })
    }

    fn advance(&self) -> char {
        let current_char = self.source.chars().nth(*self.current.borrow()).unwrap();
        *self.current.borrow_mut() += 1;
        current_char
    }

    fn is_end(&self) -> bool {
        *self.current.borrow() >= self.source.len()
    }
}
