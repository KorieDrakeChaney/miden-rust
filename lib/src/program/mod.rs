mod block;
mod empty;
mod error;
mod execute;
mod field;
mod io;
mod manipulation;
mod operand;
mod parser;
mod proc;
mod u32;
mod utils;
use crate::Inputs;

pub use empty::*;
pub use operand::Operand;

use std::collections::{HashMap, VecDeque};

use math::{fields::f64::BaseElement, FieldElement, StarkField};
use miden::{
    prove, AdviceInputs, Assembler, DefaultHost, MemAdviceProvider, ProvingOptions, StackInputs,
};

pub use self::proc::Proc;

pub struct MidenProgram {
    pub stack: VecDeque<BaseElement>,
    pub advice_stack: VecDeque<u64>,
    pub operand_stack: VecDeque<Operand>,
    internal_programs: HashMap<String, Proc>,

    proc_script: String,

    runnable: bool,

    stack_inputs: StackInputs,
    advice_inputs: AdviceInputs,

    ram_memory: HashMap<u32, [BaseElement; 4]>,
    loc_memory: HashMap<u16, [BaseElement; 4]>,
}

impl MidenProgram {
    /// Creates a new `MidenProgram` with default values.
    ///
    /// # Returns
    ///
    /// A new `MidenProgram`.
    pub fn new() -> MidenProgram {
        MidenProgram {
            stack: VecDeque::from(vec![BaseElement::ZERO; 16]),
            operand_stack: VecDeque::new(),
            advice_stack: VecDeque::new(),

            internal_programs: HashMap::new(),

            proc_script: String::new(),

            runnable: true,

            stack_inputs: StackInputs::default(),
            advice_inputs: AdviceInputs::default(),
            ram_memory: HashMap::new(),
            loc_memory: HashMap::new(),
        }
    }

    /// Returns the Miden Assembly (MASM) representation of the program.
    ///
    /// # Returns
    ///
    /// A string containing the MASM representation of the program.
    pub fn get_masm(&self) -> String {
        let mut masm: String = self.proc_script.clone();
        masm.push_str("begin\n");

        let mut scope = 1;
        for op in self.operand_stack.iter() {
            match op {
                &Operand::IF | &Operand::WHILE | &Operand::REPEAT(_) => {
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                    scope += 1;
                }
                &Operand::ELSE => {
                    scope -= 1;
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                    scope += 1;
                }
                &Operand::END => {
                    scope -= 1;
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n\n", tabs, op));
                }

                &Operand::PRINT(_) | &Operand::Error(_) => {}
                _ => {
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                }
            }
        }

        masm.push_str(&format!("end\n\n"));

        masm.push_str(&format!("#stack output : {:?} \n", &self.stack));

        masm
    }

    /// Returns a reference to the stack.
    ///
    /// # Returns
    ///
    /// A reference to the `VecDeque` representing the stack.
    pub fn get_stack(&self) -> &VecDeque<BaseElement> {
        &self.stack
    }

    /// Returns a reference to the RAM memory.
    ///
    /// # Returns
    ///
    /// A reference to the `HashMap` representing the RAM memory.
    pub fn get_ram_memory(&self) -> &HashMap<u32, [BaseElement; 4]> {
        &self.ram_memory
    }

    /// Prints the Miden Assembly (MASM) representation of the program.
    pub fn print_masm(&self) {
        println!("{}", self);
    }

    /// Prints the operands in the operand stack.
    pub fn print_operands(&self) {
        println!("{:?}", self.operand_stack);
    }

    /// Saves the MASM representation of the program to a file.
    ///
    /// # Arguments
    ///
    /// * `file` - The name of the file to save to.
    pub fn save(&self, file: &str) {
        std::fs::write(file, self.get_masm()).unwrap();
    }

    /// Proves the program and returns the result.
    ///
    /// # Returns
    ///
    /// The result of the proof as an `Option<u64>`.

    pub fn prove(&mut self) -> Option<u64> {
        let assembler = Assembler::default();
        let masm = self.get_masm();

        match assembler.compile(masm) {
            Ok(program) => {
                let advice_provider = MemAdviceProvider::from(self.advice_inputs.clone());

                let host = DefaultHost::new(advice_provider);

                let (outputs, _) = prove(
                    &program,
                    self.stack_inputs.clone(),
                    host,
                    ProvingOptions::default(),
                )
                .unwrap();

                if let Some(output) = outputs.stack().first() {
                    return Some(*output);
                } else {
                    return None;
                }
            }
            Err(e) => {
                println!("{}", e);
                return None;
            }
        }
    }

    /// Sets the inputs of the program and returns the program.
    ///
    /// # Arguments
    ///
    /// * `inputs` - The inputs to set.
    ///
    /// # Returns
    ///
    /// The program with the specified inputs.
    pub fn with_inputs(mut self, inputs: Inputs) -> Self {
        if let Some(operand_stack) = inputs.operand_stack {
            self.stack_inputs =
                StackInputs::try_from_values(operand_stack.iter().map(|n| n.as_int())).unwrap();

            let mut i = 0;
            while i < operand_stack.len() {
                self.stack.push_front(operand_stack[i]);
                i += 1;
            }
        }
        if let Some(mut advice_stack) = inputs.advice_stack {
            self.advice_inputs =
                AdviceInputs::with_stack_values(AdviceInputs::default(), advice_stack.clone())
                    .unwrap();

            while let Some(a) = advice_stack.pop() {
                self.advice_stack.push_front(a);
            }

            println!("advice stack: {:?}", self.advice_stack);
        }
        self
    }

    /// Sets the operand stack of the program and returns the program.
    ///
    /// # Arguments
    ///
    /// * `operand_stack` - The operand stack to set.
    ///
    /// # Returns
    ///
    /// The program with the specified operand stack.
    pub fn with_operand_stack(mut self, operand_stack: Vec<BaseElement>) -> Self {
        let mut i = 0;
        while i < operand_stack.len() {
            self.stack.push_front(operand_stack[i]);
            i += 1;
        }

        self.stack_inputs =
            StackInputs::try_from_values(operand_stack.iter().map(|n| n.as_int())).unwrap();
        self
    }

    /// Sets the advice stack of the program and returns the program.
    ///
    /// # Arguments
    ///
    /// * `advice_stack` - The advice stack to set.
    ///
    /// # Returns
    ///
    /// The program with the specified advice stack.
    pub fn with_advice_stack(mut self, advice_stack: Vec<u64>) -> Self {
        let mut i = 0;

        while i < advice_stack.len() {
            self.advice_stack.push_front(advice_stack[i]);
            i += 1;
        }

        self.advice_inputs =
            AdviceInputs::with_stack_values(AdviceInputs::default(), advice_stack.clone()).unwrap();
        self
    }

    /// Adds the specified operands to the operand stack of the program.
    ///
    /// # Arguments
    ///
    /// * `operands` - The operands to add.
    pub fn add_operands(&mut self, operands: &mut VecDeque<Operand>) {
        for op in operands.clone() {
            self.operand_stack.push_back(op.clone());
        }
        self.execute_block(operands);
    }

    pub fn add_operand(&mut self, operand: Operand) {
        println!("executing operand: {:?}", &operand);
        self.execute_operand(&operand);
        self.operand_stack.push_back(operand);
    }

    /// Adds the specified operands to the operand stack of the program.
    ///
    /// # Arguments
    ///
    /// * `operands` - The operands to add.
    pub fn add_program<F>(&mut self, program: F)
    where
        F: FnOnce() -> VecDeque<Operand>,
    {
        self.add_operands(&mut program());
    }

    /// Adds a `PRINT` operand to the operand stack with the specified message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to print.
    pub fn print(&mut self, message: &str) {
        self.add_operand(Operand::PRINT(message.to_string()));
    }

    /// Appends a procedure to the internal programs of the program.
    ///
    /// # Arguments
    ///
    /// * `program` - The procedure to append.
    ///
    pub fn add_proc(&mut self, program: Proc) {
        self.proc_script.push_str(&program.get_masm());

        self.proc_script.push_str("\n");

        self.internal_programs.insert(program.name.clone(), program);
    }

    pub fn add_procs(&mut self, programs: Vec<Proc>) {
        for program in programs {
            self.add_proc(program);
        }
    }

    pub fn set_runnable(&mut self, runnable: bool) {
        self.runnable = runnable;
    }

    /// Returns a clone of the operand stack.
    ///
    /// # Returns
    ///
    /// A `VecDeque` containing the operands in the operand stack.
    pub fn get_operands(&self) -> VecDeque<Operand> {
        self.operand_stack.clone()
    }
}

impl std::fmt::Display for MidenProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_masm())
    }
}
