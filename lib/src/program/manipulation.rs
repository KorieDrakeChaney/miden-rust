use super::{MidenProgram, Operand};

impl MidenProgram {
    pub fn drop(&mut self) {
        self.add_operand(Operand::Drop);
    }

    pub fn swap(&mut self) {
        self.add_operand(Operand::Swap(1));
    }

    pub fn swap_n(&mut self, n: usize) {
        self.add_operand(Operand::Swap(n));
    }

    pub fn dup(&mut self) {
        self.add_operand(Operand::Dup(1));
    }

    pub fn dup_n(&mut self, n: usize) {
        self.add_operand(Operand::Dup(n));
    }

    pub fn swapw(&mut self) {
        self.add_operand(Operand::SwapW(1));
    }

    pub fn swapw_n(&mut self, n: usize) {
        self.add_operand(Operand::SwapW(n));
    }

    pub fn padw(&mut self) {
        self.add_operand(Operand::PadW);
    }

    pub fn movup_n(&mut self, n: usize) {
        self.add_operand(Operand::MovUp(n));
    }

    pub fn movupw_n(&mut self, n: usize) {
        self.add_operand(Operand::MovUpW(n));
    }

    pub fn movdn_n(&mut self, n: usize) {
        self.add_operand(Operand::MovDn(n));
    }

    pub fn movdnw_n(&mut self, n: usize) {
        self.add_operand(Operand::MovDnW(n));
    }
}
