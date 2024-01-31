#[derive(Clone, Debug, Default)]
pub struct Token<'a> {
    /// The dot-separated parts of a token, e.g. `push.1` is split into `['push', '1']`.
    pub parts: Vec<&'a str>,
}

impl<'a> Token<'a> {
    pub fn new(parts: &'a str) -> Self {
        Self {
            parts: parts.split('.').collect(),
        }
    }
    pub fn parts(&self) -> &[&str] {
        &self.parts
    }

    pub fn num_parts(&self) -> usize {
        self.parts.len()
    }
}
