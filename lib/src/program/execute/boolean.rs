use math::{fields::f64::BaseElement, FieldElement};

use crate::{MidenProgram, Operand};

pub fn execute_boolean(program: &mut MidenProgram, operand: &Operand) {
    match operand {
        Operand::Or => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                if a == BaseElement::ONE || b == BaseElement::ONE {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Operand::And => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                if a == BaseElement::ONE && b == BaseElement::ONE {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
            println!("stack: {:?}", program.stack);
        }

        Operand::Xor => {
            if let (Some(b), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                if a != b && (a == BaseElement::ONE || b == BaseElement::ONE) {
                    program.stack.push_front(BaseElement::ONE);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }

        Operand::Not => {
            if let Some(a) = program.stack.pop_front() {
                if a == BaseElement::ZERO {
                    program.stack.push_front(BaseElement::ONE);
                } else if a == BaseElement::ONE {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }
        }
        _ => {}
    }
}
