#[cfg(test)]
mod tests;

use anyhow::Result;

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
    literal: String, // TODO
    line: usize,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Result<&[Token]> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        }); //todo
        Ok(&self.tokens)
    }

    fn scan_token(&mut self) -> Result<()> {
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

            _ => todo!(),
        }
        Ok(())
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }

        let current_char = self.source.chars().nth(self.current).unwrap();
        if current_char != expected {
            return false;
        }

        // ok
        self.current += 1;
        return true;
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: "".to_string(), // TODO
            line: self.line,
        })
    }

    fn advance(&mut self) -> char {
        let current_char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        current_char
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
