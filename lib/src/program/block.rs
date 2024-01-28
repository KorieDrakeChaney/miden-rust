use std::collections::VecDeque;

use super::{MidenProgram, Operand};

impl MidenProgram {
    /// Constructs a new `if-else` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `if_op` - A closure that returns the operands for the `if` block.
    /// * `else_op` - A closure that returns the operands for the `else` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main() {
    ///     let mut program = MidenProgram::new();
    ///     program.push(1);
    ///     program.if_else(|| {
    ///         let mut block = EmptyProgram::new();
    ///         block.push(1);
    ///         block.get_operands()},
    ///     || {
    ///         let mut block = EmptyProgram::new();
    ///         block.push(5);
    ///         block.get_operands()},
    ///     );
    /// }
    ///    
    /// ```
    pub fn if_else<F1, F2>(&mut self, if_op: F1, else_op: F2)
    where
        F1: FnOnce() -> VecDeque<Operand>,
        F2: FnOnce() -> VecDeque<Operand>,
    {
        let mut temp_stack = VecDeque::new();
        let mut if_operands = if_op();
        let mut else_operands = else_op();

        temp_stack.push_back(Operand::IF);
        temp_stack.append(&mut if_operands);
        temp_stack.push_back(Operand::ELSE);
        temp_stack.append(&mut else_operands);
        temp_stack.push_back(Operand::END);

        self.add_operands(&mut temp_stack);
    }
    /// Constructs a new `while` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `block` - A closure that returns the operands for the `while` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main() {
    ///     let mut program = MidenProgram::new();
    ///     program.while_block(|| {
    ///         let mut block = EmptyProgram::new();
    ///         block.push(1);
    ///         block.increment();
    ///         block.dup();
    ///         block.neq_n(10);
    ///
    ///         block.get_operands()
    ///     });
    /// }
    /// ```
    pub fn while_block<F>(&mut self, block: F)
    where
        F: FnOnce() -> VecDeque<Operand>,
    {
        let mut block_operands = block();
        block_operands.push_front(Operand::WHILE);
        block_operands.push_back(Operand::END);
        self.add_operands(&mut block_operands);
    }
    /// Constructs a new `repeat` block in the Miden program.
    ///
    /// # Arguments
    /// * `n` - The number of times to repeat the block.
    /// * `program` - A closure that returns the operands for the `repeat` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main(){
    ///     let mut program = MidenProgram::new();
    ///     program.repeat(5, || {
    ///         let mut block = EmptyProgram::new();
    ///         block.push(1);
    ///         block.push(2);
    ///         block.add();
    ///
    ///         block.get_operands()
    ///     });
    /// }
    ///
    pub fn repeat<F>(&mut self, n: usize, program: F)
    where
        F: FnOnce() -> VecDeque<Operand>,
    {
        let mut operands = program();
        operands.push_front(Operand::REPEAT(n));
        operands.push_back(Operand::END);
        self.add_operands(&mut operands);
    }
}
