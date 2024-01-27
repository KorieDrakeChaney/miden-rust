use math::fields::f64::BaseElement;

use super::{error::MidenProgramError, MidenProgram, Operand, ProgramType};

impl MidenProgram {
    /// Pushes a value onto the stack.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to push onto the stack.
    pub fn push(&mut self, value: u64) {
        self.add_operand(Operand::Push(BaseElement::from(value)));
    }
    /// Pushes the nth advice onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the advice to push onto the stack.
    ///
    /// # Errors
    ///
    /// Returns `MidenProgramError::AdviceStackReadOutOfBounds` if `n` is greater than the length of the advice stack.
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
    /// Stores the second value on the stack in memory at the address specified by the first value on the stack.
    pub fn mem_store(&mut self) {
        self.add_operand(Operand::MemStore);
    }

    /// Stores the first value in the stack in memory at the address specified by `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store the first value on the stack.
    pub fn mem_store_n(&mut self, n: u32) {
        self.add_operand(Operand::MemStoreImm(n));
    }

    /// Uses the first value on the stack as an address and stores the 2-5th values on the stack in memory at that address.
    pub fn mem_store_w(&mut self) {
        self.add_operand(Operand::MemStoreW);
    }

    /// Stores the first word in the stack in memory at the address specified by `n`.
    /// # Arguments
    /// * `n` - The address to store the first word on the stack.
    pub fn mem_store_w_n(&mut self, n: u32) {
        self.add_operand(Operand::MemStoreWImm(n));
    }

    /// Assumes top value on the stack is an address and pops it off, then loads the value at that address from RAM onto the stack.
    pub fn mem_load(&mut self) {
        self.add_operand(Operand::MemLoad);
    }

    /// Loads the value at address `n` from RAM onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load onto the stack.
    pub fn mem_load_n(&mut self, n: u32) {
        self.add_operand(Operand::MemLoadImm(n));
    }

    /// Assumes top value on the stack is an address and pops it off, then loads the word at that address from RAM onto the stack.
    pub fn mem_load_w(&mut self) {
        self.add_operand(Operand::MemLoadW);
    }

    /// Loads the word at address `n` from RAM onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load onto the stack.
    pub fn mem_load_w_n(&mut self, n: u32) {
        self.add_operand(Operand::MemLoadWImm(n));
    }

    /// Stores the top value on the stack in local memory at address `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store the top value on the stack.
    ///
    /// # Errors
    ///
    /// Returns `MidenProgramError::LocStoreInBegin` if the program type is `Begin`.
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

    /// Stores the first word on the stack in local memory at address `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store the top value on the stack.
    /// # Errors
    /// Returns `MidenProgramError::LocStoreInBegin` if the program type is `Begin`.
    pub fn loc_store_w(&mut self, n: u16) {
        let op: Operand;
        match self.program_type {
            ProgramType::Proc(_) => {
                if n >= self.loc_count {
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

    /// Assumes top value on the stack is an address and pops it off, then loads the value at that address onto the stack.
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

    /// Loads the word at address `n` from local memory onto the stack.
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
