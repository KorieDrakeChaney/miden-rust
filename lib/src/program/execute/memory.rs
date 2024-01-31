use miden::math::{Felt, FieldElement, StarkField};

use crate::{Instruction, MidenProgram};

pub fn execute_memory(program: &mut MidenProgram, operand: &Instruction) {
    match operand {
        Instruction::MemStore => {
            if let (Some(key), Some(a)) = (program.stack.pop_front(), program.stack.pop_front()) {
                program
                    .ram_memory
                    .insert(key.as_int() as u32, [Felt::ZERO, Felt::ZERO, Felt::ZERO, a]);
            }
            println!("ram: {:?}", program.ram_memory);
        }

        Instruction::MemStoreImm(key) => {
            if let Some(a) = program.stack.pop_front() {
                program
                    .ram_memory
                    .insert(*key, [Felt::ZERO, Felt::ZERO, Felt::ZERO, a]);
            }
        }

        Instruction::MemLoad => {
            if let Some(key) = program.stack.pop_front() {
                if let Some([_, _, _, a]) = program.ram_memory.get(&(key.as_int() as u32)) {
                    program.stack.push_front(*a);
                } else {
                    program.stack.push_front(Felt::ZERO);
                }
            }
        }

        Instruction::MemLoadImm(key) => {
            if let Some([_, _, _, a]) = program.ram_memory.get(&key) {
                program.stack.push_front(*a);
            } else {
                program.stack.push_front(Felt::ZERO);
            }
        }

        Instruction::MemLoadW => {
            if let (Some(key), Some(_), Some(_), Some(_), Some(_)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                if let Some([a, b, c, d]) = program.ram_memory.get(&(key.as_int() as u32)) {
                    program.stack.push_front(*d);
                    program.stack.push_front(*c);
                    program.stack.push_front(*b);
                    program.stack.push_front(*a);
                } else {
                    program.stack.push_front(Felt::ZERO);
                    program.stack.push_front(Felt::ZERO);
                    program.stack.push_front(Felt::ZERO);
                    program.stack.push_front(Felt::ZERO);
                }
            }
        }

        Instruction::MemLoadWImm(key) => {
            if let (Some(_), Some(_), Some(_), Some(_)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                if let Some([a, b, c, d]) = program.ram_memory.get(&key) {
                    program.stack.push_front(*d);
                    program.stack.push_front(*c);
                    program.stack.push_front(*b);
                    program.stack.push_front(*a);
                } else {
                    program.stack.push_front(Felt::ZERO);
                    program.stack.push_front(Felt::ZERO);
                    program.stack.push_front(Felt::ZERO);
                    program.stack.push_front(Felt::ZERO);
                }
            }
        }

        Instruction::MemStoreW => {
            while program.stack.len() < 5 {
                program.stack.push_back(Felt::ZERO);
            }
            if let (Some(key), Some(a), Some(b), Some(c), Some(d)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                program.ram_memory.insert(key.as_int() as u32, [a, b, c, d]);
                program.stack.push_front(d);
                program.stack.push_front(c);
                program.stack.push_front(b);
                program.stack.push_front(a);
            }
        }

        Instruction::MemStoreWImm(key) => {
            while program.stack.len() < 4 {
                program.stack.push_back(Felt::ZERO);
            }
            if let (Some(a), Some(b), Some(c), Some(d)) = (
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
                program.stack.pop_front(),
            ) {
                program.ram_memory.insert(*key, [a, b, c, d]);
                program.stack.push_front(d);
                program.stack.push_front(c);
                program.stack.push_front(b);
                program.stack.push_front(a);
            }
        }

        _ => {}
    }
}
