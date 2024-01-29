use math::{fields::f64::BaseElement, FieldElement, StarkField};

use crate::{program::error::MidenProgramError, MidenProgram, Operand};

pub fn execute_boolean(program: &mut MidenProgram, operand: &Operand) {
    match operand {
        Operand::Or => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                if a_int != 1 && a_int != 0 {
                    program.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                        a.as_int(),
                    )));
                } else if b_int != 1 && b_int != 0 {
                    program.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                        b.as_int(),
                    )));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();
                    if a_int == 1 || b_int == 1 {
                        program.stack.push_front(BaseElement::ONE);
                    } else {
                        program.stack.push_front(BaseElement::ZERO);
                    }
                }
            }
        }

        Operand::And => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                if a_int != 1 && a_int != 0 {
                    program.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                        a.as_int(),
                    )));
                } else if b_int != 1 && b_int != 0 {
                    program.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                        b.as_int(),
                    )));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();
                    if a_int == 1 && b_int == 1 {
                        program.stack.push_front(BaseElement::ONE);
                    } else {
                        program.stack.push_front(BaseElement::ZERO);
                    }
                }
            }
        }

        Operand::Xor => {
            if let (Some(b), Some(a)) = (program.stack.get(0), program.stack.get(1)) {
                let a_int = a.as_int();
                let b_int = b.as_int();
                if a_int != 1 && a_int != 0 {
                    program.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                        a.as_int(),
                    )));
                } else if b_int != 1 && b_int != 0 {
                    program.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                        b.as_int(),
                    )));
                } else {
                    program.stack.pop_front();
                    program.stack.pop_front();
                    if a_int != 1 && (a_int == 1 || b_int == 1) {
                        program.stack.push_front(BaseElement::ONE);
                    } else {
                        program.stack.push_front(BaseElement::ZERO);
                    }
                }
            }
        }

        Operand::Not => {
            if let Some(a) = program.stack.get(0) {
                let a_int = a.as_int();
                if a_int != 1 && a_int != 0 {
                    program.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                        a.as_int(),
                    )));
                } else {
                    program.stack.pop_front();
                    if a_int == 0 {
                        program.stack.push_front(BaseElement::ONE);
                    } else {
                        program.stack.push_front(BaseElement::ZERO);
                    }
                }
            }
        }
        _ => {}
    }
}
