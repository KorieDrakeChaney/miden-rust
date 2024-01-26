use std::collections::VecDeque;

use super::{MidenProgram, Operand, Program};

impl MidenProgram {
    pub fn if_else<T>(&mut self, if_op: &T, else_op: &T)
    where
        T: Program,
    {
        let mut temp_stack = VecDeque::new();
        let mut if_operands = if_op.get_operands();
        let mut else_operands = else_op.get_operands();

        temp_stack.push_back(Operand::IF);
        temp_stack.append(&mut if_operands);
        temp_stack.push_back(Operand::ELSE);
        temp_stack.append(&mut else_operands);
        temp_stack.push_back(Operand::END);

        self.add_operands(&temp_stack);
    }

    pub fn while_block<T>(&mut self, block: &T)
    where
        T: Program,
    {
        let mut temp_stack = VecDeque::new();
        let mut block_operands = block.get_operands();
        temp_stack.push_back(Operand::WHILE);
        temp_stack.append(&mut block_operands);
        temp_stack.push_back(Operand::END);
        self.add_operands(&temp_stack);
    }

    pub fn repeat<T>(&mut self, n: usize, program: &T)
    where
        T: Program,
    {
        let mut operands = program.get_operands();
        operands.push_front(Operand::REPEAT(n));
        operands.push_back(Operand::END);
        self.add_operands(&operands);
    }
}
