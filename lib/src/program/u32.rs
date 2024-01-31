use crate::{Instruction, MidenProgram};

impl MidenProgram {
    pub fn u32checked_add(&mut self) {
        self.add_instruction(Instruction::U32CheckedAdd);
    }

    pub fn u32checked_add_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedAddImm(n));
    }

    pub fn u32overflowing_add(&mut self) {
        self.add_instruction(Instruction::U32OverflowingAdd);
    }

    pub fn u32overflowing_add_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32OverflowingAddImm(n));
    }

    pub fn u32wrapping_add(&mut self) {
        self.add_instruction(Instruction::U32WrappingAdd);
    }

    pub fn u32wrapping_add_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32WrappingAddImm(n));
    }

    pub fn u32checked_sub(&mut self) {
        self.add_instruction(Instruction::U32CheckedSub);
    }

    pub fn u32checked_sub_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedSubImm(n));
    }

    pub fn u32overflowing_sub(&mut self) {
        self.add_instruction(Instruction::U32OverflowingSub);
    }

    pub fn u32overflowing_sub_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32OverflowingSubImm(n));
    }

    pub fn u32wrapping_sub(&mut self) {
        self.add_instruction(Instruction::U32WrappingSub);
    }

    pub fn u32wrapping_sub_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32WrappingSubImm(n));
    }

    pub fn u32checked_mul(&mut self) {
        self.add_instruction(Instruction::U32CheckedMul);
    }

    pub fn u32checked_mul_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedMulImm(n));
    }

    pub fn u32overflowing_mul(&mut self) {
        self.add_instruction(Instruction::U32OverflowingMul);
    }

    pub fn u32overflowing_mul_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32OverflowingMulImm(n));
    }

    pub fn u32wrapping_mul(&mut self) {
        self.add_instruction(Instruction::U32WrappingMul);
    }

    pub fn u32wrapping_mul_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32WrappingMulImm(n));
    }

    pub fn u32checked_div(&mut self) {
        self.add_instruction(Instruction::U32CheckedDiv);
    }

    pub fn u32checked_div_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedDivImm(n));
    }

    pub fn u32unchecked_div(&mut self) {
        self.add_instruction(Instruction::U32UncheckedDiv);
    }

    pub fn u32unchecked_div_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedDivImm(n));
    }

    pub fn u32overflowing_madd(&mut self) {
        self.add_instruction(Instruction::U32OverflowingMadd);
    }

    pub fn u32wrapping_madd(&mut self) {
        self.add_instruction(Instruction::U32WrappingMadd);
    }

    pub fn u32checked_mod(&mut self) {
        self.add_instruction(Instruction::U32CheckedMod);
    }

    pub fn u32checked_mod_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedModImm(n));
    }

    pub fn u32unchecked_mod(&mut self) {
        self.add_instruction(Instruction::U32UncheckedMod);
    }

    pub fn u32unchecked_mod_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedModImm(n));
    }

    pub fn u32checked_divmod(&mut self) {
        self.add_instruction(Instruction::U32CheckedDivMod);
    }

    pub fn u32checked_divmod_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedDivModImm(n));
    }

    pub fn u32unchecked_divmod(&mut self) {
        self.add_instruction(Instruction::U32UncheckedDivMod);
    }

    pub fn u32unchecked_divmod_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedDivModImm(n));
    }

    // bitwise

    pub fn u32checked_and(&mut self) {
        self.add_instruction(Instruction::U32CheckedAnd);
    }

    pub fn u32checked_or(&mut self) {
        self.add_instruction(Instruction::U32CheckedOr);
    }

    pub fn u32checked_xor(&mut self) {
        self.add_instruction(Instruction::U32CheckedXor);
    }

    pub fn u32checked_not(&mut self) {
        self.add_instruction(Instruction::U32CheckedNot);
    }

    pub fn u32checked_shl(&mut self) {
        self.add_instruction(Instruction::U32CheckedShl);
    }

    pub fn u32checked_shl_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedShlImm(n));
    }

    pub fn u32unchecked_shl(&mut self) {
        self.add_instruction(Instruction::U32UncheckedShl);
    }

    pub fn u32unchecked_shl_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedShlImm(n));
    }

    pub fn u32checked_shr(&mut self) {
        self.add_instruction(Instruction::U32CheckedShr);
    }

    pub fn u32checked_shr_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedShrImm(n));
    }

    pub fn u32unchecked_shr(&mut self) {
        self.add_instruction(Instruction::U32UncheckedShr);
    }

    pub fn u32unchecked_shr_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedShrImm(n));
    }

    pub fn u32checked_rotl(&mut self) {
        self.add_instruction(Instruction::U32CheckedRotl);
    }

    pub fn u32checked_rotl_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedRotlImm(n));
    }

    pub fn u32unchecked_rotl(&mut self) {
        self.add_instruction(Instruction::U32UncheckedRotl);
    }

    pub fn u32unchecked_rotl_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedRotlImm(n));
    }

    pub fn u32checked_rotr(&mut self) {
        self.add_instruction(Instruction::U32CheckedRotr);
    }

    pub fn u32checked_rotr_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedRotrImm(n));
    }

    pub fn u32unchecked_rotr(&mut self) {
        self.add_instruction(Instruction::U32UncheckedRotr);
    }

    pub fn u32unchecked_rotr_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32UncheckedRotrImm(n));
    }

    pub fn u32checked_popcnt(&mut self) {
        self.add_instruction(Instruction::U32CheckedPopcnt);
    }

    pub fn u32unchecked_popcnt(&mut self) {
        self.add_instruction(Instruction::U32UncheckedPopcnt);
    }

    // comparison

    pub fn u32checked_eq(&mut self) {
        self.add_instruction(Instruction::U32CheckedEq);
    }

    pub fn u32checked_eq_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedEqImm(n));
    }

    pub fn u32checked_neq(&mut self) {
        self.add_instruction(Instruction::U32CheckedNeq);
    }

    pub fn u32checked_neq_n(&mut self, n: u32) {
        self.add_instruction(Instruction::U32CheckedNeqImm(n));
    }

    pub fn u32checked_lt(&mut self) {
        self.add_instruction(Instruction::U32CheckedLt);
    }

    pub fn u32unchecked_lt(&mut self) {
        self.add_instruction(Instruction::U32UncheckedLt);
    }

    pub fn u32checked_lte(&mut self) {
        self.add_instruction(Instruction::U32CheckedLte);
    }

    pub fn u32unchecked_lte(&mut self) {
        self.add_instruction(Instruction::U32UncheckedLte);
    }

    pub fn u32checked_gt(&mut self) {
        self.add_instruction(Instruction::U32CheckedGt);
    }

    pub fn u32unchecked_gt(&mut self) {
        self.add_instruction(Instruction::U32UncheckedGt);
    }

    pub fn u32checked_gte(&mut self) {
        self.add_instruction(Instruction::U32CheckedGte);
    }

    pub fn u32unchecked_gte(&mut self) {
        self.add_instruction(Instruction::U32UncheckedGte);
    }

    pub fn u32checked_min(&mut self) {
        self.add_instruction(Instruction::U32CheckedMin);
    }

    pub fn u32unchecked_min(&mut self) {
        self.add_instruction(Instruction::U32UncheckedMin);
    }

    pub fn u32checked_max(&mut self) {
        self.add_instruction(Instruction::U32CheckedMax);
    }

    pub fn u32unchecked_max(&mut self) {
        self.add_instruction(Instruction::U32UncheckedMax);
    }
}
