use math::fields::f64::BaseElement;

use super::{Instruction, MidenProgram};

impl MidenProgram {
    /// Adds the top two values on the stack.
    pub fn add(&mut self) {
        self.add_instruction(Instruction::Add);
    }

    /// Adds `n` to the top value on the stack.
    /// # Arguments
    ///
    /// * `n` - The number to add to the top value on the stack.
    pub fn add_n(&mut self, n: u64) {
        self.add_instruction(Instruction::AddImm(BaseElement::from(n)));
    }

    /// Subtracts the first value from the second value on the stack.
    pub fn sub(&mut self) {
        self.add_instruction(Instruction::Sub);
    }

    /// Subtracts the top value by `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The number to subtract from the top value on the stack.
    pub fn sub_n(&mut self, n: u64) {
        self.add_instruction(Instruction::SubImm(BaseElement::from(n)));
    }

    /// Multiplies the top two values on the stack.
    pub fn mul(&mut self) {
        self.add_instruction(Instruction::Mul);
    }

    /// Multiplies the top value on the stack by `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The number to multiply the top value on the stack by.
    pub fn mul_n(&mut self, n: u64) {
        self.add_instruction(Instruction::MulImm(BaseElement::from(n)));
    }

    /// Divides the first value into the second value on the stack.
    pub fn div(&mut self) {
        self.add_instruction(Instruction::Div);
    }

    /// Divides the top value on the stack by `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The number to divide the top value on the stack by.
    pub fn div_n(&mut self, n: u64) {
        self.add_instruction(Instruction::DivImm(BaseElement::from(n)));
    }

    /// Negates the top value on the stack.
    pub fn neg(&mut self) {
        self.add_instruction(Instruction::Neg);
    }

    /// Inverts the top value on the stack.
    pub fn inv(&mut self) {
        self.add_instruction(Instruction::Inv);
    }

    /// Pushes 2 to the power of the first value on the stack.
    pub fn pow2(&mut self) {
        self.add_instruction(Instruction::Pow2);
    }

    /// Raises the number `e` to the power of the top value on the stack.
    pub fn exp(&mut self) {
        self.add_instruction(Instruction::Exp);
    }

    /// Raises the number `e` to the power of `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The exponent to raise `e` to.
    pub fn exp_n(&mut self, n: u64) {
        self.add_instruction(Instruction::ExpImm(n));
    }

    /// Performs a bitwise AND operation on the top two values on the stack.
    ///
    /// # Errors
    ///
    /// Returns `MidenProgramError::NotBinaryValue` if either of the top two values on the stack is not a binary value.
    pub fn and(&mut self) {
        self.add_instruction(Instruction::And);
    }

    /// Performs a bitwise OR operation on the top two values on the stack.
    ///
    /// # Errors
    ///
    /// Returns `MidenProgramError::NotBinaryValue` if either of the top two values on the stack is not a binary value.
    pub fn or(&mut self) {
        self.add_instruction(Instruction::Or);
    }

    /// Performs a bitwise XOR operation on the top two values on the stack.
    ///
    /// # Errors
    ///
    /// Returns `MidenProgramError::NotBinaryValue` if either of the top two values on the stack is not a binary value.
    pub fn xor(&mut self) {
        self.add_instruction(Instruction::Xor);
    }

    /// Performs a bitwise NOT operation on the top value on the stack.
    ///
    /// # Errors
    ///
    /// Returns `MidenProgramError::NotBinaryValue` if the top value on the stack is not a binary value.
    pub fn not(&mut self) {
        self.add_instruction(Instruction::Not);
    }

    /// Checks if the top two values on the stack are equal.
    pub fn eq(&mut self) {
        self.add_instruction(Instruction::Eq);
    }

    /// Checks if the top value on the stack is equal to `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The number to compare the top value on the stack to.
    pub fn eq_n(&mut self, n: u64) {
        self.add_instruction(Instruction::EqImm(BaseElement::from(n)));
    }

    /// Checks if the top two values on the stack are not equal.
    pub fn neq(&mut self) {
        self.add_instruction(Instruction::Neq);
    }

    /// Checks if the top value on the stack is not equal to `n`.
    ///
    /// # Arguments
    ///
    /// * `n` - The number to compare the top value on the stack to.
    pub fn neq_n(&mut self, n: u64) {
        self.add_instruction(Instruction::NeqImm(BaseElement::from(n)));
    }

    /// Checks if the second top value on the stack is less than the top value.
    pub fn lt(&mut self) {
        self.add_instruction(Instruction::Lt);
    }

    /// Checks if the second top value on the stack is less than or equal to the top value.
    pub fn lte(&mut self) {
        self.add_instruction(Instruction::Lte);
    }

    /// Checks if the second top value on the stack is greater than the top value.
    pub fn gt(&mut self) {
        self.add_instruction(Instruction::Gt);
    }

    /// Checks if the second top value on the stack is greater than or equal to the top value.
    pub fn gte(&mut self) {
        self.add_instruction(Instruction::Gte);
    }

    /// Checks if the top value on the stack is odd.
    pub fn is_odd(&mut self) {
        self.add_instruction(Instruction::IsOdd);
    }

    /// Checks if the top value on the stack is equal to the width of the field.
    pub fn eqw(&mut self) {
        self.add_instruction(Instruction::EqW);
    }

    /// Increments the top value on the stack by 1.
    pub fn increment(&mut self) {
        self.add_instruction(Instruction::Increment);
    }

    /// Decrements the top value on the stack by 1.
    pub fn decrement(&mut self) {
        self.add_instruction(Instruction::Decrement);
    }

    /// Assumes a b c d are top of the stack, and performs the following operations:
    /// b = b + d
    /// a = a + c
    /// pushes a and b to the stack
    pub fn ext2add(&mut self) {
        self.add_instruction(Instruction::Ext2Add);
    }
    /// Assumes a b c d are top of the stack, and performs the following operations:
    /// a = (a + b) * (c + d)
    /// b = (a * c) - 2 * (b * d)
    /// pushes a and b to the stack
    pub fn ext2mul(&mut self) {
        self.add_instruction(Instruction::Ext2Mul);
    }
    /// Assumes a b c d are top of the stack, and performs the following operations:
    /// a = a - c
    /// b = b - d
    /// pushes a and b to the stack
    pub fn ext2sub(&mut self) {
        self.add_instruction(Instruction::Ext2Sub);
    }
    /// Assumes a b are top of the stack, and performs the following operations:
    /// a = -a
    /// b = -b
    /// pushes a and b to the stack
    pub fn ext2neg(&mut self) {
        self.add_instruction(Instruction::Ext2Neg);
    }

    /// Assumes a b are top of the stack, and performs the following operations:
    /// a = 1/a
    /// b = 1/b
    /// pushes a and b to the stack
    pub fn ext2inv(&mut self) {
        self.add_instruction(Instruction::Ext2Inv);
    }
    /// Assumes a b c d are top of the stack, and performs the following operations:
    /// a = a / c
    /// b = b / d
    /// pushes a and b to the stack
    pub fn ext2div(&mut self) {
        self.add_instruction(Instruction::Ext2Div);
    }
}
