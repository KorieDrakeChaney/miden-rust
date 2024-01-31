use math::{fields::f64::BaseElement, FieldElement, StarkField};

use crate::{Instruction, MidenProgram};

pub fn execute_comparison(program: &mut MidenProgram, operand: &Instruction) {
    match operand {
        Instruction::Eq => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                if a == b {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::EqImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                if a == BaseElement::from(*b) {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::EqW => {
            if let (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h)) = (
                program.stack.get(0),
                program.stack.get(1),
                program.stack.get(2),
                program.stack.get(3),
                program.stack.get(4),
                program.stack.get(5),
                program.stack.get(6),
                program.stack.get(7),
            ) {
                if a == e && b == f && c == g && d == h {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::Lt => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                if a.as_int() < b.as_int() {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::Gt => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                if a.as_int() > b.as_int() {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::Lte => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                if a.as_int() <= b.as_int() {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::Gte => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                if a.as_int() >= b.as_int() {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::Neq => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                if a != b {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::NeqImm(b) => {
            if let Some(a) = program.stack.pop_front() {
                if a != BaseElement::from(*b) {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::IsOdd => {
            if let Some(a) = program.stack.pop_front() {
                if a.as_int() % 2 == 1 {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }
        _ => {}
    }
}
