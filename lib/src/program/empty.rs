use std::collections::VecDeque;

use math::fields::f64::BaseElement;

use super::operand::Operand;

pub struct EmptyProgram {
    stack_operands: VecDeque<Operand>,
}

impl EmptyProgram {
    pub fn new() -> Self {
        Self {
            stack_operands: VecDeque::new(),
        }
    }

    pub fn add_operand(&mut self, operand: Operand) {
        self.stack_operands.push_back(operand);
    }

    pub fn add_operands(&mut self, operands: VecDeque<Operand>) {
        for op in operands {
            self.stack_operands.push_back(op);
        }
    }
    /// Constructs a new `if-else` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `if_op` - A closure that returns the operands for the `if` block.
    /// * `else_op` - A closure that returns the operands for the `else` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main() {
    ///     let mut program = MidenProgram::new();
    ///     program.push(1);
    ///     program.if_else(|| {
    ///         let mut block = EmptyProgram::new();
    ///         block.push(1);
    ///         block.get_operands()},
    ///     || {
    ///         let mut block = EmptyProgram::new();
    ///         block.push(5);
    ///         block.get_operands()},
    ///     );
    /// }
    ///    
    /// ```
    pub fn if_else<F1, F2>(&mut self, if_op: F1, else_op: F2)
    where
        F1: FnOnce() -> VecDeque<Operand>,
        F2: FnOnce() -> VecDeque<Operand>,
    {
        let mut temp_stack = VecDeque::new();
        let mut if_operands = if_op();
        let mut else_operands = else_op();

        temp_stack.push_back(Operand::IF);
        temp_stack.append(&mut if_operands);
        temp_stack.push_back(Operand::ELSE);
        temp_stack.append(&mut else_operands);
        temp_stack.push_back(Operand::END);

        self.add_operands(temp_stack);
    }
    /// Constructs a new `while` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `block` - A closure that returns the operands for the `while` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main() {
    ///     let mut program = MidenProgram::new();
    ///     program.while_block(|| {
    ///         let mut block = EmptyProgram::new();
    ///         block.push(1);
    ///         block.increment();
    ///         block.dup();
    ///         block.neq_n(10);
    ///
    ///         block.get_operands()
    ///     });
    /// }
    /// ```
    pub fn while_block<F>(&mut self, block: F)
    where
        F: FnOnce() -> VecDeque<Operand>,
    {
        let mut block_operands = block();
        block_operands.push_front(Operand::WHILE);
        block_operands.push_back(Operand::END);
        self.add_operands(block_operands);
    }
    /// Constructs a new `repeat` block in the Miden program.
    ///
    /// # Arguments
    /// * `n` - The number of times to repeat the block.
    /// * `program` - A closure that returns the operands for the `repeat` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main(){
    ///     let mut program = MidenProgram::new();
    ///     program.repeat(5, || {
    ///         let mut block = EmptyProgram::new();
    ///         block.push(1);
    ///         block.push(2);
    ///         block.add();
    ///
    ///         block.get_operands()
    ///     });
    /// }
    ///
    pub fn repeat<F>(&mut self, n: usize, program: F)
    where
        F: FnOnce() -> VecDeque<Operand>,
    {
        let mut operands = program();
        operands.push_front(Operand::REPEAT(n));
        operands.push_back(Operand::END);
        self.add_operands(operands);
    }

    /// Pushes a print instruction to the stack with a message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to print.
    pub fn print(&mut self, message: &str) {
        self.stack_operands
            .push_back(Operand::PRINT(message.to_string()));
    }

    /// Pushes `Drop` command onto the stack.
    pub fn drop(&mut self) {
        self.add_operand(Operand::Drop);
    }

    /// Pushes `Swap` command onto the stack.
    pub fn swap(&mut self) {
        self.add_operand(Operand::Swap(1));
    }

    /// Pushes `Swap` command with value `n` onto the stack.
    ///     
    /// # Arguments
    ///
    /// * `n` - The value to swap
    pub fn swap_n(&mut self, n: usize) {
        self.add_operand(Operand::Swap(n));
    }

    /// Pushes `Dup` command onto the stack.
    pub fn dup(&mut self) {
        self.add_operand(Operand::Dup(1));
    }

    /// Pushes `Dup` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to duplicate.
    pub fn dup_n(&mut self, n: usize) {
        self.add_operand(Operand::Dup(n));
    }

    /// Pushes `SwapW` command onto the stack.
    pub fn swapw(&mut self) {
        self.add_operand(Operand::SwapW(1));
    }

    /// Pushes `SwapW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to swap.
    pub fn swapw_n(&mut self, n: usize) {
        self.add_operand(Operand::SwapW(n));
    }

    /// Pushes `PadW` command onto the stack.
    pub fn padw(&mut self) {
        self.add_operand(Operand::PadW);
    }

    /// Pushes `MovUp` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move up.
    pub fn movup_n(&mut self, n: usize) {
        self.add_operand(Operand::MovUp(n));
    }

    /// Pushes `MovUpW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move up.
    pub fn movupw_n(&mut self, n: usize) {
        self.add_operand(Operand::MovUpW(n));
    }

    /// Pushes `MovDn` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move down.
    pub fn movdn_n(&mut self, n: usize) {
        self.add_operand(Operand::MovDn(n));
    }

    /// Pushes `MovDnW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move down.
    pub fn movdnw_n(&mut self, n: usize) {
        self.add_operand(Operand::MovDnW(n));
    }

    /// Pushes `Add` command onto the stack.
    pub fn add(&mut self) {
        self.add_operand(Operand::Add);
    }

    /// Pushes `AddImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to add.
    pub fn add_n(&mut self, n: u64) {
        self.add_operand(Operand::AddImm(BaseElement::from(n)));
    }

    /// Pushes `Sub` command onto the stack.
    pub fn sub(&mut self) {
        self.add_operand(Operand::Sub);
    }

    /// Pushes `SubImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to subtract.
    pub fn sub_n(&mut self, n: u64) {
        self.add_operand(Operand::SubImm(BaseElement::from(n)));
    }

    /// Pushes `Mul` command onto the stack.
    pub fn mul(&mut self) {
        self.add_operand(Operand::Mul);
    }

    /// Pushes `MulImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to multiply.
    pub fn mul_n(&mut self, n: u64) {
        self.add_operand(Operand::MulImm(BaseElement::from(n)));
    }

    /// Pushes `Div` command onto the stack.
    pub fn div(&mut self) {
        self.add_operand(Operand::Div);
    }

    /// Pushes `DivImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to divide.
    pub fn div_n(&mut self, n: u64) {
        self.add_operand(Operand::DivImm(BaseElement::from(n)));
    }

    /// Pushes `Neg` command onto the stack.
    pub fn neg(&mut self) {
        self.add_operand(Operand::Neg);
    }

    /// Pushes `Inv` command onto the stack.
    pub fn inv(&mut self) {
        self.add_operand(Operand::Inv);
    }

    /// Pushes `Pow2` command onto the stack.
    pub fn pow2(&mut self) {
        self.add_operand(Operand::Pow2);
    }

    /// Pushes `Exp` command onto the stack.
    pub fn exp(&mut self) {
        self.add_operand(Operand::Exp);
    }

    /// Pushes `ExpImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to exponentiate.
    pub fn exp_n(&mut self, n: u64) {
        self.add_operand(Operand::ExpImm(n));
    }

    /// Pushes `And` command onto the stack.
    pub fn and(&mut self) {
        self.add_operand(Operand::And);
    }

    /// Pushes `Or` command onto the stack.
    pub fn or(&mut self) {
        self.add_operand(Operand::Or);
    }

    /// Pushes `Xor` command onto the stack.
    pub fn xor(&mut self) {
        self.add_operand(Operand::Xor);
    }

    pub fn not(&mut self) {
        self.add_operand(Operand::Not);
    }

    /// Pushes `Eq` command onto the stack.
    pub fn eq(&mut self) {
        self.add_operand(Operand::Eq);
    }

    /// Pushes `EqImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to compare for equality.
    pub fn eq_n(&mut self, n: u64) {
        self.add_operand(Operand::EqImm(BaseElement::from(n)));
    }

    /// Pushes `Neq` command onto the stack.
    pub fn neq(&mut self) {
        self.add_operand(Operand::Neq);
    }

    /// Pushes `NeqImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to compare for inequality.
    pub fn neq_n(&mut self, n: u64) {
        self.add_operand(Operand::NeqImm(BaseElement::from(n)));
    }

    /// Pushes `Lt` command onto the stack.
    pub fn lt(&mut self) {
        self.add_operand(Operand::Lt);
    }

    /// Pushes `Lte` command onto the stack.
    pub fn lte(&mut self) {
        self.add_operand(Operand::Lte);
    }

    /// Pushes `Gt` command onto the stack.
    pub fn gt(&mut self) {
        self.add_operand(Operand::Gt);
    }

    /// Pushes `Gte` command onto the stack.
    pub fn gte(&mut self) {
        self.add_operand(Operand::Gte);
    }

    /// Pushes `IsOdd` command onto the stack.
    pub fn is_odd(&mut self) {
        self.add_operand(Operand::IsOdd);
    }

    /// Pushes `Eqw` command onto the stack.
    pub fn eqw(&mut self) {
        self.add_operand(Operand::EqW);
    }

    /// Pushes `MemLoad` command onto the stack.
    pub fn mem_load(&mut self) {
        self.add_operand(Operand::MemLoad);
    }

    /// Pushes `MemLoadImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from memory.
    pub fn mem_load_n(&mut self, n: u32) {
        self.add_operand(Operand::MemLoadImm(n));
    }

    /// Pushes `MemLoadW` command onto the stack.
    pub fn mem_load_w(&mut self) {
        self.add_operand(Operand::MemLoadW);
    }

    /// Pushes `MemLoadWImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from memory.
    pub fn mem_load_w_n(&mut self, n: u32) {
        self.add_operand(Operand::MemLoadWImm(n));
    }

    /// Pushes `MemStore` command onto the stack.
    pub fn mem_store(&mut self) {
        self.add_operand(Operand::MemStore);
    }

    /// Pushes `MemStoreImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in memory.
    pub fn mem_store_n(&mut self, n: u32) {
        self.add_operand(Operand::MemStoreImm(n));
    }

    /// Pushes `MemStoreW` command onto the stack.
    pub fn mem_store_w(&mut self) {
        self.add_operand(Operand::MemStoreW);
    }

    /// Pushes `MemStoreWImm` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in memory.
    pub fn mem_store_w_n(&mut self, n: u32) {
        self.add_operand(Operand::MemStoreWImm(n));
    }

    /// Pushes `LocLoad` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from local storage.
    pub fn loc_load(&mut self, n: u16) {
        self.add_operand(Operand::LocLoad(n));
    }

    /// Pushes `LocLoadW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from local storage.
    pub fn loc_load_w(&mut self, n: u16) {
        self.add_operand(Operand::LocLoadW(n));
    }

    /// Pushes `LocStore` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in local storage.
    pub fn loc_store(&mut self, n: u16) {
        self.add_operand(Operand::LocStore(n));
    }

    /// Pushes `LocStoreW` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in local storage.
    pub fn loc_store_w(&mut self, n: u16) {
        self.add_operand(Operand::LocStoreW(n));
    }

    /// Pushes `Push` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to push onto the stack.
    pub fn push(&mut self, n: u64) {
        self.add_operand(Operand::Push(BaseElement::from(n)));
    }

    /// Pushes `AdvPush` command with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to push onto the stack.
    pub fn adv_push(&mut self, n: usize) {
        self.add_operand(Operand::AdvPush(n));
    }

    /// Returns a clone of the stack operands.
    ///
    /// # Returns
    ///
    /// A `VecDeque` containing the operands on the stack.
    pub fn get_operands(&self) -> VecDeque<Operand> {
        self.stack_operands.clone()
    }

    /// Adds a program to the stack operands.
    ///
    /// # Arguments
    ///
    /// * `program` - A function that returns a `VecDeque` of operands.
    pub fn add_program<F>(&mut self, program: F)
    where
        F: FnOnce() -> VecDeque<Operand>,
    {
        self.add_operands(program());
    }

    pub fn exec(&mut self, name: &str) {
        self.add_operand(Operand::Exec(name.to_string()));
    }

    /// Pushes `Increment` command onto the stack.
    pub fn increment(&mut self) {
        self.add_operand(Operand::Increment);
    }

    /// Pushes `Decrement` command onto the stack.
    pub fn decrement(&mut self) {
        self.add_operand(Operand::Decrement);
    }

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
