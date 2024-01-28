use std::collections::VecDeque;

use math::{fields::f64::BaseElement, FieldElement};

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

pub fn execute_while(program: &mut MidenProgram, block: &mut VecDeque<Operand>) {
    let mut while_block = VecDeque::new();
    let mut scope_count = 1;
    'while_block: while let Some(next_op) = block.pop_front() {
        match next_op {
            Operand::END => {
                scope_count -= 1;
                if scope_count == 0 {
                    break 'while_block;
                } else {
                    while_block.push_back(next_op);
                }
            }
            Operand::WHILE | Operand::IF | Operand::REPEAT(_) => {
                scope_count += 1;
                while_block.push_back(next_op);
            }
            _ => {
                while_block.push_back(next_op);
            }
        }
    }

    'while_loop: loop {
        if let Some(n) = program.stack.pop_front() {
            if n == BaseElement::ONE {
                program.execute_block(&mut while_block.clone());
            } else {
                break 'while_loop;
            }
        }
    }
}

pub fn execute_if_else(program: &mut MidenProgram, block: &mut VecDeque<Operand>) {
    if let Some(n) = program.stack.pop_front() {
        let mut if_block = VecDeque::new();
        let mut else_block = VecDeque::new();
        let mut if_scope_count = 1;
        let mut else_scope_count = 1;

        'if_block: while let Some(next_op) = block.pop_front() {
            match next_op {
                Operand::ELSE => {
                    if if_scope_count == 1 {
                        break 'if_block;
                    } else {
                        if_block.push_back(next_op);
                    }
                }
                Operand::IF | Operand::WHILE | Operand::REPEAT(_) => {
                    if_scope_count += 1;
                    if_block.push_back(next_op);
                }
                Operand::END => {
                    if_scope_count -= 1;
                    if if_scope_count == 0 {
                        break 'if_block;
                    } else {
                        if_block.push_back(next_op);
                    }
                }
                _ => {
                    if_block.push_back(next_op);
                }
            }
        }

        if if_scope_count > 0 {
            'else_block: while let Some(next_op) = block.pop_front() {
                match next_op {
                    Operand::END => {
                        else_scope_count -= 1;
                        if else_scope_count == 0 {
                            break 'else_block;
                        } else {
                            else_block.push_back(next_op);
                        }
                    }
                    Operand::WHILE | Operand::IF | Operand::REPEAT(_) => {
                        else_scope_count += 1;
                        else_block.push_back(next_op);
                    }
                    _ => {
                        else_block.push_back(next_op);
                    }
                }
            }
        }

        if n == BaseElement::ONE {
            if if_block.len() > 0 {
                program.execute_block(&mut if_block.clone());
            }
        } else {
            if else_block.len() > 0 {
                program.execute_block(&mut else_block.clone());
            }
        }
    }
}

pub fn execute_repeat(n: usize, program: &mut MidenProgram, mut block: &mut VecDeque<Operand>) {
    let mut repeat_operands = VecDeque::new();
    let mut scope_count = 1;
    'outer: while let Some(next_op) = block.pop_front() {
        match next_op {
            Operand::END => {
                scope_count -= 1;
                if scope_count == 0 {
                    break 'outer;
                } else {
                    repeat_operands.push_back(next_op);
                }
            }
            Operand::WHILE | Operand::IF | Operand::REPEAT(_) => {
                scope_count += 1;
                repeat_operands.push_back(next_op);
            }
            _ => {
                repeat_operands.push_back(next_op);
            }
        }
    }

    for _ in 0..n {
        program.execute_block(&mut repeat_operands.clone());
    }
}
