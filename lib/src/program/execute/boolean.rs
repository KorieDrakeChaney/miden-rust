use math::{fields::f64::BaseElement, FieldElement, StarkField};

use crate::{Instruction, MidenProgram};

pub fn execute_boolean(program: &mut MidenProgram, operand: &Instruction) {
    match operand {
        Instruction::Or => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if a_int == 1 || b_int == 1 {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::And => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();

                if a_int == 1 && b_int == 1 {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::Xor => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                if a_int != 1 && (a_int == 1 || b_int == 1) {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Instruction::Not => {
            if let Some(a) = program.stack.pop_front() {
                let a_int = a.as_int();
                if a_int == 0 {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }
        _ => {}
    }
}
