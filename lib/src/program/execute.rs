use crate::program::block::execute_repeat;

use super::block::{execute_if_else, execute_while};
use super::{MidenProgram, Operand};
use math::{fields::f64::BaseElement, FieldElement, StarkField};
use std::collections::VecDeque;
use std::ops::Neg;

impl MidenProgram {
    pub fn execute_block(&mut self, block: &mut VecDeque<Operand>) {
        while let Some(op) = block.pop_front() {
            // println!("op: {:?} {:?}", op, self.stack);
            match op {
                Operand::IF => execute_if_else(self, block),
                Operand::WHILE => execute_while(self, block),
                Operand::REPEAT(n) => execute_repeat(n, self, block),
                _ => {
                    self.execute_operand(&op);
                }
            }
            while self.stack.len() < 16 {
                self.stack.push_back(BaseElement::from(0_u64));
            }
        }
    }

    pub fn execute_operand(&mut self, op: &Operand) {
        match op {
            Operand::Push(x) => self.stack.push_front(*x),
            Operand::Add => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    self.stack.push_front(a + b);
                }
            }
            Operand::Sub => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    self.stack.push_front(a - b);
                }
            }
            Operand::Mul => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    self.stack.push_front(a * b);
                }
            }
            Operand::Div => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    self.stack.push_front(a / b);
                }
            }
            Operand::AddImm(n) => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack.push_front(a + *n);
                }
            }
            Operand::SubImm(n) => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack.push_front(a - *n);
                }
            }
            Operand::MulImm(n) => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack.push_front(a * *n);
                }
            }
            Operand::DivImm(n) => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack.push_front(a / *n);
                }
            }
            Operand::Neg => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack.push_front(a.neg());
                }
            }
            Operand::Inv => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack.push_front(a.inv());
                }
            }
            Operand::Dup(n) => {
                if let Some(a) = self.stack.get(*n as usize) {
                    self.stack.push_front(*a);
                }
            }
            Operand::Drop => if let Some(_) = self.stack.pop_front() {},

            Operand::Swap(n) => {
                if self.stack.len() > 0 {
                    self.stack.swap(0, *n);
                }
            }
            Operand::AdvPush(x) => {
                if *x >= 1 && *x <= 16 {
                    if let Some(a) = self.advice_stack.get((*x - 1) as usize) {
                        self.stack.push_front(BaseElement::from(*a));
                    }
                }
            }
            Operand::Not => {
                if let Some(a) = self.stack.pop_front() {
                    if a == BaseElement::ZERO {
                        self.stack.push_front(BaseElement::ONE);
                    } else if a == BaseElement::ONE {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }
            Operand::PadW => {
                if self.stack.len() < 16 {
                    for _ in 0..4 {
                        self.stack.push_front(BaseElement::from(0_u64));
                    }
                }
            }
            Operand::SwapW(n) => {
                if self.stack.len() > 0 {
                    if *n > 1 && *n <= 3 {
                        while self.stack.len() < 8 {
                            self.stack.push_back(BaseElement::from(0_u64));
                        }
                        self.stack.swap(0, *n * 4);
                        self.stack.swap(1, *n * 4 + 1);
                        self.stack.swap(2, *n * 4 + 2);
                        self.stack.swap(3, *n * 4 + 3);
                    }
                }
            }
            Operand::MovDn(n) => {
                if *n < self.stack.len() {
                    if let Some(a) = self.stack.pop_front() {
                        self.stack.insert(*n, a);
                    }
                }
            }

            Operand::Exec(name) => {
                if let Some(mut program) = self.internal_programs.get(name).cloned() {
                    program.execute(self);
                }
            }

            Operand::MovDnW(n) => {
                if *n < self.stack.len() {
                    if let (Some(a), Some(b), Some(c), Some(d)) = (
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                        self.stack.pop_front(),
                    ) {
                        self.stack.insert(*n * 4, a);
                        self.stack.insert(*n * 4 + 1, b);
                        self.stack.insert(*n * 4 + 2, c);
                        self.stack.insert(*n * 4 + 3, d);
                    }
                }
            }

            Operand::MovUp(n) => {
                if *n < self.stack.len() {
                    if let Some(a) = self.stack.remove(*n) {
                        self.stack.push_front(a);
                    }
                }
            }

            Operand::MovUpW(n) => {
                if *n < self.stack.len() {
                    if let (Some(a), Some(b), Some(c), Some(d)) = (
                        self.stack.remove(*n * 4),
                        self.stack.remove(*n * 4 + 1),
                        self.stack.remove(*n * 4 + 2),
                        self.stack.remove(*n * 4 + 3),
                    ) {
                        self.stack.push_front(a);
                        self.stack.push_front(b);
                        self.stack.push_front(c);
                        self.stack.push_front(d);
                    }
                }
            }

            Operand::Eq => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    if a == b {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::EqImm(x) => {
                if let Some(a) = self.stack.pop_front() {
                    if a == BaseElement::from(*x) {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            // Comparisons
            Operand::Lt => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    if a.as_int() > b.as_int() {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::Gt => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    if a.as_int() < b.as_int() {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::Lte => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    if a.as_int() >= b.as_int() {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::Gte => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    if a.as_int() <= b.as_int() {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::Or => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    if a == BaseElement::ONE || b == BaseElement::ONE {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::And => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    if a == BaseElement::ONE && b == BaseElement::ONE {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::Xor => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    if a != b && (a == BaseElement::ONE || b == BaseElement::ONE) {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::Neq => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    if a != b {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::NeqImm(x) => {
                if let Some(a) = self.stack.pop_front() {
                    if a != BaseElement::from(*x) {
                        self.stack.push_front(BaseElement::ONE);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::Pow2 => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack
                        .push_front(BaseElement::from(2_u64).exp(a.into()));
                }
            }

            Operand::MemStore => {
                if let (Some(key), Some(a)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    self.ram_memory.insert(
                        key.as_int() as u32,
                        [BaseElement::ZERO, BaseElement::ZERO, BaseElement::ZERO, a],
                    );
                }
            }

            Operand::MemStoreImm(key) => {
                if let Some(a) = self.stack.pop_front() {
                    self.ram_memory.insert(
                        *key,
                        [BaseElement::ZERO, BaseElement::ZERO, BaseElement::ZERO, a],
                    );
                }
            }

            Operand::MemLoad => {
                if let Some(key) = self.stack.pop_front() {
                    if let Some([_, _, _, a]) = self.ram_memory.get(&(key.as_int() as u32)) {
                        self.stack.push_front(*a);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::MemLoadImm(key) => {
                if let Some([_, _, _, a]) = self.ram_memory.get(&key) {
                    self.stack.push_front(*a);
                } else {
                    self.stack.push_front(BaseElement::ZERO);
                }
            }

            Operand::MemLoadW => {
                if let (Some(key), Some(_), Some(_), Some(_), Some(_)) = (
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                ) {
                    if let Some([a, b, c, d]) = self.ram_memory.get(&(key.as_int() as u32)) {
                        self.stack.push_front(*d);
                        self.stack.push_front(*c);
                        self.stack.push_front(*b);
                        self.stack.push_front(*a);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                        self.stack.push_front(BaseElement::ZERO);
                        self.stack.push_front(BaseElement::ZERO);
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::MemLoadWImm(key) => {
                if let (Some(_), Some(_), Some(_), Some(_)) = (
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                ) {
                    if let Some([a, b, c, d]) = self.ram_memory.get(&key) {
                        self.stack.push_front(*d);
                        self.stack.push_front(*c);
                        self.stack.push_front(*b);
                        self.stack.push_front(*a);
                    } else {
                        self.stack.push_front(BaseElement::ZERO);
                        self.stack.push_front(BaseElement::ZERO);
                        self.stack.push_front(BaseElement::ZERO);
                        self.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::MemStoreW => {
                while self.stack.len() < 5 {
                    self.stack.push_back(BaseElement::ZERO);
                }
                if let (Some(key), Some(a), Some(b), Some(c), Some(d)) = (
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                ) {
                    self.ram_memory.insert(key.as_int() as u32, [a, b, c, d]);
                    self.stack.push_front(d);
                    self.stack.push_front(c);
                    self.stack.push_front(b);
                    self.stack.push_front(a);
                }
            }

            Operand::MemStoreWImm(key) => {
                while self.stack.len() < 4 {
                    self.stack.push_back(BaseElement::ZERO);
                }
                if let (Some(a), Some(b), Some(c), Some(d)) = (
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                    self.stack.pop_front(),
                ) {
                    self.ram_memory.insert(*key, [a, b, c, d]);
                    self.stack.push_front(d);
                    self.stack.push_front(c);
                    self.stack.push_front(b);
                    self.stack.push_front(a);
                }
            }

            Operand::PRINT(message) => {
                println!("____________________________________________________\n");
                println!("{}", message);
                println!("stack : {:?}\n", self.stack);
                println!("____________________________________________________\n");
            }

            Operand::Error(error) => {
                println!("Error: {}", error);
            }

            Operand::Exp => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    self.stack.push_front(a.exp(b.into()));
                }
            }

            Operand::ExpImm(n) => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack.push_front(a.exp(*n));
                }
            }

            Operand::Increment => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack.push_front(a + BaseElement::ONE);
                }
            }

            Operand::Decrement => {
                if let Some(a) = self.stack.pop_front() {
                    self.stack.push_front(a - BaseElement::ONE);
                }
            }

            Operand::U32UncheckedMod => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    self.stack
                        .push_front(BaseElement::from(b.as_int() % a.as_int()));
                }
            }

            Operand::U32CheckedMod => {
                if let (Some(a), Some(b)) = (self.stack.pop_front(), self.stack.pop_front()) {
                    self.stack
                        .push_front(BaseElement::from(b.as_int() % a.as_int()));
                }
            }

            _ => {}
        }

        while self.stack.len() < 16 {
            self.stack.push_back(BaseElement::from(0_u64));
        }
    }

    pub fn exec(&mut self, name: &str) {
        self.add_operand(Operand::Exec(name.to_string()));
    }
}
