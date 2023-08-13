#[derive(Debug, Clone, Copy)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_to_str() {
        let result = Token {
            token_type: TokenType::PLUS,
            lexeme: "+".to_string(),
            literal: "+".to_string(),
            line: 2,
        };
        assert_eq!(result.to_string(), "PLUS + +");
    }
}
