use math::{fields::f64::BaseElement, FieldElement};

use crate::{MidenProgram, Operand};

pub fn execute_manipulation(program: &mut MidenProgram, operand: &Operand) {
    match operand {
        Operand::Push(x) => program.stack.push_front(*x),

        Operand::Dup(n) => {
            if let Some(a) = program.stack.get(*n as usize) {
                program.stack.push_front(*a);
            }
        }
        Operand::Drop => if let Some(_) = program.stack.pop_front() {},

        Operand::DropW => {
            for _ in 0..4 {
                program.stack.pop_front();
                program.stack.pop_front();
                program.stack.pop_front();
                program.stack.pop_front();
            }
        }

        Operand::Swap(n) => {
            program.stack.swap(0, *n);
        }

        Operand::PadW => {
            for _ in 0..4 {
                program.stack.push_front(BaseElement::ZERO);
            }
        }
        Operand::SwapW(n) => {
            program.stack.swap(0, *n * 4);
            program.stack.swap(1, *n * 4 + 1);
            program.stack.swap(2, *n * 4 + 2);
            program.stack.swap(3, *n * 4 + 3);
        }
        Operand::MovDn(n) => {
            if let Some(a) = program.stack.pop_front() {
                program.stack.insert(*n, a);
            }
        }

        Operand::MovDnW(n) => {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                program.stack.insert(*n * 4, a);
                program.stack.insert(*n * 4 + 1, b);
                program.stack.insert(*n * 4 + 2, c);
                program.stack.insert(*n * 4 + 3, d);
            }
        }

        Operand::MovUp(n) => {
            if let Some(a) = program.stack.remove(*n) {
                program.stack.push_front(a);
            }
        }

        Operand::MovUpW(n) => {
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                program.stack.remove(*n * 4),
                program.stack.remove(*n * 4),
                program.stack.remove(*n * 4),
                program.stack.remove(*n * 4),
            ) {
                program.stack.push_front(d);
                program.stack.push_front(c);
                program.stack.push_front(b);
                program.stack.push_front(a);
            }
        }

        _ => {}
    }
}
