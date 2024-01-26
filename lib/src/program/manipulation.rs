use super::{error::MidenProgramError, MidenProgram, Operand};

impl MidenProgram {
    /// Drops the top value from the stack.
    pub fn drop(&mut self) {
        self.add_operand(Operand::Drop);
    }
    /// Swaps the top two values on the stack.
    pub fn swap(&mut self) {
        self.add_operand(Operand::Swap(1));
    }

    /// Swaps the top value with the nth value on the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The index to swap with the top value. Must be in the range 1-15.
    pub fn swap_n(&mut self, n: usize) {
        if n < 1 || n > 15 {
            self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                "swap".to_string(),
                n,
                1,
                15,
            )))
        } else {
            self.add_operand(Operand::Swap(n));
        }
    }

    /// Duplicates the top value then pushes it onto the stack.
    pub fn dup(&mut self) {
        self.add_operand(Operand::Dup(0));
    }

    /// Duplicates the nth value then pushes it onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The index to duplicate. Must be in the range 0-15.
    pub fn dup_n(&mut self, n: usize) {
        if n > 15 {
            self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                "dup".to_string(),
                n,
                0,
                15,
            )))
        } else {
            self.add_operand(Operand::Dup(n));
        }
    }

    /// Swaps the top two words on the stack.
    pub fn swapw(&mut self) {
        self.add_operand(Operand::SwapW(1));
    }

    /// Swaps the top word with the nth word on the stack.
    /// # Arguments
    /// * `n` - The index to swap with the top word. Must be in the range 1-3
    pub fn swapw_n(&mut self, n: usize) {
        if n < 1 || n > 3 {
            self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                "swapw".to_string(),
                n,
                1,
                3,
            )))
        } else {
            self.add_operand(Operand::SwapW(n));
        }
    }

    /// Pads the stack with a zero word.
    pub fn padw(&mut self) {
        self.add_operand(Operand::PadW);
    }

    /// Moves the nth value up to the top of the stack.
    /// # Arguments
    /// * `n` - The index to move up. Must be in the range 1-15.
    pub fn movup_n(&mut self, n: usize) {
        if n < 1 || n > 15 {
            self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                "movup".to_string(),
                n,
                1,
                15,
            )))
        } else {
            self.add_operand(Operand::MovUp(n));
        }
    }

    /// Moves the nth word up to the top of the stack.
    /// # Arguments
    /// * `n` - The index to move up. Must be in the range 1-3.
    pub fn movupw_n(&mut self, n: usize) {
        if n < 1 || n > 3 {
            self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                "movupw".to_string(),
                n,
                1,
                3,
            )))
        } else {
            self.add_operand(Operand::MovUpW(n));
        }
    }

    /// Moves the first value down to the nth position on the stack.
    /// # Arguments
    /// * `n` - The index to move down. Must be in the range 2-15.
    pub fn movdn_n(&mut self, n: usize) {
        if n < 2 || n > 15 {
            self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                "movdn".to_string(),
                n,
                2,
                15,
            )))
        } else {
            self.add_operand(Operand::MovDn(n));
        }
    }

    /// Moves the first word down to the nth position on the stack.
    /// # Arguments
    /// * `n` - The index to move down. Must be in the range 2-3.
    pub fn movdnw_n(&mut self, n: usize) {
        if n < 2 || n > 3 {
            self.add_operand(Operand::Error(MidenProgramError::InvalidParameter(
                "movdnw".to_string(),
                n,
                2,
                3,
            )))
        } else {
            self.add_operand(Operand::MovDnW(n));
        }
    }
}
