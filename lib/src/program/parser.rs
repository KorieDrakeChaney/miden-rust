use crate::{parse, tokenize, MidenProgram};

impl MidenProgram {
    pub fn parse(masm: &str) -> Result<Self, String> {
        let tokens = tokenize(masm);
        println!("{:?}", tokens);
        parse(tokens)
    }
}
