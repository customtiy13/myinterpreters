use crate::parser::Expr;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::tokens::{Token, TokenType, Type};

#[test]
fn test_token_to_str() {
    let result = Token {
        token_type: TokenType::PLUS,
        lexeme: "+".to_string(),
        literal: Type::Nil,
        line: 2,
    };
    assert_eq!(result.to_string(), "PLUS + Nil");
}

#[test]
fn test_add_single_token() {
    use TokenType::*;
    let scanner = Scanner::new("(*");
    let result = scanner.scan_tokens().unwrap();
    let expected = &[
        Token {
            token_type: LeftParen,
            lexeme: "(".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: STAR,
            lexeme: "*".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: Type::Nil,
            line: 1,
        },
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_add_two_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("!=");
    let result = scanner.scan_tokens().unwrap();
    let expected = &[
        Token {
            token_type: BangEqual,
            lexeme: "!=".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: Type::Nil,
            line: 1,
        },
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_comments_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("!=/(//klasjdlfkjasldkfaslkjdf()");
    let result = scanner.scan_tokens().unwrap();
    let expected = &[
        Token {
            token_type: BangEqual,
            lexeme: "!=".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: SLASH,
            lexeme: "/".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: LeftParen,
            lexeme: "(".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: Type::Nil,
            line: 1,
        },
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_string_literal_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("\"asdf\"");
    let result = scanner.scan_tokens().unwrap();
    let expected = &[
        Token {
            token_type: STRING,
            lexeme: "\"asdf\"".to_string(),
            literal: Type::String("asdf".to_string()),
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: Type::Nil,
            line: 1,
        },
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_number_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("123.53//asdf");
    let result = scanner.scan_tokens().unwrap();
    let expected = &[
        Token {
            token_type: NUMBER,
            lexeme: "123.53".to_string(),
            literal: Type::Number("123.53".parse::<f64>().unwrap()),
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: Type::Nil,
            line: 1,
        },
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_identifier_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("asdf98");
    let result = scanner.scan_tokens().unwrap();
    let expected = &[
        Token {
            token_type: IDENTIFIER,
            lexeme: "asdf98".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: Type::Nil,
            line: 1,
        },
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_reserved_tokens() {
    use TokenType::*;
    let scanner = Scanner::new("class//asdfjlasdjf");
    let result = scanner.scan_tokens().unwrap();
    let expected = &[
        Token {
            token_type: CLASS,
            lexeme: "class".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: Type::Nil,
            line: 1,
        },
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_parser_primary() {
    use Expr::*;
    use TokenType::*;
    let tokens = &[
        Token {
            token_type: NUMBER,
            lexeme: "3".to_string(),
            literal: Type::Number("3".parse::<f64>().unwrap()),
            line: 1,
        },
        Token {
            token_type: PLUS,
            lexeme: "+".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: NUMBER,
            lexeme: "4".to_string(),
            literal: Type::Number("4".parse::<f64>().unwrap()),
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: Type::Nil,
            line: 2,
        },
    ];
    let expected = Binary {
        left: Box::new(Literal(Type::Number("3".parse::<f64>().unwrap()))),
        op: Token {
            token_type: PLUS,
            lexeme: "+".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        right: Box::new(Literal(Type::Number("4".parse::<f64>().unwrap()))),
    };
    let parser = Parser::new(tokens);
    let result = parser.parse();
    assert_eq!(result, expected);
}
