#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Any(Box<Type>),
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn type_helper(t: &Type) -> String {
            match t {
                Type::Nil => "".to_string(),
                Type::Bool(v) => v.to_string(),
                Type::Number(v) => v.to_string(),
                Type::String(v) => v.clone(),
                Type::Any(v) => type_helper(v),
            }
        }

        let result = type_helper(self);
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Type, // TODO
    pub line: usize,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
