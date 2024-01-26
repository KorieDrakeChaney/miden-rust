use math::fields::f64::BaseElement;

use super::{error::MidenProgramError, utils::is_binary, MidenProgram, Operand};

impl MidenProgram {
    pub fn add(&mut self) {
        self.add_operand(Operand::Add);
    }

    pub fn add_n(&mut self, n: u64) {
        self.add_operand(Operand::AddImm(BaseElement::from(n)));
    }

    pub fn sub(&mut self) {
        self.add_operand(Operand::Sub);
    }

    pub fn sub_n(&mut self, n: u64) {
        self.add_operand(Operand::SubImm(BaseElement::from(n)));
    }

    pub fn mul(&mut self) {
        self.add_operand(Operand::Mul);
    }

    pub fn mul_n(&mut self, n: u64) {
        self.add_operand(Operand::MulImm(BaseElement::from(n)));
    }

    pub fn div(&mut self) {
        self.add_operand(Operand::Div);
    }

    pub fn div_n(&mut self, n: u64) {
        self.add_operand(Operand::DivImm(BaseElement::from(n)));
    }

    pub fn neg(&mut self) {
        self.add_operand(Operand::Neg);
    }

    pub fn inv(&mut self) {
        self.add_operand(Operand::Inv);
    }

    pub fn pow2(&mut self) {
        self.add_operand(Operand::Pow2);
    }

    pub fn exp(&mut self) {
        self.add_operand(Operand::Exp);
    }

    pub fn exp_n(&mut self, n: u64) {
        self.add_operand(Operand::ExpImm(n));
    }

    pub fn and(&mut self) {
        if !is_binary(&self.stack[0]) {
            self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                (self.stack[0]).into(),
            )))
        } else if !is_binary(&self.stack[1]) {
            self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                (self.stack[1]).into(),
            )))
        } else {
            self.add_operand(Operand::And);
        }
    }

    pub fn or(&mut self) {
        if !is_binary(&self.stack[0]) {
            self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                (self.stack[0]).into(),
            )))
        } else if !is_binary(&self.stack[1]) {
            self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                (self.stack[1]).into(),
            )))
        } else {
            self.add_operand(Operand::Or);
        }
    }

    pub fn xor(&mut self) {
        if !is_binary(&self.stack[0]) {
            self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                (self.stack[0]).into(),
            )))
        } else if !is_binary(&self.stack[1]) {
            self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                (self.stack[1]).into(),
            )))
        } else {
            self.add_operand(Operand::Xor);
        }
    }

    pub fn not(&mut self) {
        if !is_binary(&self.stack[0]) {
            self.add_operand(Operand::Error(MidenProgramError::NotBinaryValue(
                (self.stack[0]).into(),
            )))
        } else {
            self.add_operand(Operand::Not);
        }
    }

    pub fn eq(&mut self) {
        self.add_operand(Operand::Eq);
    }

    pub fn eq_n(&mut self, n: u64) {
        self.add_operand(Operand::EqImm(BaseElement::from(n)));
    }

    pub fn neq(&mut self) {
        self.add_operand(Operand::Neq);
    }

    pub fn neq_n(&mut self, n: u64) {
        self.add_operand(Operand::NeqImm(BaseElement::from(n)));
    }

    pub fn lt(&mut self) {
        self.add_operand(Operand::Lt);
    }

    pub fn lte(&mut self) {
        self.add_operand(Operand::Lte);
    }

    pub fn gt(&mut self) {
        self.add_operand(Operand::Gt);
    }

    pub fn gte(&mut self) {
        self.add_operand(Operand::Gte);
    }

    pub fn is_odd(&mut self) {
        self.add_operand(Operand::IsOdd);
    }

    pub fn eqw(&mut self) {
        self.add_operand(Operand::Eqw);
    }
}
