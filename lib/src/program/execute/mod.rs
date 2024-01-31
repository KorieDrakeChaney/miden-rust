mod advice;
mod arithmetic;
mod boolean;
mod comparison;
mod conditional_manipulation;
mod conversion;
mod environment;
mod extensions;
mod hashing;
mod manipulation;
mod memory;
mod merkle;
mod u32_arithmetic;
mod u32_bitwise;
mod utils;
mod valid_checker;

use self::arithmetic::execute_arithmetic;
use self::comparison::execute_comparison;
use self::conditional_manipulation::execute_conditional;
use self::extensions::execute_extensions;
use self::manipulation::execute_manipulation;
use self::u32_arithmetic::execute_u32_arithmetic;
use self::u32_bitwise::execute_u32_bitwise;
use self::{boolean::execute_boolean, memory::execute_memory};

use super::{Instruction, MidenProgram};
use math::{fields::f64::BaseElement, FieldElement};
use std::collections::VecDeque;

impl MidenProgram {
    pub fn execute_block(&mut self, block: &mut VecDeque<Instruction>, scope: usize) {
        let mut index = scope;
        while let Some(op) = block.pop_front() {
            match self.is_valid_operand(&op) {
                Some(error) => {
                    if let Some(op) = self.instructions.get_mut(index) {
                        match op {
                            Instruction::Error(_) | Instruction::CommentedOut(_) => {}
                            _ => {
                                *op = Instruction::CommentedOut(op.to_string());
                                self.instructions
                                    .insert(index, Instruction::Error(error.clone()));
                            }
                        }
                    }
                    index += 2;
                    continue;
                }
                _ => {
                    index += 1;
                }
            }

            match op {
                Instruction::IF => {
                    if let Some(n) = self.stack.pop_front() {
                        let mut if_block = VecDeque::new();
                        let mut else_block = VecDeque::new();
                        let mut if_scope_count = 1;
                        let mut else_scope_count = 1;

                        'if_block: while let Some(next_op) = block.pop_front() {
                            match next_op {
                                Instruction::ELSE => {
                                    if if_scope_count == 1 {
                                        break 'if_block;
                                    } else {
                                        if_block.push_back(next_op);
                                    }
                                }
                                Instruction::IF | Instruction::WHILE | Instruction::REPEAT(_) => {
                                    if_scope_count += 1;
                                    if_block.push_back(next_op);
                                }
                                Instruction::END => {
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
                                    Instruction::END => {
                                        else_scope_count -= 1;
                                        if else_scope_count == 0 {
                                            break 'else_block;
                                        } else {
                                            else_block.push_back(next_op);
                                        }
                                    }
                                    Instruction::WHILE
                                    | Instruction::IF
                                    | Instruction::REPEAT(_) => {
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
                                self.execute_block(&mut if_block.clone(), index);
                            }
                        } else {
                            if else_block.len() > 0 {
                                self.execute_block(&mut else_block.clone(), index);
                            }
                        }

                        index += if_block.len() + else_block.len() + 1;

                        if if_scope_count > 0 {
                            index += 1;
                        }
                    }
                }
                Instruction::WHILE => {
                    let mut while_block = VecDeque::new();
                    let mut scope_count = 1;
                    'while_block: while let Some(next_op) = block.pop_front() {
                        match next_op {
                            Instruction::END => {
                                scope_count -= 1;
                                if scope_count == 0 {
                                    break 'while_block;
                                } else {
                                    while_block.push_back(next_op);
                                }
                            }
                            Instruction::WHILE | Instruction::IF | Instruction::REPEAT(_) => {
                                scope_count += 1;
                                while_block.push_back(next_op);
                            }
                            _ => {
                                while_block.push_back(next_op);
                            }
                        }
                    }

                    'while_loop: loop {
                        if let Some(n) = self.stack.pop_front() {
                            if n == BaseElement::ONE {
                                self.execute_block(&mut while_block.clone(), index);
                            } else {
                                break 'while_loop;
                            }
                        }
                    }
                    index += while_block.len() + 1;
                }
                Instruction::REPEAT(n) => {
                    let mut repeat_operands = VecDeque::new();
                    let mut scope_count = 1;
                    'outer: while let Some(next_op) = block.pop_front() {
                        match next_op {
                            Instruction::END => {
                                scope_count -= 1;
                                if scope_count == 0 {
                                    break 'outer;
                                } else {
                                    repeat_operands.push_back(next_op);
                                }
                            }
                            Instruction::WHILE | Instruction::IF | Instruction::REPEAT(_) => {
                                scope_count += 1;
                                repeat_operands.push_back(next_op);
                            }
                            _ => {
                                repeat_operands.push_back(next_op);
                            }
                        }
                    }

                    for _ in 0..n {
                        self.execute_block(&mut repeat_operands.clone(), index);
                    }
                    index += repeat_operands.len() + 1;
                }

                Instruction::Error(error) => {
                    println!("Error: {}", error);
                }

                _ => {
                    self.execute_operand(&op);
                }
            }
            while self.stack.len() < 16 {
                self.stack.push_back(BaseElement::from(0_u64));
            }
        }
    }

    pub fn execute_operand(&mut self, op: &Instruction) {
        execute_arithmetic(self, op);
        execute_manipulation(self, op);
        execute_comparison(self, op);
        execute_boolean(self, op);
        execute_memory(self, op);
        execute_u32_arithmetic(self, op);
        execute_u32_bitwise(self, op);
        execute_extensions(self, op);
        execute_conditional(self, op);

        match op {
            Instruction::AdvPush(n) => {
                for _ in 0..*n {
                    if let Some(a) = self.advice_stack.pop_front() {
                        self.stack.push_front(BaseElement::from(a));
                    }
                }
            }

            Instruction::AdvPipe => {
                if let (s2, s1, s0, Some(a), t1, t0) = (
                    (
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                    ),
                    (
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                    ),
                    (
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                    ),
                    self.stack.pop_front(),
                    (
                        self.advice_stack.pop_front(),
                        self.advice_stack.pop_front(),
                        self.advice_stack.pop_front(),
                        self.advice_stack.pop_front(),
                    ),
                    (
                        self.advice_stack.pop_front(),
                        self.advice_stack.pop_front(),
                        self.advice_stack.pop_front(),
                        self.advice_stack.pop_front(),
                    ),
                ) {
                    self.stack.push_front(a + BaseElement::from(2_u64));
                    // S0
                    if let (Some(d), Some(c), Some(b), Some(a)) = s0 {
                        self.stack.push_front(BaseElement::from(a));
                        self.stack.push_front(BaseElement::from(b));
                        self.stack.push_front(BaseElement::from(c));
                        self.stack.push_front(BaseElement::from(d));
                    }
                    // S1
                    if let (Some(_), Some(_), Some(_), Some(_)) = s1 {
                        if let (Some(d), Some(c), Some(b), Some(a)) = t0 {
                            self.stack.push_front(BaseElement::from(a));
                            self.stack.push_front(BaseElement::from(b));
                            self.stack.push_front(BaseElement::from(c));
                            self.stack.push_front(BaseElement::from(d));
                            self.ram_memory.insert(
                                1,
                                [
                                    BaseElement::from(a),
                                    BaseElement::from(b),
                                    BaseElement::from(c),
                                    BaseElement::from(d),
                                ],
                            );
                        }
                    }
                    // S2
                    if let (Some(_), Some(_), Some(_), Some(_)) = s2 {
                        if let (Some(d), Some(c), Some(b), Some(a)) = t1 {
                            self.stack.push_front(BaseElement::from(a));
                            self.stack.push_front(BaseElement::from(b));
                            self.stack.push_front(BaseElement::from(c));
                            self.stack.push_front(BaseElement::from(d));

                            self.ram_memory.insert(
                                0,
                                [
                                    BaseElement::from(a),
                                    BaseElement::from(b),
                                    BaseElement::from(c),
                                    BaseElement::from(d),
                                ],
                            );
                        }
                    }
                }
                for _ in 0..8 {
                    if let (Some(a), Some(_), Some(_), Some(_)) = (
                        self.advice_stack.pop_front(),
                        self.advice_stack.pop_front(),
                        self.advice_stack.pop_front(),
                        self.advice_stack.pop_front(),
                    ) {
                        self.stack.push_front(BaseElement::from(a));
                    }
                }
            }

            Instruction::Exec(name) => {
                if let Some(program) = self.internal_programs.get(name).cloned() {
                    program.borrow_mut().execute(self);
                }
            }

            Instruction::PRINT(message) => {
                println!("____________________________________________________\n");
                println!("{}", message);
                println!("stack : {:?}\n", self.stack);
                println!("ram : {:?}\n", self.ram_memory);
                println!("loc : {:?}\n", self.loc_memory);
                println!("____________________________________________________\n");
            }

            Instruction::Error(error) => {
                println!("Error: {}", error);
            }

            Instruction::Assert => {
                if let Some(a) = self.stack.get(0) {
                    if *a == BaseElement::ONE {
                        self.stack.pop_front();
                    }
                }
            }

            Instruction::Assertz => {
                if let Some(a) = self.stack.get(0) {
                    if *a == BaseElement::ZERO {
                        self.stack.pop_front();
                    }
                }
            }

            Instruction::AssertEqW => {
                if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h)) = (
                    self.stack.get(0),
                    self.stack.get(1),
                    self.stack.get(2),
                    self.stack.get(3),
                    self.stack.get(4),
                    self.stack.get(5),
                    self.stack.get(6),
                    self.stack.get(7),
                ) {
                    if a == e && b == f && c == g && d == h {
                        for _ in 0..8 {
                            self.stack.pop_front();
                        }
                    }
                }
            }

            Instruction::AssertEq => {
                if let (Some(b), Some(a)) = (self.stack.get(0), self.stack.get(1)) {
                    if a == b {
                        self.stack.pop_front();
                        self.stack.pop_front();
                    }
                }
            }

            _ => {}
        }

        while self.stack.len() < 16 {
            self.stack.push_back(BaseElement::from(0_u64));
        }
    }

    pub fn exec(&mut self, name: &str) {
        self.add_instruction(Instruction::Exec(name.to_string()));
    }
}
