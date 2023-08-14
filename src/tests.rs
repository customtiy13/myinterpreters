use super::*;

#[test]
fn test_token_to_str() {
    let result = Token {
        token_type: TokenType::PLUS,
        lexeme: "+".to_string(),
        literal: None,
        line: 2,
    };
    assert_eq!(result.to_string(), "PLUS + None");
}

#[test]
fn test_add_single_token() {
    use TokenType::*;
    let scanner = Scanner::new("(*");
    scanner.scan_tokens().unwrap();
    let result = &[
        Token {
            token_type: LeftParen,
            lexeme: "(".to_string(),
            literal: None,
            line: 1,
        },
        Token {
            token_type: STAR,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: None,
            line: 1,
        },
    ];
    assert_eq!(scanner.tokens.take(), result);
}

#[test]
fn test_add_two_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("!=");
    scanner.scan_tokens().unwrap();
    let result = &[
        Token {
            token_type: BangEqual,
            lexeme: "!=".to_string(),
            literal: None,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: None,
            line: 1,
        },
    ];
    assert_eq!(scanner.tokens.take(), result);
}

#[test]
fn test_comments_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("!=/(//klasjdlfkjasldkfaslkjdf()");
    scanner.scan_tokens().unwrap();
    let result = &[
        Token {
            token_type: BangEqual,
            lexeme: "!=".to_string(),
            literal: None,
            line: 1,
        },
        Token {
            token_type: SLASH,
            lexeme: "/".to_string(),
            literal: None,
            line: 1,
        },
        Token {
            token_type: LeftParen,
            lexeme: "(".to_string(),
            literal: None,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: None,
            line: 1,
        },
    ];
    assert_eq!(scanner.tokens.take(), result);
}

#[test]
fn test_string_literal_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("\"asdf\"");
    scanner.scan_tokens().unwrap();
    let result = &[
        Token {
            token_type: STRING,
            lexeme: "\"asdf\"".to_string(),
            literal: Some("asdf".to_string()),
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: None,
            line: 1,
        },
    ];
    assert_eq!(scanner.tokens.take(), result);
}

#[test]
fn test_number_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("123.53//asdf");
    scanner.scan_tokens().unwrap();
    let result = &[
        Token {
            token_type: NUMBER,
            lexeme: "123.53".to_string(),
            literal: Some("123.53".to_string()),
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: None,
            line: 1,
        },
    ];
    assert_eq!(scanner.tokens.take(), result);
}

#[test]
fn test_identifier_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("asdf98");
    scanner.scan_tokens().unwrap();
    let result = &[
        Token {
            token_type: IDENTIFIER,
            lexeme: "asdf98".to_string(),
            literal: None,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: None,
            line: 1,
        },
    ];
    assert_eq!(scanner.tokens.take(), result);
}

#[test]
fn test_reserved_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("class//asdfjlasdjf");
    scanner.scan_tokens().unwrap();
    let result = &[
        Token {
            token_type: CLASS,
            lexeme: "class".to_string(),
            literal: None,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: None,
            line: 1,
        },
    ];
    assert_eq!(scanner.tokens.take(), result);
}
