use std::collections::VecDeque;

use math::{fields::f64::BaseElement, FieldElement};

use crate::{Instruction, MidenProgram, Program};

#[derive(Clone, Debug, PartialEq)]
pub struct Proc {
    pub name: String,
    pub operands: VecDeque<Instruction>,
    loc_count: u16,
}

impl Proc {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            operands: VecDeque::new(),
            loc_count: 0,
        }
    }

    pub fn get_masm(&self) -> String {
        let mut masm: String = String::new();
        masm.push_str(&format!("proc.{}", self.name));
        if self.loc_count > 0 {
            masm.push_str(&format!(".{}", self.loc_count));
        }
        masm.push_str(&format!("\n"));

        let mut scope = 1;
        for op in self.operands.iter() {
            match op {
                &Instruction::IF | &Instruction::WHILE | &Instruction::REPEAT(_) => {
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));

                    scope += 1;
                }
                &Instruction::ELSE => {
                    scope -= 1;
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                    scope += 1;
                }
                &Instruction::END => {
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

        masm
    }
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
    /// use rust_masm::{MidenProgram, EmptyProgram, Proc};
    ///
    /// fn main() {
    ///     let mut procedure = Proc::new("ifElseExample");
    ///     procedure.push(1);
    ///     let mut if_program = EmptyProgram::new();
    ///     if_program.increment();
    ///     let mut else_program = EmptyProgram::new();
    ///     else_program.decrement();
    ///     procedure.if_else_block(&mut if_program, &mut else_program);
    ///     let mut program = MidenProgram::new();
    ///     program.push(1);
    ///     program.add_proc(procedure);
    ///     program.exec("ifElseExample");
    /// }
    /// ```
    pub fn if_else_block<'a, T>(&'a mut self, if_program: &mut T, else_program: &mut T)
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

        self.add_operands(temp_stack);
    }

    /// Constructs a new `while` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `program` - An instance of a type implementing the `Program` trait, which provides the instructions for the `while` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram, Proc};
    ///
    /// fn main() {
    ///     let mut procedure = Proc::new("whileExample");
    ///     procedure.push(1);
    ///     let mut while_program = EmptyProgram::new();
    ///     while_program.increment();
    ///     while_program.dup();
    ///     while_program.neq_n(10);
    ///     procedure.while_block(&mut while_program);
    ///     let mut program = MidenProgram::new();
    ///     program.push(1);
    ///     program.add_proc(procedure);
    ///     program.exec("whileExample");
    /// }
    /// ```
    pub fn while_block<'a, T>(&mut self, program: &mut T)
    where
        T: Program + 'a,
    {
        let mut instructions = program.get_instructions();
        instructions.push_front(Instruction::WHILE);
        instructions.push_back(Instruction::END);
        self.add_operands(instructions);
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
    /// use rust_masm::{MidenProgram, EmptyProgram, Proc};
    ///
    /// fn main(){
    ///     let mut procedure = Proc::new("repeatExample");
    ///     procedure.push(1);
    ///     let mut repeat_program = EmptyProgram::new();
    ///     repeat_program.increment();
    ///     procedure.repeat(3, &mut repeat_program);
    ///     let mut program = MidenProgram::new();
    ///     program.push(1);
    ///     program.add_proc(procedure);
    ///     program.exec("repeatExample");
    /// }
    /// ```
    pub fn repeat<'a, T>(&mut self, n: usize, program: &mut T)
    where
        T: Program + 'a,
    {
        let mut instructions = program.get_instructions();
        instructions.push_front(Instruction::REPEAT(n));
        instructions.push_back(Instruction::END);
        self.add_operands(instructions);
    }

    /// Constructs a new `if` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `program` - A mutable reference to the program for the `if` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram, Proc};
    ///
    /// fn main(){
    ///     let mut procedure = Proc::new("ifExample");
    ///     procedure.push(1);
    ///     let mut if_program = EmptyProgram::new();
    ///     if_program.increment();
    ///     procedure.if_block(&mut if_program);
    ///     let mut program = MidenProgram::new();
    ///     program.push(1);
    ///     program.add_proc(procedure);
    ///     program.exec("ifExample");
    /// }
    /// ```
    pub fn if_block<'a, T>(&mut self, program: &mut T)
    where
        T: Program + 'a,
    {
        let mut instructions = program.get_instructions();
        instructions.push_front(Instruction::IF);
        instructions.push_back(Instruction::END);
        self.add_operands(instructions);
    }

    pub fn add_instruction(&mut self, operand: Instruction) {
        match &operand {
            Instruction::LocLoad(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
            }
            Instruction::LocLoadW(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
            }
            Instruction::LocStore(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
            }

            Instruction::LocStoreW(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
            }

            _ => {}
        }
        self.operands.push_back(operand);
    }

    pub fn add_operands(&mut self, operands: VecDeque<Instruction>) {
        self.operands.append(&mut operands.into_iter().collect());
    }

    pub fn execute_block(
        &mut self,
        program: &mut MidenProgram,
        block: &mut VecDeque<Instruction>,
        scope: usize,
    ) {
        let mut index = scope;
        while let Some(operand) = block.pop_front() {
            println!("operand: {:?}", operand);
            match program.is_valid_operand(&operand) {
                Some(error) => {
                    if let Some(op) = self.operands.get_mut(index) {
                        match op {
                            Instruction::Error(_) | Instruction::CommentedOut(_) => {}
                            _ => {
                                *op = Instruction::CommentedOut(op.to_string());
                                self.operands
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

            match operand {
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
                        if let Some(n) = program.stack.pop_front() {
                            if n == BaseElement::ONE {
                                self.execute_block(program, &mut while_block.clone(), index);
                            } else {
                                break 'while_loop;
                            }
                        }
                    }
                    index += while_block.len() + 1;
                }
                Instruction::IF => {
                    if let Some(n) = program.stack.pop_front() {
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
                                self.execute_block(program, &mut if_block, index);
                            }
                        } else {
                            if else_block.len() > 0 {
                                self.execute_block(program, &mut else_block, index);
                            }
                        }

                        index += if_block.len() + else_block.len() + 1;

                        if if_scope_count > 0 {
                            index += 1;
                        }
                    }
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
                        self.execute_block(program, &mut repeat_operands.clone(), index);
                    }

                    index += repeat_operands.len() + 1;
                }
                _ => {
                    self.execute_operand(program, &operand);
                }
            }
        }
    }

    pub fn execute(&mut self, program: &mut MidenProgram) {
        self.execute_block(program, &mut self.operands.clone(), 0);
    }

    pub fn execute_operand(&mut self, program: &mut MidenProgram, operand: &Instruction) {
        match operand {
            Instruction::LocLoad(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
                if let Some([_, _, _, a]) = program.loc_memory.get(&key) {
                    program.stack.push_front(*a);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }

            Instruction::LocLoadW(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
                if let (Some(_), Some(_), Some(_), Some(_)) = (
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                ) {
                    if let Some([a, b, c, d]) = program.loc_memory.get(&key) {
                        program.stack.push_front(*d);
                        program.stack.push_front(*c);
                        program.stack.push_front(*b);
                        program.stack.push_front(*a);
                    } else {
                        program.stack.push_front(BaseElement::ZERO);
                        program.stack.push_front(BaseElement::ZERO);
                        program.stack.push_front(BaseElement::ZERO);
                        program.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Instruction::LocStore(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
                if let Some(a) = program.stack.pop_front() {
                    program.loc_memory.insert(
                        *key,
                        [BaseElement::ZERO, BaseElement::ZERO, BaseElement::ZERO, a],
                    );
                }
            }

            Instruction::LocStoreW(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
                if let (Some(a), Some(b), Some(c), Some(d)) = (
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                ) {
                    program.loc_memory.insert(*key, [a, b, c, d]);
                    program.stack.push_front(d);
                    program.stack.push_front(c);
                    program.stack.push_front(b);
                    program.stack.push_front(a);
                }
            }
            _ => {
                program.execute_operand(&operand);
            }
        }
    }

    /// Pushes a print instruction to the stack with a message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to print.
    pub fn print(&mut self, message: &str) {
        self.add_instruction(Instruction::PRINT(message.to_string()));
    }

    /// Pushes `Drop` command onto the stack.
    pub fn drop(&mut self) {
        self.add_instruction(Instruction::Drop);
    }

    /// Pushes `Swap` command onto the stack.
    pub fn swap(&mut self) {
        self.add_instruction(Instruction::Swap(1));
    }

    /// Pushes `Swap` command with value `n` onto the stack.
    ///     
    /// # Arguments
    ///
    /// * `n` - The value to swap
    pub fn swap_n(&mut self, n: usize) {
        self.add_instruction(Instruction::Swap(n));
    }

    /// Pushes `Dup` command onto the stack.
    pub fn dup(&mut self) {
        self.add_instruction(Instruction::Dup(1));
    }

    /// Pushes `Dup` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to duplicate.
    pub fn dup_n(&mut self, n: usize) {
        self.add_instruction(Instruction::Dup(n));
    }

    /// Pushes `SwapW` command onto the stack.
    pub fn swapw(&mut self) {
        self.add_instruction(Instruction::SwapW(1));
    }

    /// Pushes `SwapW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to swap.
    pub fn swapw_n(&mut self, n: usize) {
        self.add_instruction(Instruction::SwapW(n));
    }

    /// Pushes `PadW` command onto the stack.
    pub fn padw(&mut self) {
        self.add_instruction(Instruction::PadW);
    }

    /// Pushes `MovUp` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move up.
    pub fn movup_n(&mut self, n: usize) {
        self.add_instruction(Instruction::MovUp(n));
    }

    /// Pushes `MovUpW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move up.
    pub fn movupw_n(&mut self, n: usize) {
        self.add_instruction(Instruction::MovUpW(n));
    }

    /// Pushes `MovDn` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move down.
    pub fn movdn_n(&mut self, n: usize) {
        self.add_instruction(Instruction::MovDn(n));
    }

    /// Pushes `MovDnW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move down.
    pub fn movdnw_n(&mut self, n: usize) {
        self.add_instruction(Instruction::MovDnW(n));
    }

    /// Pushes `Add` command onto the stack.
    pub fn add(&mut self) {
        self.add_instruction(Instruction::Add);
    }

    /// Pushes `AddImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to add.
    pub fn add_n(&mut self, n: u64) {
        self.add_instruction(Instruction::AddImm(BaseElement::from(n)));
    }

    /// Pushes `Sub` command onto the stack.
    pub fn sub(&mut self) {
        self.add_instruction(Instruction::Sub);
    }

    /// Pushes `SubImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to subtract.
    pub fn sub_n(&mut self, n: u64) {
        self.add_instruction(Instruction::SubImm(BaseElement::from(n)));
    }

    /// Pushes `Mul` command onto the stack.
    pub fn mul(&mut self) {
        self.add_instruction(Instruction::Mul);
    }

    /// Pushes `MulImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to multiply.
    pub fn mul_n(&mut self, n: u64) {
        self.add_instruction(Instruction::MulImm(BaseElement::from(n)));
    }

    /// Pushes `Div` command onto the stack.
    pub fn div(&mut self) {
        self.add_instruction(Instruction::Div);
    }

    /// Pushes `DivImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to divide.
    pub fn div_n(&mut self, n: u64) {
        self.add_instruction(Instruction::DivImm(BaseElement::from(n)));
    }

    /// Pushes `Neg` command onto the stack.
    pub fn neg(&mut self) {
        self.add_instruction(Instruction::Neg);
    }

    /// Pushes `Inv` command onto the stack.
    pub fn inv(&mut self) {
        self.add_instruction(Instruction::Inv);
    }

    /// Pushes `Pow2` command onto the stack.
    pub fn pow2(&mut self) {
        self.add_instruction(Instruction::Pow2);
    }

    /// Pushes `Exp` command onto the stack.
    pub fn exp(&mut self) {
        self.add_instruction(Instruction::Exp);
    }

    /// Pushes `ExpImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to exponentiate.
    pub fn exp_n(&mut self, n: u64) {
        self.add_instruction(Instruction::ExpImm(n));
    }

    /// Pushes `And` command onto the stack.
    pub fn and(&mut self) {
        self.add_instruction(Instruction::And);
    }

    /// Pushes `Or` command onto the stack.
    pub fn or(&mut self) {
        self.add_instruction(Instruction::Or);
    }

    /// Pushes `Xor` command onto the stack.
    pub fn xor(&mut self) {
        self.add_instruction(Instruction::Xor);
    }

    pub fn not(&mut self) {
        self.add_instruction(Instruction::Not);
    }

    /// Pushes `Eq` command onto the stack.
    pub fn eq(&mut self) {
        self.add_instruction(Instruction::Eq);
    }

    /// Pushes `EqImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to compare for equality.
    pub fn eq_n(&mut self, n: u64) {
        self.add_instruction(Instruction::EqImm(BaseElement::from(n)));
    }

    /// Pushes `Neq` command onto the stack.
    pub fn neq(&mut self) {
        self.add_instruction(Instruction::Neq);
    }

    /// Pushes `NeqImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to compare for inequality.
    pub fn neq_n(&mut self, n: u64) {
        self.add_instruction(Instruction::NeqImm(BaseElement::from(n)));
    }

    /// Pushes `Lt` command onto the stack.
    pub fn lt(&mut self) {
        self.add_instruction(Instruction::Lt);
    }

    /// Pushes `Lte` command onto the stack.
    pub fn lte(&mut self) {
        self.add_instruction(Instruction::Lte);
    }

    /// Pushes `Gt` command onto the stack.
    pub fn gt(&mut self) {
        self.add_instruction(Instruction::Gt);
    }

    /// Pushes `Gte` command onto the stack.
    pub fn gte(&mut self) {
        self.add_instruction(Instruction::Gte);
    }

    /// Pushes `IsOdd` command onto the stack.
    pub fn is_odd(&mut self) {
        self.add_instruction(Instruction::IsOdd);
    }

    /// Pushes `Eqw` command onto the stack.
    pub fn eqw(&mut self) {
        self.add_instruction(Instruction::EqW);
    }

    /// Pushes `MemLoad` command onto the stack.
    pub fn mem_load(&mut self) {
        self.add_instruction(Instruction::MemLoad);
    }

    /// Pushes `MemLoadImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from memory.
    pub fn mem_load_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemLoadImm(n));
    }

    /// Pushes `MemLoadW` command onto the stack.
    pub fn mem_load_w(&mut self) {
        self.add_instruction(Instruction::MemLoadW);
    }

    /// Pushes `MemLoadWImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from memory.
    pub fn mem_load_w_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemLoadWImm(n));
    }

    /// Pushes `MemStore` command onto the stack.
    pub fn mem_store(&mut self) {
        self.add_instruction(Instruction::MemStore);
    }

    /// Pushes `MemStoreImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in memory.
    pub fn mem_store_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemStoreImm(n));
    }

    /// Pushes `MemStoreW` command onto the stack.
    pub fn mem_store_w(&mut self) {
        self.add_instruction(Instruction::MemStoreW);
    }

    /// Pushes `MemStoreWImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in memory.
    pub fn mem_store_w_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemStoreWImm(n));
    }

    /// Pushes `LocLoad` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from local storage.
    pub fn loc_load(&mut self, n: u16) {
        self.add_instruction(Instruction::LocLoad(n));
    }

    /// Pushes `LocLoadW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from local storage.
    pub fn loc_load_w(&mut self, n: u16) {
        self.add_instruction(Instruction::LocLoadW(n));
    }

    /// Pushes `LocStore` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in local storage.
    pub fn loc_store(&mut self, n: u16) {
        self.add_instruction(Instruction::LocStore(n));
    }

    /// Pushes `LocStoreW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in local storage.
    pub fn loc_store_w(&mut self, n: u16) {
        self.add_instruction(Instruction::LocStoreW(n));
    }

    /// Pushes `Push` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to push onto the stack.
    pub fn push(&mut self, n: u64) {
        self.add_instruction(Instruction::Push(BaseElement::from(n)));
    }

    /// Pushes `AdvPush` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to push onto the stack.
    pub fn adv_push(&mut self, n: usize) {
        self.add_instruction(Instruction::AdvPush(n));
    }

    pub fn exec(&mut self, name: &str) {
        self.add_instruction(Instruction::Exec(name.to_string()));
    }

    /// Pushes `Increment` command onto the stack.
    pub fn increment(&mut self) {
        self.add_instruction(Instruction::Increment);
    }

    /// Pushes `Decrement` command onto the stack.
    pub fn decrement(&mut self) {
        self.add_instruction(Instruction::Decrement);
    }

    pub fn u32checked_add(&mut self) {
        self.add_instruction(Instruction::U32CheckedAdd);
    }

    pub fn u32checked_add_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedAddImm(n));
    }

    pub fn u32overflowing_add(&mut self) {
        self.add_instruction(Instruction::U32OverflowingAdd);
    }

    pub fn u32overflowing_add_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32OverflowingAddImm(n));
    }

    pub fn u32wrapping_add(&mut self) {
        self.add_instruction(Instruction::U32WrappingAdd);
    }

    pub fn u32wrapping_add_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32WrappingAddImm(n));
    }

    pub fn u32checked_sub(&mut self) {
        self.add_instruction(Instruction::U32CheckedSub);
    }

    pub fn u32checked_sub_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedSubImm(n));
    }

    pub fn u32overflowing_sub(&mut self) {
        self.add_instruction(Instruction::U32OverflowingSub);
    }

    pub fn u32overflowing_sub_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32OverflowingSubImm(n));
    }

    pub fn u32wrapping_sub(&mut self) {
        self.add_instruction(Instruction::U32WrappingSub);
    }

    pub fn u32wrapping_sub_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32WrappingSubImm(n));
    }

    pub fn u32checked_mul(&mut self) {
        self.add_instruction(Instruction::U32CheckedMul);
    }

    pub fn u32checked_mul_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedMulImm(n));
    }

    pub fn u32overflowing_mul(&mut self) {
        self.add_instruction(Instruction::U32OverflowingMul);
    }

    pub fn u32overflowing_mul_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32OverflowingMulImm(n));
    }

    pub fn u32wrapping_mul(&mut self) {
        self.add_instruction(Instruction::U32WrappingMul);
    }

    pub fn u32wrapping_mul_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32WrappingMulImm(n));
    }

    pub fn u32checked_div(&mut self) {
        self.add_instruction(Instruction::U32CheckedDiv);
    }

    pub fn u32checked_div_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedDivImm(n));
    }

    pub fn u32unchecked_div(&mut self) {
        self.add_instruction(Instruction::U32UncheckedDiv);
    }

    pub fn u32unchecked_div_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedDivImm(n));
    }

    pub fn u32overflowing_madd(&mut self) {
        self.add_instruction(Instruction::U32OverflowingMadd);
    }

    pub fn u32wrapping_madd(&mut self) {
        self.add_instruction(Instruction::U32WrappingMadd);
    }

    pub fn u32checked_mod(&mut self) {
        self.add_instruction(Instruction::U32CheckedMod);
    }

    pub fn u32checked_mod_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedModImm(n));
    }

    pub fn u32unchecked_mod(&mut self) {
        self.add_instruction(Instruction::U32UncheckedMod);
    }

    pub fn u32unchecked_mod_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedModImm(n));
    }

    pub fn u32checked_divmod(&mut self) {
        self.add_instruction(Instruction::U32CheckedDivMod);
    }

    pub fn u32checked_divmod_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedDivModImm(n));
    }

    pub fn u32unchecked_divmod(&mut self) {
        self.add_instruction(Instruction::U32UncheckedDivMod);
    }

    pub fn u32unchecked_divmod_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedDivModImm(n));
    }

    // bitwise

    pub fn u32checked_and(&mut self) {
        self.add_instruction(Instruction::U32CheckedAnd);
    }

    pub fn u32checked_or(&mut self) {
        self.add_instruction(Instruction::U32CheckedOr);
    }

    pub fn u32checked_xor(&mut self) {
        self.add_instruction(Instruction::U32CheckedXor);
    }

    pub fn u32checked_not(&mut self) {
        self.add_instruction(Instruction::U32CheckedNot);
    }

    pub fn u32checked_shl(&mut self) {
        self.add_instruction(Instruction::U32CheckedShl);
    }

    pub fn u32checked_shl_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedShlImm(n));
    }

    pub fn u32unchecked_shl(&mut self) {
        self.add_instruction(Instruction::U32UncheckedShl);
    }

    pub fn u32unchecked_shl_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedShlImm(n));
    }

    pub fn u32checked_shr(&mut self) {
        self.add_instruction(Instruction::U32CheckedShr);
    }

    pub fn u32checked_shr_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedShrImm(n));
    }

    pub fn u32unchecked_shr(&mut self) {
        self.add_instruction(Instruction::U32UncheckedShr);
    }

    pub fn u32unchecked_shr_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedShrImm(n));
    }

    pub fn u32checked_rotl(&mut self) {
        self.add_instruction(Instruction::U32CheckedRotl);
    }

    pub fn u32checked_rotl_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedRotlImm(n));
    }

    pub fn u32unchecked_rotl(&mut self) {
        self.add_instruction(Instruction::U32UncheckedRotl);
    }

    pub fn u32unchecked_rotl_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedRotlImm(n));
    }

    pub fn u32checked_rotr(&mut self) {
        self.add_instruction(Instruction::U32CheckedRotr);
    }

    pub fn u32checked_rotr_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedRotrImm(n));
    }

    pub fn u32unchecked_rotr(&mut self) {
        self.add_instruction(Instruction::U32UncheckedRotr);
    }

    pub fn u32unchecked_rotr_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedRotrImm(n));
    }

    pub fn u32checked_popcnt(&mut self) {
        self.add_instruction(Instruction::U32CheckedPopcnt);
    }

    pub fn u32unchecked_popcnt(&mut self) {
        self.add_instruction(Instruction::U32UncheckedPopcnt);
    }

    // comparison

    pub fn u32checked_eq(&mut self) {
        self.add_instruction(Instruction::U32CheckedEq);
    }

    pub fn u32checked_eq_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedEqImm(n));
    }

    pub fn u32checked_neq(&mut self) {
        self.add_instruction(Instruction::U32CheckedNeq);
    }

    pub fn u32checked_neq_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedNeqImm(n));
    }

    pub fn u32checked_lt(&mut self) {
        self.add_instruction(Instruction::U32CheckedLt);
    }

    pub fn u32unchecked_lt(&mut self) {
        self.add_instruction(Instruction::U32UncheckedLt);
    }

    pub fn u32checked_lte(&mut self) {
        self.add_instruction(Instruction::U32CheckedLte);
    }

    pub fn u32unchecked_lte(&mut self) {
        self.add_instruction(Instruction::U32UncheckedLte);
    }

    pub fn u32checked_gt(&mut self) {
        self.add_instruction(Instruction::U32CheckedGt);
    }

    pub fn u32unchecked_gt(&mut self) {
        self.add_instruction(Instruction::U32UncheckedGt);
    }

    pub fn u32checked_gte(&mut self) {
        self.add_instruction(Instruction::U32CheckedGte);
    }

    pub fn u32unchecked_gte(&mut self) {
        self.add_instruction(Instruction::U32UncheckedGte);
    }

    pub fn u32checked_min(&mut self) {
        self.add_instruction(Instruction::U32CheckedMin);
    }

    pub fn u32unchecked_min(&mut self) {
        self.add_instruction(Instruction::U32UncheckedMin);
    }

    pub fn u32checked_max(&mut self) {
        self.add_instruction(Instruction::U32CheckedMax);
    }

    pub fn u32unchecked_max(&mut self) {
        self.add_instruction(Instruction::U32UncheckedMax);
    }

    pub fn add_program<F>(&mut self, program: F)
    where
        F: FnOnce() -> VecDeque<Instruction>,
    {
        self.add_operands(program());
    }
}

impl Program for Proc {
    fn get_instructions(&self) -> VecDeque<Instruction> {
        self.operands.clone()
    }
}
