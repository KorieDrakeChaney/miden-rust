use super::Token;

pub fn tokenize<'a>(sanitized: &'a str) -> Vec<Token<'a>> {
    sanitized
        .split_whitespace()
        .map(|word| Token::new(word))
        .collect()
}
