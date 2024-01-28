use lazy_regex::*;

use super::{sanitize, Token};

static TOKENS: Lazy<Regex> = lazy_regex!(r"[0-9]+|[a-zA-Z0-9_]+");

pub fn tokenize(input: &str) -> Vec<Token> {
    let sanitize = sanitize(input);
    TOKENS
        .find_iter(&sanitize)
        .map(|m| Token::from(m.as_str()))
        .collect()
}
