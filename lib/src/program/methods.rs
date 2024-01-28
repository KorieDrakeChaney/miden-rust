use std::collections::VecDeque;

use math::fields::f64::BaseElement;

use crate::Operand;

pub trait Methods {
    fn add_operand(&mut self, operand: Operand);
    /// Returns stack operands.
    ///
    /// # Returns
    ///
    /// A `VecDeque` containing the operands on the stack.
    fn get_operands(&self) -> VecDeque<Operand>;
    /// Adds a program to the stack operands.
    ///
    /// # Arguments
    ///
    /// * `program` - A function that returns a `VecDeque` of operands.
    fn add_program<F>(&mut self, program: F)
    where
        F: FnOnce() -> VecDeque<Operand>;
    /// Pushes a print instruction to the stack with a message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to print.
    fn print(&mut self, message: &str) {
        self.add_operand(Operand::PRINT(message.to_string()));
    }

    /// Pushes `Drop` command onto the stack.
    fn drop(&mut self) {
        self.add_operand(Operand::Drop);
    }

    /// Pushes `Swap` command onto the stack.
    fn swap(&mut self) {
        self.add_operand(Operand::Swap(1));
    }

    /// Pushes `Swap` command with value `n` onto the stack.
    ///     
    /// # Arguments
    ///
    /// * `n` - The value to swap
    fn swap_n(&mut self, n: usize) {
        self.add_operand(Operand::Swap(n));
    }

    /// Pushes `Dup` command onto the stack.
    fn dup(&mut self) {
        self.add_operand(Operand::Dup(1));
    }

    /// Pushes `Dup` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to duplicate.
    fn dup_n(&mut self, n: usize) {
        self.add_operand(Operand::Dup(n));
    }

    /// Pushes `SwapW` command onto the stack.
    fn swapw(&mut self) {
        self.add_operand(Operand::SwapW(1));
    }

    /// Pushes `SwapW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to swap.
    fn swapw_n(&mut self, n: usize) {
        self.add_operand(Operand::SwapW(n));
    }

    /// Pushes `PadW` command onto the stack.
    fn padw(&mut self) {
        self.add_operand(Operand::PadW);
    }

    /// Pushes `MovUp` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move up.
    fn movup_n(&mut self, n: usize) {
        self.add_operand(Operand::MovUp(n));
    }

    /// Pushes `MovUpW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move up.
    fn movupw_n(&mut self, n: usize) {
        self.add_operand(Operand::MovUpW(n));
    }

    /// Pushes `MovDn` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move down.
    fn movdn_n(&mut self, n: usize) {
        self.add_operand(Operand::MovDn(n));
    }

    /// Pushes `MovDnW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move down.
    fn movdnw_n(&mut self, n: usize) {
        self.add_operand(Operand::MovDnW(n));
    }

    /// Pushes `Add` command onto the stack.
    fn add(&mut self) {
        self.add_operand(Operand::Add);
    }

    /// Pushes `AddImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to add.
    fn add_n(&mut self, n: u64) {
        self.add_operand(Operand::AddImm(BaseElement::from(n)));
    }

    /// Pushes `Sub` command onto the stack.
    fn sub(&mut self) {
        self.add_operand(Operand::Sub);
    }

    /// Pushes `SubImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to subtract.
    fn sub_n(&mut self, n: u64) {
        self.add_operand(Operand::SubImm(BaseElement::from(n)));
    }

    /// Pushes `Mul` command onto the stack.
    fn mul(&mut self) {
        self.add_operand(Operand::Mul);
    }

    /// Pushes `MulImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to multiply.
    fn mul_n(&mut self, n: u64) {
        self.add_operand(Operand::MulImm(BaseElement::from(n)));
    }

    /// Pushes `Div` command onto the stack.
    fn div(&mut self) {
        self.add_operand(Operand::Div);
    }

    /// Pushes `DivImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to divide.
    fn div_n(&mut self, n: u64) {
        self.add_operand(Operand::DivImm(BaseElement::from(n)));
    }

    /// Pushes `Neg` command onto the stack.
    fn neg(&mut self) {
        self.add_operand(Operand::Neg);
    }

    /// Pushes `Inv` command onto the stack.
    fn inv(&mut self) {
        self.add_operand(Operand::Inv);
    }

    /// Pushes `Pow2` command onto the stack.
    fn pow2(&mut self) {
        self.add_operand(Operand::Pow2);
    }

    /// Pushes `Exp` command onto the stack.
    fn exp(&mut self) {
        self.add_operand(Operand::Exp);
    }

    /// Pushes `ExpImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to exponentiate.
    fn exp_n(&mut self, n: u64) {
        self.add_operand(Operand::ExpImm(n));
    }

    /// Pushes `And` command onto the stack.
    fn and(&mut self) {
        self.add_operand(Operand::And);
    }

    /// Pushes `Or` command onto the stack.
    fn or(&mut self) {
        self.add_operand(Operand::Or);
    }

    /// Pushes `Xor` command onto the stack.
    fn xor(&mut self) {
        self.add_operand(Operand::Xor);
    }

    fn not(&mut self) {
        self.add_operand(Operand::Not);
    }

    /// Pushes `Eq` command onto the stack.
    fn eq(&mut self) {
        self.add_operand(Operand::Eq);
    }

    /// Pushes `EqImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to compare for equality.
    fn eq_n(&mut self, n: u64) {
        self.add_operand(Operand::EqImm(BaseElement::from(n)));
    }

    /// Pushes `Neq` command onto the stack.
    fn neq(&mut self) {
        self.add_operand(Operand::Neq);
    }

    /// Pushes `NeqImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to compare for inequality.
    fn neq_n(&mut self, n: u64) {
        self.add_operand(Operand::NeqImm(BaseElement::from(n)));
    }

    /// Pushes `Lt` command onto the stack.
    fn lt(&mut self) {
        self.add_operand(Operand::Lt);
    }

    /// Pushes `Lte` command onto the stack.
    fn lte(&mut self) {
        self.add_operand(Operand::Lte);
    }

    /// Pushes `Gt` command onto the stack.
    fn gt(&mut self) {
        self.add_operand(Operand::Gt);
    }

    /// Pushes `Gte` command onto the stack.
    fn gte(&mut self) {
        self.add_operand(Operand::Gte);
    }

    /// Pushes `IsOdd` command onto the stack.
    fn is_odd(&mut self) {
        self.add_operand(Operand::IsOdd);
    }

    /// Pushes `Eqw` command onto the stack.
    fn eqw(&mut self) {
        self.add_operand(Operand::EqW);
    }

    /// Pushes `MemLoad` command onto the stack.
    fn mem_load(&mut self) {
        self.add_operand(Operand::MemLoad);
    }

    /// Pushes `MemLoadImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from memory.
    fn mem_load_n(&mut self, n: u32) {
        self.add_operand(Operand::MemLoadImm(n));
    }

    /// Pushes `MemLoadW` command onto the stack.
    fn mem_load_w(&mut self) {
        self.add_operand(Operand::MemLoadW);
    }

    /// Pushes `MemLoadWImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from memory.
    fn mem_load_w_n(&mut self, n: u32) {
        self.add_operand(Operand::MemLoadWImm(n));
    }

    /// Pushes `MemStore` command onto the stack.
    fn mem_store(&mut self) {
        self.add_operand(Operand::MemStore);
    }

    /// Pushes `MemStoreImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in memory.
    fn mem_store_n(&mut self, n: u32) {
        self.add_operand(Operand::MemStoreImm(n));
    }

    /// Pushes `MemStoreW` command onto the stack.
    fn mem_store_w(&mut self) {
        self.add_operand(Operand::MemStoreW);
    }

    /// Pushes `MemStoreWImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in memory.
    fn mem_store_w_n(&mut self, n: u32) {
        self.add_operand(Operand::MemStoreWImm(n));
    }

    /// Pushes `LocLoad` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from local storage.
    fn loc_load(&mut self, n: u16) {
        self.add_operand(Operand::LocLoad(n));
    }

    /// Pushes `LocLoadW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from local storage.
    fn loc_load_w(&mut self, n: u16) {
        self.add_operand(Operand::LocLoadW(n));
    }

    /// Pushes `LocStore` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in local storage.
    fn loc_store(&mut self, n: u16) {
        self.add_operand(Operand::LocStore(n));
    }

    /// Pushes `LocStoreW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in local storage.
    fn loc_store_w(&mut self, n: u16) {
        self.add_operand(Operand::LocStoreW(n));
    }

    /// Pushes `Push` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to push onto the stack.
    fn push(&mut self, n: u64) {
        self.add_operand(Operand::Push(BaseElement::from(n)));
    }

    /// Pushes `AdvPush` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to push onto the stack.
    fn adv_push(&mut self, n: usize) {
        self.add_operand(Operand::AdvPush(n));
    }

    fn exec(&mut self, name: &str) {
        self.add_operand(Operand::Exec(name.to_string()));
    }

    /// Pushes `Increment` command onto the stack.
    fn increment(&mut self) {
        self.add_operand(Operand::Increment);
    }

    /// Pushes `Decrement` command onto the stack.
    fn decrement(&mut self) {
        self.add_operand(Operand::Decrement);
    }

    fn u32checked_add(&mut self) {
        self.add_operand(Operand::U32CheckedAdd);
    }

    fn u32checked_add_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedAddImm(n));
    }

    fn u32overflowing_add(&mut self) {
        self.add_operand(Operand::U32OverflowingAdd);
    }

    fn u32overflowing_add_n(&mut self, n: u32) {
        self.add_operand(Operand::U32OverflowingAddImm(n));
    }

    fn u32wrapping_add(&mut self) {
        self.add_operand(Operand::U32WrappingAdd);
    }

    fn u32wrapping_add_n(&mut self, n: u32) {
        self.add_operand(Operand::U32WrappingAddImm(n));
    }

    fn u32checked_sub(&mut self) {
        self.add_operand(Operand::U32CheckedSub);
    }

    fn u32checked_sub_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedSubImm(n));
    }

    fn u32overflowing_sub(&mut self) {
        self.add_operand(Operand::U32OverflowingSub);
    }

    fn u32overflowing_sub_n(&mut self, n: u32) {
        self.add_operand(Operand::U32OverflowingSubImm(n));
    }

    fn u32wrapping_sub(&mut self) {
        self.add_operand(Operand::U32WrappingSub);
    }

    fn u32wrapping_sub_n(&mut self, n: u32) {
        self.add_operand(Operand::U32WrappingSubImm(n));
    }

    fn u32checked_mul(&mut self) {
        self.add_operand(Operand::U32CheckedMul);
    }

    fn u32checked_mul_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedMulImm(n));
    }

    fn u32overflowing_mul(&mut self) {
        self.add_operand(Operand::U32OverflowingMul);
    }

    fn u32overflowing_mul_n(&mut self, n: u32) {
        self.add_operand(Operand::U32OverflowingMulImm(n));
    }

    fn u32wrapping_mul(&mut self) {
        self.add_operand(Operand::U32WrappingMul);
    }

    fn u32wrapping_mul_n(&mut self, n: u32) {
        self.add_operand(Operand::U32WrappingMulImm(n));
    }

    fn u32checked_div(&mut self) {
        self.add_operand(Operand::U32CheckedDiv);
    }

    fn u32checked_div_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedDivImm(n));
    }

    fn u32unchecked_div(&mut self) {
        self.add_operand(Operand::U32UncheckedDiv);
    }

    fn u32unchecked_div_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedDivImm(n));
    }

    fn u32overflowing_madd(&mut self) {
        self.add_operand(Operand::U32OverflowingMadd);
    }

    fn u32wrapping_madd(&mut self) {
        self.add_operand(Operand::U32WrappingMadd);
    }

    fn u32checked_mod(&mut self) {
        self.add_operand(Operand::U32CheckedMod);
    }

    fn u32checked_mod_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedModImm(n));
    }

    fn u32unchecked_mod(&mut self) {
        self.add_operand(Operand::U32UncheckedMod);
    }

    fn u32unchecked_mod_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedModImm(n));
    }

    fn u32checked_divmod(&mut self) {
        self.add_operand(Operand::U32CheckedDivMod);
    }

    fn u32checked_divmod_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedDivModImm(n));
    }

    fn u32unchecked_divmod(&mut self) {
        self.add_operand(Operand::U32UncheckedDivMod);
    }

    fn u32unchecked_divmod_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedDivModImm(n));
    }

    // bitwise

    fn u32checked_and(&mut self) {
        self.add_operand(Operand::U32CheckedAnd);
    }

    fn u32checked_or(&mut self) {
        self.add_operand(Operand::U32CheckedOr);
    }

    fn u32checked_xor(&mut self) {
        self.add_operand(Operand::U32CheckedXor);
    }

    fn u32checked_not(&mut self) {
        self.add_operand(Operand::U32CheckedNot);
    }

    fn u32checked_shl(&mut self) {
        self.add_operand(Operand::U32CheckedShl);
    }

    fn u32checked_shl_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedShlImm(n));
    }

    fn u32unchecked_shl(&mut self) {
        self.add_operand(Operand::U32UncheckedShl);
    }

    fn u32unchecked_shl_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedShlImm(n));
    }

    fn u32checked_shr(&mut self) {
        self.add_operand(Operand::U32CheckedShr);
    }

    fn u32checked_shr_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedShrImm(n));
    }

    fn u32unchecked_shr(&mut self) {
        self.add_operand(Operand::U32UncheckedShr);
    }

    fn u32unchecked_shr_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedShrImm(n));
    }

    fn u32checked_rotl(&mut self) {
        self.add_operand(Operand::U32CheckedRotl);
    }

    fn u32checked_rotl_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedRotlImm(n));
    }

    fn u32unchecked_rotl(&mut self) {
        self.add_operand(Operand::U32UncheckedRotl);
    }

    fn u32unchecked_rotl_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedRotlImm(n));
    }

    fn u32checked_rotr(&mut self) {
        self.add_operand(Operand::U32CheckedRotr);
    }

    fn u32checked_rotr_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedRotrImm(n));
    }

    fn u32unchecked_rotr(&mut self) {
        self.add_operand(Operand::U32UncheckedRotr);
    }

    fn u32unchecked_rotr_n(&mut self, n: u32) {
        self.add_operand(Operand::U32UncheckedRotrImm(n));
    }

    fn u32checked_popcnt(&mut self) {
        self.add_operand(Operand::U32CheckedPopcnt);
    }

    fn u32unchecked_popcnt(&mut self) {
        self.add_operand(Operand::U32UncheckedPopcnt);
    }

    // comparison

    fn u32checked_eq(&mut self) {
        self.add_operand(Operand::U32CheckedEq);
    }

    fn u32checked_eq_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedEqImm(n));
    }

    fn u32checked_neq(&mut self) {
        self.add_operand(Operand::U32CheckedNeq);
    }

    fn u32checked_neq_n(&mut self, n: u32) {
        self.add_operand(Operand::U32CheckedNeqImm(n));
    }

    fn u32checked_lt(&mut self) {
        self.add_operand(Operand::U32CheckedLt);
    }

    fn u32unchecked_lt(&mut self) {
        self.add_operand(Operand::U32UncheckedLt);
    }

    fn u32checked_lte(&mut self) {
        self.add_operand(Operand::U32CheckedLte);
    }

    fn u32unchecked_lte(&mut self) {
        self.add_operand(Operand::U32UncheckedLte);
    }

    fn u32checked_gt(&mut self) {
        self.add_operand(Operand::U32CheckedGt);
    }

    fn u32unchecked_gt(&mut self) {
        self.add_operand(Operand::U32UncheckedGt);
    }

    fn u32checked_gte(&mut self) {
        self.add_operand(Operand::U32CheckedGte);
    }

    fn u32unchecked_gte(&mut self) {
        self.add_operand(Operand::U32UncheckedGte);
    }

    fn u32checked_min(&mut self) {
        self.add_operand(Operand::U32CheckedMin);
    }

    fn u32unchecked_min(&mut self) {
        self.add_operand(Operand::U32UncheckedMin);
    }

    fn u32checked_max(&mut self) {
        self.add_operand(Operand::U32CheckedMax);
    }

    fn u32unchecked_max(&mut self) {
        self.add_operand(Operand::U32UncheckedMax);
    }
}
