use std::collections::{HashMap, VecDeque};

use math::{fields::f64::BaseElement, FieldElement};

use crate::{MidenProgram, Operand};

use super::block::{execute_if_else, execute_repeat, execute_while};
#[derive(Clone, Debug, PartialEq)]
pub struct Proc {
    pub name: String,
    operands: VecDeque<Operand>,
    loc_count: u16,
    loc_memory: HashMap<u16, [BaseElement; 4]>,
}

impl Proc {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            operands: VecDeque::new(),
            loc_count: 0,
            loc_memory: HashMap::new(),
        }
    }

    pub fn get_masm(&self) -> String {
        let mut masm: String = String::new();
        masm.push_str(&format!("proc.{}", self.name));
        if self.loc_count > 0 {
            masm.push_str(&format!(".{}", self.loc_count));
        }
        masm.push_str(&format!("\n"));

        let mut scope = 1;
        for op in self.operands.iter() {
            match op {
                &Operand::IF | &Operand::WHILE | &Operand::REPEAT(_) => {
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                    scope += 1;
                }
                &Operand::ELSE => {
                    scope -= 1;
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                    scope += 1;
                }
                &Operand::END => {
                    scope -= 1;
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                }

                &Operand::PRINT(_) | &Operand::Error(_) => {}
                _ => {
                    let tabs = "\t".repeat(scope);
                    masm.push_str(&format!("{}{}\n", tabs, op));
                }
            }
        }

        masm.push_str(&format!("end\n\n"));

        masm
    }

    pub fn add_operand(&mut self, operand: Operand) {
        match &operand {
            Operand::LocStore(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
            }

            Operand::LocStoreW(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
            }

            _ => {}
        }
        self.operands.push_back(operand);
    }

    pub fn add_operands(&mut self, operands: VecDeque<Operand>) {
        self.operands.append(&mut operands.into_iter().collect());
    }

    pub fn execute_block(&mut self, program: &mut MidenProgram, block: &mut VecDeque<Operand>) {
        while let Some(operand) = block.pop_front() {
            match operand {
                Operand::WHILE => execute_while(program, block),
                Operand::IF => execute_if_else(program, block),
                Operand::REPEAT(n) => execute_repeat(n, program, block),
                _ => {
                    self.execute_operand(program, &operand);
                }
            }
        }
    }

    pub fn execute(&mut self, program: &mut MidenProgram) {
        self.execute_block(program, &mut self.operands.clone());
    }

    pub fn execute_operand(&mut self, program: &mut MidenProgram, operand: &Operand) {
        match operand {
            Operand::LocLoad(key) => {
                if let Some([_, _, _, a]) = self.loc_memory.get(&key) {
                    program.stack.push_front(*a);
                } else {
                    program.stack.push_front(BaseElement::ZERO);
                }
            }

            Operand::LocLoadW(key) => {
                if let (Some(_), Some(_), Some(_), Some(_)) = (
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                ) {
                    if let Some([a, b, c, d]) = self.loc_memory.get(&key) {
                        program.stack.push_front(*d);
                        program.stack.push_front(*c);
                        program.stack.push_front(*b);
                        program.stack.push_front(*a);
                    } else {
                        program.stack.push_front(BaseElement::ZERO);
                        program.stack.push_front(BaseElement::ZERO);
                        program.stack.push_front(BaseElement::ZERO);
                        program.stack.push_front(BaseElement::ZERO);
                    }
                }
            }

            Operand::LocStore(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
                if let Some(a) = program.stack.pop_front() {
                    self.loc_memory.insert(
                        *key,
                        [BaseElement::ZERO, BaseElement::ZERO, BaseElement::ZERO, a],
                    );
                }
            }

            Operand::LocStoreW(key) => {
                if *key >= self.loc_count {
                    self.loc_count = key + 1;
                }
                if let (Some(a), Some(b), Some(c), Some(d)) = (
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                    program.stack.pop_front(),
                ) {
                    self.loc_memory.insert(*key, [a, b, c, d]);
                    program.stack.push_front(d);
                    program.stack.push_front(c);
                    program.stack.push_front(b);
                    program.stack.push_front(a);
                }
            }
            _ => {
                program.execute_operand(&operand);
            }
        }
    }

    pub fn get_operands(&mut self) -> VecDeque<Operand> {
        std::mem::take(&mut self.operands)
    }
}
