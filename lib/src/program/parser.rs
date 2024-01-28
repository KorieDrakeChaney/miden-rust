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

    pub fn parse_from_file(file: &str) -> Result<Self, String> {
        let mut program = MidenProgram::new();
        let file = std::fs::read_to_string(file);

        match file {
            Ok(file_string) => {
                let tokens = tokenize(&file_string);

                let (mut operands, procedures) = parse(tokens)?;

                program.add_procs(procedures);

                program.add_operands(&mut operands);

                Ok(program)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn parse_from_file_with_inputs(file: &str, inputs: Inputs) -> Result<Self, String> {
        let mut program = MidenProgram::new().with_inputs(inputs);
        let file = std::fs::read_to_string(file);

        match file {
            Ok(file_string) => {
                let tokens = tokenize(&file_string);

                let (mut operands, procedures) = parse(tokens)?;

                program.add_procs(procedures);

                program.add_operands(&mut operands);

                Ok(program)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
