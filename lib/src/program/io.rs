use math::fields::f64::BaseElement;

use super::{error::MidenProgramError, MidenProgram, Operand, ProgramType};

impl MidenProgram {
    pub fn push(&mut self, value: u64) {
        self.add_operand(Operand::Push(BaseElement::from(value)));
    }

    pub fn adv_push(&mut self, n: usize) {
        let op: Operand;
        if n > self.advice_stack.len() {
            op = Operand::Error(MidenProgramError::AdviceStackReadOutOfBounds(
                n,
                self.advice_stack.len(),
            ));
        } else {
            op = Operand::AdvPush(n);
        }
        self.add_operand(op);
    }

    pub fn mem_store(&mut self) {
        self.add_operand(Operand::MemStore);
    }

    pub fn mem_store_n(&mut self, n: u32) {
        self.add_operand(Operand::MemStoreImm(n));
    }

    pub fn mem_store_w(&mut self) {
        self.add_operand(Operand::MemStoreW);
    }

    pub fn mem_store_w_n(&mut self, n: u32) {
        self.add_operand(Operand::MemStoreWImm(n));
    }

    pub fn mem_load(&mut self) {
        self.add_operand(Operand::MemLoad);
    }

    pub fn mem_load_n(&mut self, n: u32) {
        self.add_operand(Operand::MemLoadImm(n));
    }

    pub fn mem_load_w(&mut self) {
        self.add_operand(Operand::MemLoadW);
    }

    pub fn mem_load_w_n(&mut self, n: u32) {
        self.add_operand(Operand::MemLoadWImm(n));
    }

    pub fn loc_store(&mut self, n: u16) {
        let op: Operand;
        match self.program_type {
            ProgramType::Proc(_) => {
                if n > self.loc_count {
                    self.loc_count = n + 1;
                }
                op = Operand::LocStore(n);
            }
            ProgramType::Begin => {
                op = Operand::Error(MidenProgramError::LocStoreInBegin);
            }
        }
        self.add_operand(op);
    }

    pub fn loc_store_w(&mut self, n: u16) {
        let op: Operand;
        match self.program_type {
            ProgramType::Proc(_) => {
                if n > self.loc_count {
                    self.loc_count = n + 1;
                }
                op = Operand::LocStoreW(n);
            }
            ProgramType::Begin => {
                op = Operand::Error(MidenProgramError::LocStoreInBegin);
            }
        }
        self.add_operand(op);
    }

    pub fn loc_load(&mut self, n: u16) {
        let op: Operand;
        match self.program_type {
            ProgramType::Proc(_) => {
                op = Operand::LocLoad(n);
            }
            ProgramType::Begin => {
                op = Operand::Error(MidenProgramError::LocLoadInBegin);
            }
        }
        self.add_operand(op);
    }

    pub fn loc_load_w(&mut self, n: u16) {
        let op: Operand;
        match self.program_type {
            ProgramType::Proc(_) => {
                op = Operand::LocLoadW(n);
            }
            ProgramType::Begin => {
                op = Operand::Error(MidenProgramError::LocLoadInBegin);
            }
        }
        self.add_operand(op);
    }
}
