use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::stmt::Stmt;
use crate::tokens::{Token, TokenType, Type};
use anyhow::Result;

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
fn test_parser_primary() -> Result<()> {
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
            token_type: SEMICOLON,
            lexeme: ";".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        Token {
            token_type: EOF,
            lexeme: "".to_string(),
            literal: Type::Nil,
            line: 2,
        },
    ];
    let expected = vec![Stmt::ExprStmt(Binary {
        left: Box::new(Literal(Type::Number("3".parse::<f64>().unwrap()))),
        op: Token {
            token_type: PLUS,
            lexeme: "+".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        right: Box::new(Literal(Type::Number("4".parse::<f64>().unwrap()))),
    })];
    let parser = Parser::new(tokens);
    let result = parser.parse()?;
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_evalute_literal() -> Result<()> {
    use Expr::*;
    let expr = Literal(Type::Number("3".parse::<f64>().unwrap()));
    let expected = Type::Number("3".parse::<f64>().unwrap());

    let interpreter = Interpreter::new();
    let result = interpreter.evaluate_expr(&expr)?;
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_evalute_unary() -> Result<()> {
    use Expr::*;
    use TokenType::*;
    use Type::*;
    let expr = Unary {
        op: Token {
            token_type: MINUS,
            lexeme: "-".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        right: Box::new(Literal(Number(3.0))),
    };
    let expected = Type::Number("-3".parse::<f64>().unwrap());

    let interpreter = Interpreter::new();
    let result = interpreter.evaluate_expr(&expr)?;
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_evalute_grouping() -> Result<()> {
    use Expr::*;
    use TokenType::*;
    use Type::*;
    let expr = Grouping(Box::new(Binary {
        left: Box::new(Literal(Number(1.0))),
        op: Token {
            token_type: PLUS,
            lexeme: "+".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        right: Box::new(Literal(Number(3.0))),
    }));
    let expected = Type::Number("4.0".parse::<f64>().unwrap());

    let interpreter = Interpreter::new();
    let result = interpreter.evaluate_expr(&expr)?;
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_evalute_binary_1() -> Result<()> {
    use Expr::*;
    use TokenType::*;
    use Type::*;
    let expr = Binary {
        left: Box::new(Grouping(Box::new(Binary {
            left: Box::new(Literal(Number(3.0))),
            op: Token {
                token_type: PLUS,
                lexeme: "+".to_string(),
                literal: Type::Nil,
                line: 1,
            },
            right: Box::new(Literal(Number(5.0))),
        }))),
        op: Token {
            token_type: MINUS,
            lexeme: "-".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        right: Box::new(Literal(Number(5.0))),
    };
    let expected = Type::Number("3.0".parse::<f64>().unwrap());

    let interpreter = Interpreter::new();
    let result = interpreter.evaluate_expr(&expr)?;
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_evalute_binary_2() -> Result<()> {
    use Expr::*;
    use TokenType::*;
    use Type::*;
    let expr = Binary {
        left: Box::new(Literal(Number(5.0))),
        op: Token {
            token_type: GREATER,
            lexeme: ">".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        right: Box::new(Literal(Number(3.0))),
    };
    let expected = Type::Bool(true);

    let interpreter = Interpreter::new();
    let result = interpreter.evaluate_expr(&expr)?;
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_evalute_binary_3() -> Result<()> {
    use Expr::*;
    use TokenType::*;
    use Type::*;
    let expr = Binary {
        left: Box::new(Literal(Number(6.0))),
        op: Token {
            token_type: SLASH,
            lexeme: "/".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        right: Box::new(Literal(Number(3.0))),
    };
    let expected = Type::Number(2.0);

    let interpreter = Interpreter::new();
    let result = interpreter.evaluate_expr(&expr)?;
    assert_eq!(result, expected);

    Ok(())
}

#[test]
fn test_evalute_binary_4() -> Result<()> {
    use Expr::*;
    use TokenType::*;
    use Type::*;
    let expr = Binary {
        left: Box::new(Literal(String("asdf".to_string()))),
        op: Token {
            token_type: PLUS,
            lexeme: "+".to_string(),
            literal: Type::Nil,
            line: 1,
        },
        right: Box::new(Literal(String("123".to_string()))),
    };
    let expected = Type::String("asdf123".to_string());

    let interpreter = Interpreter::new();
    let result = interpreter.evaluate_expr(&expr)?;
    assert_eq!(result, expected);

    Ok(())
}
