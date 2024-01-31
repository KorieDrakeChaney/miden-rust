use crate::{parse, tokenize, Inputs, MidenProgram};

impl MidenProgram {
    /// Parses a Miden assembly string into a MidenProgram.
    ///
    /// # Arguments
    ///
    /// * `masm` - A string containing Miden assembly code.
    ///
    /// # Returns
    ///
    /// A Result containing a MidenProgram if the parsing was successful, or a String error message otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram};
    /// let program = MidenProgram::parse("push.5\npush.3\nadd").unwrap();
    /// ```
    pub fn parse(masm: &str) -> Result<Self, String> {
        let mut program = MidenProgram::new();
        let tokens = tokenize(masm);

        let (mut instructions, procedures) = parse(tokens)?;

        program.add_procs(procedures);

        program.add_instructions(&mut instructions);

        Ok(program)
    }

    /// Parses a Miden assembly string into a MidenProgram, with a given set of inputs.
    ///
    /// # Arguments
    ///
    /// * `masm` - A string containing Miden assembly code.
    /// * `inputs` - An Inputs struct containing the inputs for the program.
    ///
    /// # Returns
    ///
    /// A Result containing a MidenProgram if the parsing was successful, or a String error message otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, Inputs};
    /// let program = MidenProgram::parse_with_inputs("push.0\npush.1\nadd", Inputs::default()).unwrap();
    /// ```
    pub fn parse_with_inputs(masm: &str, inputs: Inputs) -> Result<Self, String> {
        let mut program = MidenProgram::new().with_inputs(inputs);
        let tokens = tokenize(masm);

        let (mut instructions, procedures) = parse(tokens)?;

        program.add_procs(procedures);

        program.add_instructions(&mut instructions);

        Ok(program)
    }

    /// Parses a Miden assembly file into a MidenProgram.
    ///
    /// # Arguments
    ///
    /// * `file` - A string containing the path to the Miden assembly file.
    ///
    /// # Returns
    ///
    /// A Result containing a MidenProgram if the parsing was successful, or a String error message otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram};
    /// if let Ok(program) = MidenProgram::parse_from_file("path/to/file.masm") {
    ///    // Do something with the program
    /// }
    /// ```

    pub fn parse_from_file(file: &str) -> Result<Self, String> {
        let mut program = MidenProgram::new();
        let file = std::fs::read_to_string(file);

        match file {
            Ok(file_string) => {
                let tokens = tokenize(&file_string);

                let (mut instructions, procedures) = parse(tokens)?;

                program.add_procs(procedures);

                program.add_instructions(&mut instructions);

                Ok(program)
            }
            Err(e) => Err(e.to_string()),
        }
    }
    /// Parses a Miden assembly file into a MidenProgram, with a given set of inputs.
    ///
    /// # Arguments
    ///
    /// * `file` - A string containing the path to the Miden assembly file.
    /// * `inputs` - An Inputs struct containing the inputs for the program.
    ///
    /// # Returns
    ///
    /// A Result containing a MidenProgram if the parsing was successful, or a String error message otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, Inputs};
    /// if let Ok(program) = MidenProgram::parse_from_file_with_inputs("path/to/file.masm", Inputs::default()) {
    ///   // Do something with the program
    /// }
    /// ```
    pub fn parse_from_file_with_inputs(file: &str, inputs: Inputs) -> Result<Self, String> {
        let mut program = MidenProgram::new().with_inputs(inputs);
        let file = std::fs::read_to_string(file);

        match file {
            Ok(file_string) => {
                let tokens = tokenize(&file_string);

                let (mut instructions, procedures) = parse(tokens)?;

                program.add_procs(procedures);

                program.add_instructions(&mut instructions);

                Ok(program)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
