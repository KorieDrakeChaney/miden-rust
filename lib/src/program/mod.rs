mod advice_inject;
mod block;
mod empty;
mod error;
mod execute;
mod field;
mod instruction;
mod io;
mod manipulation;
mod parser;
mod proc;
mod u32;
use crate::Inputs;

use std::cell::RefCell;
use std::rc::Rc;

pub use empty::*;
pub use instruction::Instruction;

use std::collections::{HashMap, VecDeque};

use math::{fields::f64::BaseElement, FieldElement, StarkField};
use miden::{
    prove, AdviceInputs, Assembler, DefaultHost, ExecutionProof, MemAdviceProvider, ProvingOptions,
    StackInputs,
};

pub use self::proc::Proc;

pub trait Program {
    fn get_instructions(&self) -> VecDeque<Instruction>;
}

pub struct MidenProgram {
    pub stack: VecDeque<BaseElement>,
    pub advice_stack: VecDeque<u64>,
    pub instructions: VecDeque<Instruction>,

    internal_programs: HashMap<String, Rc<RefCell<Proc>>>,
    internal_programs_order: Vec<String>,

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
            instructions: VecDeque::new(),
            advice_stack: VecDeque::new(),

            internal_programs: HashMap::new(),
            internal_programs_order: Vec::new(),

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
        let mut masm: String = String::new();

        for name in self.internal_programs_order.iter() {
            if let Some(proc) = self.internal_programs.get(name) {
                masm.push_str(&format!("{}\n", proc.borrow().get_masm()));
            }
        }

        masm.push_str("begin\n");

        let mut scope = 1;
        for op in self.instructions.iter() {
            match op {
                Instruction::IF | Instruction::WHILE | Instruction::REPEAT(_) => {
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                    scope += 1;
                }
                Instruction::ELSE => {
                    scope -= 1;
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                    scope += 1;
                }
                Instruction::END => {
                    scope -= 1;
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n\n", tabs, op));
                }

                Instruction::Error(e) => {
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("\n{}#ERROR: {}\n", tabs, e));
                }

                Instruction::CommentedOut(_) => {
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n\n", tabs, op));
                }

                Instruction::PRINT(_) => {}
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

    /// Prints the operands in the instruction stack.
    pub fn print_operands(&self) {
        println!("{:?}", self.instructions);
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

    pub fn prove(&mut self) -> Option<ExecutionProof> {
        let assembler = Assembler::default();
        let masm = self.get_masm();

        match assembler.compile(masm) {
            Ok(program) => {
                let advice_provider = MemAdviceProvider::from(self.advice_inputs.clone());

                let host = DefaultHost::new(advice_provider);

                if let Ok((outputs, proof)) = prove(
                    &program,
                    self.stack_inputs.clone(),
                    host,
                    ProvingOptions::default(),
                ) {
                    Some(proof)
                } else {
                    None
                }
            }
            Err(_) => {
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
            let mut stack: VecDeque<BaseElement> = VecDeque::new();
            while i < operand_stack.len() {
                stack.push_front(operand_stack[i]);
                i += 1;
            }

            while stack.len() < 16 {
                stack.push_back(BaseElement::ZERO);
            }

            self.stack = stack;
        }
        if let Some(mut advice_stack) = inputs.advice_stack {
            self.advice_inputs =
                AdviceInputs::with_stack_values(AdviceInputs::default(), advice_stack.clone())
                    .unwrap();

            while let Some(a) = advice_stack.pop() {
                self.advice_stack.push_front(a);
            }
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
        let mut stack: VecDeque<BaseElement> = VecDeque::new();
        while i < operand_stack.len() {
            stack.push_front(operand_stack[i]);
            i += 1;
        }

        while stack.len() < 16 {
            stack.push_back(BaseElement::ZERO);
        }

        self.stack = stack;
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

    /// Adds the specified instructions to the instruction stack of the program.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The operands to add.
    pub fn add_instructions(&mut self, instructions: &mut VecDeque<Instruction>) {
        for instruction in instructions.iter() {
            self.instructions.push_back(instruction.clone());
        }
        self.execute_block(instructions, 0);
    }

    /// Adds the specified instruction to the instruction stack of the program.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The operands to add.
    pub fn add_instruction(&mut self, instruction: Instruction) {
        match self.is_valid_operand(&instruction) {
            Some(error) => {
                self.instructions
                    .push_back(Instruction::Error(error.clone()));
                self.instructions
                    .push_back(Instruction::CommentedOut(instruction.to_string()));
            }
            _ => {
                self.instructions.push_back(instruction.clone());
                self.execute_operand(&instruction);
            }
        }
    }

    /// Adds the specified operands to the operand stack of the program.
    ///
    /// # Arguments
    ///
    /// * `operands` - The operands to add.
    pub fn add_program<'a, T>(&'a mut self, program: &mut T)
    where
        T: Program + 'a,
    {
        self.add_instructions(&mut program.get_instructions());
    }

    /// Adds a `PRINT` operand to the operand stack with the specified message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to print.
    pub fn print(&mut self, message: &str) {
        self.add_instruction(Instruction::PRINT(message.to_string()));
    }

    /// Appends a procedure to the internal programs of the program.
    ///
    /// # Arguments
    ///
    /// * `program` - The procedure to append.
    ///
    pub fn add_proc(&mut self, program: Proc) {
        let name = program.name.clone();
        self.internal_programs_order.push(name.clone());
        self.internal_programs
            .insert(name, Rc::new(RefCell::new(program)));
    }

    pub fn add_procs(&mut self, programs: Vec<Proc>) {
        for program in programs {
            self.add_proc(program);
        }
    }
}

impl Program for MidenProgram {
    fn get_instructions(&self) -> VecDeque<Instruction> {
        self.instructions.clone()
    }
}

impl std::fmt::Display for MidenProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_masm())
    }
}
