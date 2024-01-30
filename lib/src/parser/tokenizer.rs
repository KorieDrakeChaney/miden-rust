use lazy_regex::*;

use super::{sanitize, Token};

static TOKENS: Lazy<Regex> = lazy_regex!(r"0x[a-fA-F0-9]+|[0-9]+|[a-zA-Z0-9_]+");

pub fn tokenize(input: &str) -> Vec<Token> {
    let sanitize = sanitize(input);
    let matches: Vec<&str> = TOKENS.find_iter(&sanitize).map(|m| m.as_str()).collect();

    matches
        .iter()
        .map(|&m| {
            if m.starts_with("0x") {
                let hex_str = &m[2..];
                if hex_str.len() == 64 {
                    let mut numbers = Vec::new();

                    for i in 0..4 {
                        let start = i * 16;
                        let end = (i + 1) * 16;
                        let chunk = &hex_str[start..end];
                        let chunk = chunk.chars().collect::<Vec<_>>();
                        let chunk = chunk.chunks(2).rev().flatten().collect::<String>();
                        if let Ok(num) = u64::from_str_radix(&chunk, 16) {
                            numbers.push(Token::Number(num));
                        } else {
                            return vec![Token::String(m.to_string())];
                        }
                    }
                    numbers
                } else if let Ok(m) = u64::from_str_radix(hex_str, 16) {
                    vec![Token::Number(m)]
                } else {
                    vec![Token::String(m.to_string())]
                }
            } else {
                vec![Token::from(m)]
            }
        })
        .flatten()
        .collect()
}
