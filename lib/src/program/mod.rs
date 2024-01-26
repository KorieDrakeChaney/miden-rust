mod block;
mod empty;
mod error;
mod execute;
mod field;
mod io;
mod operand;
mod utils;

pub use empty::*;

mod manipulation;

use crate::Inputs;
pub use operand::Operand;

use std::collections::{HashMap, VecDeque};

use math::{fields::f64::BaseElement, FieldElement, StarkField};
use miden::{
    prove, AdviceInputs, Assembler, DefaultHost, MemAdviceProvider, ProvingOptions, StackInputs,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramType {
    Proc(String),
    Begin,
}

pub struct MidenProgram {
    pub stack: VecDeque<BaseElement>,
    pub advice_stack: VecDeque<u64>,
    pub operand_stack: VecDeque<Operand>,

    internal_programs: HashMap<String, MidenProgram>,

    program_type: ProgramType,

    stack_inputs: StackInputs,
    advice_inputs: AdviceInputs,
    inputs: Inputs,

    ram_memory: HashMap<u32, [BaseElement; 4]>,
    loc_memory: HashMap<u16, [BaseElement; 4]>,
    loc_count: u16,
}

impl MidenProgram {
    pub fn new() -> MidenProgram {
        MidenProgram {
            stack: VecDeque::from(vec![BaseElement::ZERO; 16]),
            operand_stack: VecDeque::new(),
            advice_stack: VecDeque::new(),

            internal_programs: HashMap::new(),

            inputs: Inputs::default(),

            program_type: ProgramType::Begin,

            stack_inputs: StackInputs::default(),
            advice_inputs: AdviceInputs::default(),
            ram_memory: HashMap::new(),
            loc_memory: HashMap::new(),
            loc_count: 0,
        }
    }

    pub fn proc(name: &str) -> MidenProgram {
        MidenProgram {
            stack: VecDeque::from(vec![BaseElement::ZERO; 16]),
            operand_stack: VecDeque::new(),
            advice_stack: VecDeque::new(),

            internal_programs: HashMap::new(),

            inputs: Inputs::default(),

            program_type: ProgramType::Proc(name.to_string()),

            stack_inputs: StackInputs::default(),
            advice_inputs: AdviceInputs::default(),
            ram_memory: HashMap::new(),
            loc_memory: HashMap::new(),
            loc_count: 1,
        }
    }

    pub fn get_masm(&self) -> String {
        let mut masm: String;

        match self.program_type {
            ProgramType::Begin => {
                masm = String::new();
                for (_, program) in self.internal_programs.iter() {
                    match program.program_type {
                        ProgramType::Proc(_) => {
                            masm.push_str(&program.get_masm());
                        }
                        _ => {}
                    }
                }
                masm.push_str("begin\n");
            }
            ProgramType::Proc(ref name) => {
                masm = format!("proc.{}.{}\n", name, self.loc_count);
            }
        }

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
                    masm.push_str(&format!("{}{}\n", tabs, op));
                }

                &Operand::PRINT(_) | &Operand::Error(_) => {}
                _ => {
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                }
            }
        }

        masm.push_str(&format!("end\n\n"));

        if self.program_type == ProgramType::Begin {
            masm.push_str(&format!("#stack output : {:?} \n", &self.stack));
        }

        masm
    }

    pub fn get_stack(&self) -> &VecDeque<BaseElement> {
        &self.stack
    }

    pub fn get_ram_memory(&self) -> &HashMap<u32, [BaseElement; 4]> {
        &self.ram_memory
    }

    pub fn get_loc_memory(&self) -> &HashMap<u16, [BaseElement; 4]> {
        &self.loc_memory
    }

    pub fn print_masm(&self) {
        println!("{}", self);
    }

    pub fn print_operands(&self) {
        println!("{:?}", self.operand_stack);
    }

    pub fn save(&self, file: &str) {
        std::fs::write(file, self.get_masm()).unwrap();
    }

    pub fn prove(&mut self) -> Option<u64> {
        let assembler = Assembler::default();

        let program = assembler.compile(self.get_masm()).unwrap();

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

    pub fn with_inputs(mut self, inputs: Inputs) -> Self {
        if let Some(operand_stack) = inputs.operand_stack {
            self.stack = VecDeque::from(operand_stack.clone());
            self.inputs.operand_stack = Some(operand_stack.clone());
            self.stack_inputs =
                StackInputs::try_from_values(operand_stack.iter().map(|n| n.as_int())).unwrap();
        }
        if let Some(advice_stack) = inputs.advice_stack {
            self.advice_stack = VecDeque::from(advice_stack.clone());
            self.inputs.advice_stack = Some(advice_stack.clone());
            self.advice_inputs =
                AdviceInputs::with_stack_values(AdviceInputs::default(), advice_stack).unwrap();
        }
        self
    }

    pub fn with_operand_stack(mut self, operand_stack: Vec<BaseElement>) -> Self {
        self.stack = VecDeque::from(operand_stack.clone());
        self.inputs.operand_stack = Some(operand_stack.clone());
        self.stack_inputs =
            StackInputs::try_from_values(operand_stack.iter().map(|n| n.as_int())).unwrap();
        self
    }

    pub fn with_advice_stack(mut self, advice_stack: Vec<u64>) -> Self {
        self.advice_stack = VecDeque::from(advice_stack.clone());
        self.inputs.advice_stack = Some(advice_stack.clone());
        self.advice_inputs =
            AdviceInputs::with_stack_values(AdviceInputs::default(), advice_stack).unwrap();
        self
    }

    fn add_operands(&mut self, operands: VecDeque<Operand>) {
        for op in &operands {
            self.operand_stack.push_back(op.clone());
        }
        if self.program_type == ProgramType::Begin {
            self.execute_block(&operands);
        }
    }

    fn add_operand(&mut self, operand: Operand) {
        self.execute_operand(&operand);
        self.operand_stack.push_back(operand);
    }

    pub fn add_program<F>(&mut self, program: F)
    where
        F: FnOnce() -> VecDeque<Operand>,
    {
        self.add_operands(program());
    }

    pub fn print(&mut self, message: &str) {
        self.operand_stack
            .push_back(Operand::PRINT(message.to_string()));
    }

    pub fn append_proc(&mut self, program: MidenProgram) {
        if self.program_type == ProgramType::Begin {
            match program.program_type {
                ProgramType::Proc(ref name) => {
                    self.internal_programs.insert(name.clone(), program);
                }
                _ => {}
            }
        } else {
            todo!("error for appending proc to proc");
        }
    }

    pub fn get_operands(&self) -> VecDeque<Operand> {
        self.operand_stack.clone()
    }
}

impl std::fmt::Display for MidenProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_masm())
    }
}
