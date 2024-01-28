use crate::{parse, tokenize, Inputs, MidenProgram};

impl MidenProgram {
    pub fn parse(masm: &str) -> Result<Self, String> {
        let mut program = MidenProgram::new();
        let tokens = tokenize(masm);

        let (mut operands, procedures) = parse(tokens)?;

        program.add_procs(procedures);

        program.add_operands(&mut operands);

        Ok(program)
    }

    pub fn parse_with_inputs(masm: &str, inputs: Inputs) -> Result<Self, String> {
        let mut program = MidenProgram::new().with_inputs(inputs);
        let tokens = tokenize(masm);

        let (mut operands, procedures) = parse(tokens)?;

        program.add_procs(procedures);

        program.add_operands(&mut operands);

        Ok(program)
    }
}
