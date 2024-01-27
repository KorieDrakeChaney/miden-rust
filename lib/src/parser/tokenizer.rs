use lazy_regex::*;

use super::Token;

static TOKENS: Lazy<Regex> = lazy_regex!(r"[0-9]+|[a-zA-Z0-9_]+");

pub fn tokenize(input: &str) -> Vec<Token> {
    TOKENS
        .find_iter(input)
        .map(|m| Token::from(m.as_str()))
        .collect()
}
