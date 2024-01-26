use std::collections::VecDeque;

use math::fields::f64::BaseElement;

use super::{operand::Operand, Program};

pub struct EmptyProgram {
    stack_operands: VecDeque<Operand>,
}

impl EmptyProgram {
    pub fn new() -> Self {
        Self {
            stack_operands: VecDeque::new(),
        }
    }

    pub fn add_operand(&mut self, operand: Operand) {
        self.stack_operands.push_back(operand);
    }

    pub fn add_operands(&mut self, operands: &VecDeque<Operand>) {
        for op in operands {
            self.stack_operands.push_back(op.clone());
        }
    }

    pub fn if_else<T>(&mut self, if_op: T, else_op: T)
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

    pub fn while_block<T>(&mut self, block: T)
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

    pub fn repeat<T>(&mut self, n: usize, program: T)
    where
        T: Program,
    {
        let mut operands = program.get_operands();
        operands.push_front(Operand::REPEAT(n));
        operands.push_back(Operand::END);
        self.add_operands(&operands);
    }

    pub fn print(&mut self, message: &str) {
        self.stack_operands
            .push_back(Operand::PRINT(message.to_string()));
    }

    pub fn append_proc(&mut self, program: EmptyProgram) {
        self.add_operands(&program.stack_operands);
    }

    pub fn drop(&mut self) {
        self.add_operand(Operand::Drop);
    }

    pub fn swap(&mut self) {
        self.add_operand(Operand::Swap(1));
    }

    pub fn swap_n(&mut self, n: usize) {
        self.add_operand(Operand::Swap(n));
    }

    pub fn dup(&mut self) {
        self.add_operand(Operand::Dup(1));
    }

    pub fn dup_n(&mut self, n: usize) {
        self.add_operand(Operand::Dup(n));
    }

    pub fn swapw(&mut self) {
        self.add_operand(Operand::SwapW(1));
    }

    pub fn swapw_n(&mut self, n: usize) {
        self.add_operand(Operand::SwapW(n));
    }

    pub fn padw(&mut self) {
        self.add_operand(Operand::PadW);
    }

    pub fn movup_n(&mut self, n: usize) {
        self.add_operand(Operand::MovUp(n));
    }

    pub fn movupw_n(&mut self, n: usize) {
        self.add_operand(Operand::MovUpW(n));
    }

    pub fn movdn_n(&mut self, n: usize) {
        self.add_operand(Operand::MovDn(n));
    }

    pub fn movdnw_n(&mut self, n: usize) {
        self.add_operand(Operand::MovDnW(n));
    }

    pub fn add(&mut self) {
        self.add_operand(Operand::Add);
    }

    pub fn add_n(&mut self, n: u64) {
        self.add_operand(Operand::AddImm(BaseElement::from(n)));
    }

    pub fn sub(&mut self) {
        self.add_operand(Operand::Sub);
    }

    pub fn sub_n(&mut self, n: u64) {
        self.add_operand(Operand::SubImm(BaseElement::from(n)));
    }

    pub fn mul(&mut self) {
        self.add_operand(Operand::Mul);
    }

    pub fn mul_n(&mut self, n: u64) {
        self.add_operand(Operand::MulImm(BaseElement::from(n)));
    }

    pub fn div(&mut self) {
        self.add_operand(Operand::Div);
    }

    pub fn div_n(&mut self, n: u64) {
        self.add_operand(Operand::DivImm(BaseElement::from(n)));
    }

    pub fn neg(&mut self) {
        self.add_operand(Operand::Neg);
    }

    pub fn inv(&mut self) {
        self.add_operand(Operand::Inv);
    }

    pub fn pow2(&mut self) {
        self.add_operand(Operand::Pow2);
    }

    pub fn exp(&mut self) {
        self.add_operand(Operand::Exp);
    }

    pub fn exp_n(&mut self, n: u64) {
        self.add_operand(Operand::ExpImm(n));
    }

    pub fn and(&mut self) {
        self.add_operand(Operand::And);
    }

    pub fn or(&mut self) {
        self.add_operand(Operand::Or);
    }

    pub fn xor(&mut self) {
        self.add_operand(Operand::Xor);
    }

    pub fn not(&mut self) {
        self.add_operand(Operand::Not);
    }

    pub fn eq(&mut self) {
        self.add_operand(Operand::Eq);
    }

    pub fn eq_n(&mut self, n: u64) {
        self.add_operand(Operand::EqImm(BaseElement::from(n)));
    }

    pub fn neq(&mut self) {
        self.add_operand(Operand::Neq);
    }

    pub fn neq_n(&mut self, n: u64) {
        self.add_operand(Operand::NeqImm(BaseElement::from(n)));
    }

    pub fn lt(&mut self) {
        self.add_operand(Operand::Lt);
    }

    pub fn lte(&mut self) {
        self.add_operand(Operand::Lte);
    }

    pub fn gt(&mut self) {
        self.add_operand(Operand::Gt);
    }

    pub fn gte(&mut self) {
        self.add_operand(Operand::Gte);
    }

    pub fn is_odd(&mut self) {
        self.add_operand(Operand::IsOdd);
    }

    pub fn eqw(&mut self) {
        self.add_operand(Operand::Eqw);
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

    pub fn loc_load(&mut self, n: u16) {
        self.add_operand(Operand::LocLoad(n));
    }

    pub fn loc_load_w(&mut self, n: u16) {
        self.add_operand(Operand::LocLoadW(n));
    }

    pub fn loc_store(&mut self, n: u16) {
        self.add_operand(Operand::LocStore(n));
    }

    pub fn loc_store_w(&mut self, n: u16) {
        self.add_operand(Operand::LocStoreW(n));
    }

    pub fn exec(&mut self, name: &str) {
        self.add_operand(Operand::Exec(name.to_string()));
    }

    pub fn push(&mut self, n: u64) {
        self.add_operand(Operand::Push(BaseElement::from(n)));
    }

    pub fn adv_push(&mut self, n: usize) {
        self.add_operand(Operand::AdvPush(n));
    }
}

impl Program for EmptyProgram {
    fn get_operands(&self) -> VecDeque<Operand> {
        self.stack_operands.clone()
    }
}
