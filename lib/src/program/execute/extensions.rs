use std::ops::Neg;

use miden::math::{Felt, FieldElement};

use crate::{Instruction, MidenProgram};

pub fn execute_extensions(program: &mut MidenProgram, operand: &Instruction) {
    match operand {
        Instruction::Ext2Add => {
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
        Instruction::Ext2Sub => {
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
        Instruction::Ext2Mul => {
            if let (Some(b1), Some(b0), Some(a1), Some(a0)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                program.stack.push_front((a0 + a1) * (b0 + b1));
                program
                    .stack
                    .push_front((a0 * b0) - Felt::from(2_u64) * (a1 * b1));
            }
        }
        Instruction::Ext2Div => {
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
        Instruction::Ext2Inv => {
            if let (Some(a1), Some(a0)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a0.inv());
                program.stack.push_front(a1.inv());
            }
        }
        Instruction::Ext2Neg => {
            if let (Some(a1), Some(a0)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program.stack.push_front(a0.neg());
                program.stack.push_front(a1.neg());
            }
        }
        _ => {}
    }
}
