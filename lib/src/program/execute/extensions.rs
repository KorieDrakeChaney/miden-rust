use std::ops::Neg;

use math::{fields::f64::BaseElement, FieldElement};

use crate::{MidenProgram, Operand};

pub fn execute_extensions(program: &mut MidenProgram, operand: &Operand) {
    match operand {
        Operand::Ext2Add => {
            if let (Some(b1), Some(b0), Some(a1), Some(a0)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                program.stack.push_front(a0 + b0);
                program.stack.push_front(a1 + b1);
            }
        }
        Operand::Ext2Sub => {
            if let (Some(b1), Some(b0), Some(a1), Some(a0)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                program.stack.push_front(a0 - b0);
                program.stack.push_front(a1 - b1);
            }
        }
        Operand::Ext2Mul => {
            if let (Some(b1), Some(b0), Some(a1), Some(a0)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                program.stack.push_front((a0 + a1) * (b0 + b1));
                program
                    .stack
                    .push_front((a0 * b0) - BaseElement::from(2_u64) * (a1 * b1));
            }
        }
        Operand::Ext2Div => {
            if let (Some(b1), Some(b0), Some(a1), Some(a0)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                program.stack.push_front(a0 * b0.inv());
                program.stack.push_front(a1 * b1.inv());
            }
        }
        Operand::Ext2Inv => {
            if let (Some(a1), Some(a0)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a0.inv());
                program.stack.push_front(a1.inv());
            }
        }
        Operand::Ext2Neg => {
            if let (Some(a1), Some(a0)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a0.neg());
                program.stack.push_front(a1.neg());
            }
        }
        _ => {}
    }
}
