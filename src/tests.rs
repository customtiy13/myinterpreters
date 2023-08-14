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

#[test]
fn test_add_single_token() {
    use TokenType::*;
    let mut scanner = Scanner::new("(*");
    let tokens = scanner.scan_tokens().unwrap();
    let result = &[
        Token {
            token_type: LeftParen,
            lexeme: "(".to_string(),
            literal: "".to_string(),
            line: 1,
        },
        Token {
            token_type: STAR,
            lexeme: "*".to_string(),
            literal: "".to_string(),
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: 1,
        },
    ];
    assert_eq!(tokens, result);
}

#[test]
fn test_add_two_tokens() {
    use TokenType::*;
    let mut scanner = Scanner::new("!=");
    let tokens = scanner.scan_tokens().unwrap();
    let result = &[
        Token {
            token_type: BangEqual,
            lexeme: "!=".to_string(),
            literal: "".to_string(),
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: 1,
        },
    ];
    assert_eq!(tokens, result);
}
