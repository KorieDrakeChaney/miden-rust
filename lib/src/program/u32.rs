use crate::{MidenProgram, Operand};

impl MidenProgram {
    pub fn u32checked_add(&mut self) {
        self.add_operand(Operand::U32CheckedAdd);
    }

    pub fn u32checked_add_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedAddImm(n));
    }

    pub fn u32overflowing_add(&mut self) {
        self.add_operand(Operand::U32OverflowingAdd);
    }

    pub fn u32overflowing_add_n(&mut self, n: u32) {
        self.add_operand(Operand::U32OverflowingAddImm(n));
    }

    pub fn u32wrapping_add(&mut self) {
        self.add_operand(Operand::U32WrappingAdd);
    }

    pub fn u32wrapping_add_n(&mut self, n: u32) {
        self.add_operand(Operand::U32WrappingAddImm(n));
    }

    pub fn u32checked_sub(&mut self) {
        self.add_operand(Operand::U32CheckedSub);
    }

    pub fn u32checked_sub_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedSubImm(n));
    }

    pub fn u32overflowing_sub(&mut self) {
        self.add_operand(Operand::U32OverflowingSub);
    }

    pub fn u32overflowing_sub_n(&mut self, n: u32) {
        self.add_operand(Operand::U32OverflowingSubImm(n));
    }

    pub fn u32wrapping_sub(&mut self) {
        self.add_operand(Operand::U32WrappingSub);
    }

    pub fn u32wrapping_sub_n(&mut self, n: u32) {
        self.add_operand(Operand::U32WrappingSubImm(n));
    }

    pub fn u32checked_mul(&mut self) {
        self.add_operand(Operand::U32CheckedMul);
    }

    pub fn u32checked_mul_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedMulImm(n));
    }

    pub fn u32overflowing_mul(&mut self) {
        self.add_operand(Operand::U32OverflowingMul);
    }

    pub fn u32overflowing_mul_n(&mut self, n: u32) {
        self.add_operand(Operand::U32OverflowingMulImm(n));
    }

    pub fn u32wrapping_mul(&mut self) {
        self.add_operand(Operand::U32WrappingMul);
    }

    pub fn u32wrapping_mul_n(&mut self, n: u32) {
        self.add_operand(Operand::U32WrappingMulImm(n));
    }

    pub fn u32checked_div(&mut self) {
        self.add_operand(Operand::U32CheckedDiv);
    }

    pub fn u32checked_div_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedDivImm(n));
    }

    pub fn u32unchecked_div(&mut self) {
        self.add_operand(Operand::U32UncheckedDiv);
    }

    pub fn u32unchecked_div_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedDivImm(n));
    }

    pub fn u32overflowing_madd(&mut self) {
        self.add_operand(Operand::U32OverflowingMadd);
    }

    pub fn u32wrapping_madd(&mut self) {
        self.add_operand(Operand::U32WrappingMadd);
    }

    pub fn u32checked_mod(&mut self) {
        self.add_operand(Operand::U32CheckedMod);
    }

    pub fn u32checked_mod_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedModImm(n));
    }

    pub fn u32unchecked_mod(&mut self) {
        self.add_operand(Operand::U32UncheckedMod);
    }

    pub fn u32unchecked_mod_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedModImm(n));
    }

    pub fn u32checked_divmod(&mut self) {
        self.add_operand(Operand::U32CheckedDivMod);
    }

    pub fn u32checked_divmod_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedDivModImm(n));
    }

    pub fn u32unchecked_divmod(&mut self) {
        self.add_operand(Operand::U32UncheckedDivMod);
    }

    pub fn u32unchecked_divmod_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedDivModImm(n));
    }

    // bitwise

    pub fn u32checked_and(&mut self) {
        self.add_operand(Operand::U32CheckedAnd);
    }

    pub fn u32checked_or(&mut self) {
        self.add_operand(Operand::U32CheckedOr);
    }

    pub fn u32checked_xor(&mut self) {
        self.add_operand(Operand::U32CheckedXor);
    }

    pub fn u32checked_not(&mut self) {
        self.add_operand(Operand::U32CheckedNot);
    }

    pub fn u32checked_shl(&mut self) {
        self.add_operand(Operand::U32CheckedShl);
    }

    pub fn u32checked_shl_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedShlImm(n));
    }

    pub fn u32unchecked_shl(&mut self) {
        self.add_operand(Operand::U32UncheckedShl);
    }

    pub fn u32unchecked_shl_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedShlImm(n));
    }

    pub fn u32checked_shr(&mut self) {
        self.add_operand(Operand::U32CheckedShr);
    }

    pub fn u32checked_shr_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedShrImm(n));
    }

    pub fn u32unchecked_shr(&mut self) {
        self.add_operand(Operand::U32UncheckedShr);
    }

    pub fn u32unchecked_shr_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedShrImm(n));
    }

    pub fn u32checked_rotl(&mut self) {
        self.add_operand(Operand::U32CheckedRotl);
    }

    pub fn u32checked_rotl_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedRotlImm(n));
    }

    pub fn u32unchecked_rotl(&mut self) {
        self.add_operand(Operand::U32UncheckedRotl);
    }

    pub fn u32unchecked_rotl_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedRotlImm(n));
    }

    pub fn u32checked_rotr(&mut self) {
        self.add_operand(Operand::U32CheckedRotr);
    }

    pub fn u32checked_rotr_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedRotrImm(n));
    }

    pub fn u32unchecked_rotr(&mut self) {
        self.add_operand(Operand::U32UncheckedRotr);
    }

    pub fn u32unchecked_rotr_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedRotrImm(n));
    }

    pub fn u32checked_popcnt(&mut self) {
        self.add_operand(Operand::U32CheckedPopcnt);
    }

    pub fn u32unchecked_popcnt(&mut self) {
        self.add_operand(Operand::U32UncheckedPopcnt);
    }

    // comparison

    pub fn u32checked_eq(&mut self) {
        self.add_operand(Operand::U32CheckedEq);
    }

    pub fn u32checked_eq_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedEqImm(n));
    }

    pub fn u32checked_neq(&mut self) {
        self.add_operand(Operand::U32CheckedNeq);
    }

    pub fn u32checked_neq_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedNeqImm(n));
    }

    pub fn u32checked_lt(&mut self) {
        self.add_operand(Operand::U32CheckedLt);
    }

    pub fn u32unchecked_lt(&mut self) {
        self.add_operand(Operand::U32UncheckedLt);
    }

    pub fn u32checked_lte(&mut self) {
        self.add_operand(Operand::U32CheckedLte);
    }

    pub fn u32unchecked_lte(&mut self) {
        self.add_operand(Operand::U32UncheckedLte);
    }

    pub fn u32checked_gt(&mut self) {
        self.add_operand(Operand::U32CheckedGt);
    }

    pub fn u32unchecked_gt(&mut self) {
        self.add_operand(Operand::U32UncheckedGt);
    }

    pub fn u32checked_gte(&mut self) {
        self.add_operand(Operand::U32CheckedGte);
    }

    pub fn u32unchecked_gte(&mut self) {
        self.add_operand(Operand::U32UncheckedGte);
    }

    pub fn u32checked_min(&mut self) {
        self.add_operand(Operand::U32CheckedMin);
    }

    pub fn u32unchecked_min(&mut self) {
        self.add_operand(Operand::U32UncheckedMin);
    }

    pub fn u32checked_max(&mut self) {
        self.add_operand(Operand::U32CheckedMax);
    }

    pub fn u32unchecked_max(&mut self) {
        self.add_operand(Operand::U32UncheckedMax);
    }
}
