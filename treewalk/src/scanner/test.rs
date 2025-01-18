use super::*;

#[test]
fn test_simple_lexer() {
    let sample = "> << / //  this is a comment\n> / +";
    let scanner = Scanner::new(sample);
    let x = scanner.collect::<Vec<_>>();
    use TokenType::*;
    assert_eq!(
        x,
        vec![
            Token {
                token: GREATER,
                lexeme: ">",
                line: 0
            },
            Token {
                token: LESS,
                lexeme: "<",
                line: 0
            },
            Token {
                token: LESS,
                lexeme: "<",
                line: 0
            },
            Token {
                token: SLASH,
                lexeme: "/",
                line: 0
            },
            Token {
                token: GREATER,
                lexeme: ">",
                line: 1
            },
            Token {
                token: SLASH,
                lexeme: "/",
                line: 1
            },
            Token {
                token: PLUS,
                lexeme: "+",
                line: 1
            }
        ]
    );
}


#[test]
fn test_string_literal_with_next() {
    let source = "\"hello world\"";
    let mut scanner = Scanner::new(source);
    let token = scanner.next().unwrap();
    assert_eq!(token.token, TokenType::STRING("hello world"));
    assert!(scanner.next().is_none());
}

#[test]
fn test_string_literal_with_collect() {
    let sample = " > \"multiline\nstring\nliteral\" > ";
    let scanner = Scanner::new(sample);
    let x = scanner.collect::<Vec<_>>();
    use TokenType::*;
    assert_eq!(
        x,
        vec![
            Token {
                token: GREATER,
                lexeme: ">",
                line: 0
            },
            Token {
                token: STRING("multiline\nstring\nliteral"),
                lexeme: "multiline\nstring\nliteral",
                line: 2
            },
            Token {
                token: GREATER,
                lexeme: ">",
                line: 2
            }
        ]
    );
}

#[test]
fn test_number_literal_with_next() {
    let source = "123 45.67";
    let mut scanner = Scanner::new(source);
    let expected_tokens = vec![
        TokenType::NUMBER(123.0),
        TokenType::NUMBER(45.67),
    ];
    for expected in expected_tokens {
        let token = scanner.next().unwrap();
        assert_eq!(token.token, expected);
    }
    assert!(scanner.next().is_none());
}

#[test]
fn test_number_literal_with_collect() {
    let sample = " = 5 35.3 0.32.33 -4 =0.0 .123 123. 0. =";
    let scanner = Scanner::new(sample);
    let x = scanner.collect::<Vec<_>>();
    use TokenType::*;
    assert_eq!(
        x,
        vec![
            Token {
                token: EQUAL,
                lexeme: "=",
                line: 0
            },
            Token {
                token: NUMBER(5.0),
                lexeme: "5",
                line: 0
            },
            Token {
                token: NUMBER(35.3),
                lexeme: "35.3",
                line: 0
            },
            Token {
                token: NUMBER(0.32),
                lexeme: "0.32",
                line: 0
            },
            Token {
                token: DOT,
                lexeme: ".",
                line: 0
            },
            Token {
                token: NUMBER(33.0),
                lexeme: "33",
                line: 0
            },
            Token {
                token: MINUS,
                lexeme: "-",
                line: 0
            },
            Token {
                token: NUMBER(4.0),
                lexeme: "4",
                line: 0
            },
            Token {
                token: EQUAL,
                lexeme: "=",
                line: 0
            },
            // The numbers 0.0 is supported so should show up as a number
            Token {
                token: NUMBER(0.0),
                lexeme: "0.0",
                line: 0
            },
            // The numbers .123 is supported so should show up as a dot and a number
            Token {
                token: DOT,
                lexeme: ".",
                line: 0
            },
            Token {
                token: NUMBER(123.0),
                lexeme: "123",
                line: 0
            },
            // The numbers 123. is not supported so should show up as a number and a dot
            Token {
                token: NUMBER(123.0),
                lexeme: "123",
                line: 0
            },
            Token {
                token: DOT,
                lexeme: ".",
                line: 0
            },
            // The numbers 0. is not supported so should show up as a number and a dot
            Token {
                token: NUMBER(0.0),
                lexeme: "0",
                line: 0
            },
            Token {
                token: DOT,
                lexeme: ".",
                line: 0
            },
            Token {
                token: EQUAL,
                lexeme: "=",
                line: 0
            },
        ]
    );
}

#[test]
fn test_keyword_ident() {
    let sample = "var x = 5;\n var y = -x;";
    let scanner = Scanner::new(sample);
    let x = scanner.collect::<Vec<_>>();
    use TokenType::*;
    assert_eq!(
        x,
        vec![
            Token {
                token: VAR,
                lexeme: "var",
                line: 0
            },
            Token {
                token: IDENTIFIER("x"),
                lexeme: "x",
                line: 0
            },
            Token {
                token: EQUAL,
                lexeme: "=",
                line: 0
            },
            Token {
                token: NUMBER(5.0),
                lexeme: "5",
                line: 0
            },
            Token {
                token: SEMICOLON,
                lexeme: ";",
                line: 0
            },
            Token {
                token: VAR,
                lexeme: "var",
                line: 1
            },
            Token {
                token: IDENTIFIER("y"),
                lexeme: "y",
                line: 1
            },
            Token {
                token: EQUAL,
                lexeme: "=",
                line: 1
            },
            Token {
                token: MINUS,
                lexeme: "-",
                line: 1
            },
            Token {
                token: IDENTIFIER("x"),
                lexeme: "x",
                line: 1
            },
            Token {
                token: SEMICOLON,
                lexeme: ";",
                line: 1
            }
        ]
    );
}

// Add after existing tests

#[test]
fn test_empty_input() {
    let sample = "";
    let scanner = Scanner::new(sample);
    let tokens = scanner.collect::<Vec<_>>();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn test_whitespace_combinations() {
    let sample = "var\tx\r\ny\n\n=\t5";
    let scanner = Scanner::new(sample);
    let x = scanner.collect::<Vec<_>>();
    use TokenType::*;
    assert_eq!(
        x,
        vec![
            Token {
                token: VAR,
                lexeme: "var",
                line: 0
            },
            Token {
                token: IDENTIFIER("x"),
                lexeme: "x",
                line: 0
            },
            Token {
                token: IDENTIFIER("y"),
                lexeme: "y",
                line: 1
            },
            Token {
                token: EQUAL,
                lexeme: "=",
                line: 3
            },
            Token {
                token: NUMBER(5.0),
                lexeme: "5",
                line: 3
            },
        ]
    );
}



#[test]
#[should_panic(expected = "Unterminated string")]
fn test_unterminated_string() {
    let sample = "\"this string never ends";
    let scanner = Scanner::new(sample);
    let _tokens = scanner.collect::<Vec<_>>();
}

#[test]
#[should_panic(expected = "Unexpected input")]
fn test_invalid_character() {
    let sample = "var x = @";
    let scanner = Scanner::new(sample);
    let _tokens = scanner.collect::<Vec<_>>();
}

#[test]
fn test_two_character_tokens() {
    let source = "!= == <= >= ! = < >";
    let mut scanner = Scanner::new(source);
    let expected_tokens = vec![
        TokenType::BANG_EQUAL,
        TokenType::EQUAL_EQUAL,
        TokenType::LESS_EQUAL,
        TokenType::GREATER_EQUAL,
        TokenType::BANG,
        TokenType::EQUAL,
        TokenType::LESS,
        TokenType::GREATER,
    ];
    for expected in expected_tokens {
        let token = scanner.next().unwrap();
        assert_eq!(token.token, expected);
    }
    assert!(scanner.next().is_none());
}





#[test]
fn test_keywords() {
    let source = "and class else false for fun if nil or print return super this true var while";
    let mut scanner = Scanner::new(source);
    let expected_tokens = vec![
        TokenType::AND,
        TokenType::CLASS,
        TokenType::ELSE,
        TokenType::FALSE,
        TokenType::FOR,
        TokenType::FUN,
        TokenType::IF,
        TokenType::NIL,
        TokenType::OR,
        TokenType::PRINT,
        TokenType::RETURN,
        TokenType::SUPER,
        TokenType::THIS,
        TokenType::TRUE,
        TokenType::VAR,
        TokenType::WHILE,
    ];
    for expected in expected_tokens {
        let token = scanner.next().unwrap();
        assert_eq!(token.token, expected);
    }
    assert!(scanner.next().is_none());
}

#[test]
fn test_identifiers() {
    let source = "foo bar _baz qux123";
    let mut scanner = Scanner::new(source);
    let expected_tokens = vec![
        TokenType::IDENTIFIER("foo"),
        TokenType::IDENTIFIER("bar"),
        TokenType::IDENTIFIER("_baz"),
        TokenType::IDENTIFIER("qux123"),
    ];
    for expected in expected_tokens {
        let token = scanner.next().unwrap();
        assert_eq!(token.token, expected);
    }
    assert!(scanner.next().is_none());
}

#[test]
fn test_comments() {
    let source = "// this is a comment\n123 // another comment";
    let mut scanner = Scanner::new(source);
    let token = scanner.next().unwrap();
    assert_eq!(token.token, TokenType::NUMBER(123.0));
    assert!(scanner.next().is_none());
}
