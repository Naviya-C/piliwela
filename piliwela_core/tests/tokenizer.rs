use piliwela_core::engine::tokenizer::{
    tokenize,
    Token,
};

#[test]
fn test_simple() {
    assert_eq!(
        tokenize("Python Rust"),
        vec![
            Token::Word("Python"),
            Token::Whitespace(" "),
            Token::Word("Rust"),
        ]
    );
}

#[test]
fn test_multiple_spaces() {
    assert_eq!(
        tokenize("Python   Rust"),
        vec![
            Token::Word("Python"),
            Token::Whitespace("   "),
            Token::Word("Rust"),
        ]
    );
}

#[test]
fn test_empty() {
    assert_eq!(
        tokenize(""),
        Vec::<Token>::new(),
    );
}

#[test]
fn test_whitespace_only() {
    assert_eq!(
        tokenize("   "),
        vec![
            Token::Whitespace("   "),
        ]
    );
}