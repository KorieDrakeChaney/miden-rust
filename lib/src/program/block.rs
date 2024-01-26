use std::collections::VecDeque;

use super::{MidenProgram, Operand, Program};

impl MidenProgram {
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

        self.add_operands(&temp_stack);
    }

    pub fn while_block<F>(&mut self, block: F)
    where
        F: FnOnce() -> VecDeque<Operand>,
    {
        let mut temp_stack = VecDeque::new();
        let mut block_operands = block();
        temp_stack.push_back(Operand::WHILE);
        temp_stack.append(&mut block_operands);
        temp_stack.push_back(Operand::END);
        self.add_operands(&temp_stack);
    }

    pub fn repeat<F>(&mut self, n: usize, program: F)
    where
        F: FnOnce() -> VecDeque<Operand>,
    {
        let mut operands = program();
        operands.push_front(Operand::REPEAT(n));
        operands.push_back(Operand::END);
        self.add_operands(&operands);
    }
}
