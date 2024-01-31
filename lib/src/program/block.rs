use std::collections::VecDeque;

use crate::Program;

use super::{Instruction, MidenProgram};

impl MidenProgram {
    /// Constructs a new `if-else` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `if_program` - A mutable reference to the program for the `if` block.
    /// * `else_program` - A mutable reference to the program for the `else` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main(){
    ///     let mut program = MidenProgram::new();
    ///     let mut if_program = EmptyProgram::new();
    ///     if_program.push(1);
    ///     let mut else_program = EmptyProgram::new();
    ///     else_program.push(5);
    ///     program.if_else_block(&mut if_program, &mut else_program);
    /// }
    /// ```
    pub fn if_else_block<'a, T>(&mut self, if_program: &mut T, else_program: &mut T)
    where
        T: Program + 'a,
    {
        let mut temp_stack = VecDeque::new();
        let mut if_instructions = if_program.get_instructions();
        let mut else_instructions = else_program.get_instructions();

        temp_stack.push_back(Instruction::IF);
        temp_stack.append(&mut if_instructions);
        temp_stack.push_back(Instruction::ELSE);
        temp_stack.append(&mut else_instructions);
        temp_stack.push_back(Instruction::END);

        self.add_instructions(&mut temp_stack);
    }
    /// Constructs a new `if` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `block` - An instance of a type implementing the `Program` trait, which provides the instructions for the `if` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main(){
    ///     let mut program = MidenProgram::new();
    ///     let mut if_program = EmptyProgram::new();
    ///     if_program.push(1);
    ///     if_program.push(2);
    ///     if_program.add();
    ///     program.if_block(&mut if_program);
    /// }
    /// ```
    pub fn if_block<'a, T>(&'a mut self, block: &mut T)
    where
        T: Program + 'a,
    {
        let mut block_operands = block.get_instructions();
        block_operands.push_front(Instruction::IF);
        block_operands.push_back(Instruction::END);
        self.add_instructions(&mut block_operands);
    }
    /// Constructs a new `while` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `block` - A mutable reference to the program for the `while` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main(){
    ///     let mut program = MidenProgram::new();
    ///     let mut while_program = EmptyProgram::new();
    ///     while_program.push(1);
    ///     while_program.increment();
    ///     while_program.dup();
    ///     while_program.neq_n(10);
    ///     program.while_block(&mut while_program);
    /// }
    /// ```
    pub fn while_block<'a, T>(&'a mut self, block: &mut T)
    where
        T: Program + 'a,
    {
        let mut block_operands = block.get_instructions();
        block_operands.push_front(Instruction::WHILE);
        block_operands.push_back(Instruction::END);
        self.add_instructions(&mut block_operands);
    }
    /// Constructs a new `repeat` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of times to repeat the block.
    /// * `program` - A mutable reference to the program for the `repeat` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main(){
    ///     let mut program = MidenProgram::new();
    ///     let mut repeat_program = EmptyProgram::new();
    ///     repeat_program.push(1);
    ///     repeat_program.push(2);
    ///     repeat_program.add();
    ///     program.repeat(5, &mut repeat_program);
    /// }
    /// ```
    pub fn repeat<'a, T>(&'a mut self, n: usize, program: &mut T)
    where
        T: Program + 'a,
    {
        let mut operands = program.get_instructions();
        operands.push_front(Instruction::REPEAT(n));
        operands.push_back(Instruction::END);
        self.add_instructions(&mut operands);
    }
}
