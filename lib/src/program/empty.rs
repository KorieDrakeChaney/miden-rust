use std::collections::VecDeque;

use math::fields::f64::BaseElement;

use crate::Program;

use super::instruction::Instruction;

pub struct EmptyProgram {
    instructions: VecDeque<Instruction>,
}

impl EmptyProgram {
    pub fn new() -> Self {
        Self {
            instructions: VecDeque::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push_back(instruction);
    }

    pub fn add_operands(&mut self, instructions: VecDeque<Instruction>) {
        for instruction in instructions {
            self.instructions.push_back(instruction);
        }
    }
    /// Constructs a new `if-else` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `if_program` - A mutable reference to the program for the `if` block.
    /// * `else_program` - A mutable reference to the program for the `else` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main() {
    ///     let mut program = MidenProgram::new();
    ///     program.push(1);
    ///     let mut if_program = EmptyProgram::new();
    ///     if_program.push(1);
    ///     let mut else_program = EmptyProgram::new();
    ///     else_program.push(5);
    ///     program.if_else_block(&mut if_program, &mut else_program);
    /// }
    /// ```
    pub fn if_else_block<'a, T>(&'a mut self, if_program: &mut T, else_program: &mut T)
    where
        T: Program + 'a,
    {
        let mut temp_stack = VecDeque::new();
        let mut if_instructions = if_program.get_instructions();
        let mut else_instructions = else_program.get_instructions();

        temp_stack.push_back(Instruction::IF);
        temp_stack.append(&mut if_instructions);
        temp_stack.push_back(Instruction::ELSE);
        temp_stack.append(&mut else_instructions);
        temp_stack.push_back(Instruction::END);

        self.add_operands(temp_stack);
    }
    /// Constructs a new `while` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `program` - An instance of a type implementing the `Program` trait, which provides the instructions for the `while` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main() {
    ///     let mut program = MidenProgram::new();
    ///     program.push(1);
    ///     program.push(1);
    ///     let mut while_program = EmptyProgram::new();
    ///     while_program.increment();
    ///     while_program.dup();
    ///     while_program.neq_n(10);
    ///     program.while_block(&mut while_program);
    /// }
    /// ```
    pub fn while_block<'a, T>(&mut self, program: &mut T)
    where
        T: Program + 'a,
    {
        let mut instructions = program.get_instructions();
        instructions.push_front(Instruction::WHILE);
        instructions.push_back(Instruction::END);
        self.add_operands(instructions);
    }
    /// Constructs a new `repeat` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of times to repeat the block.
    /// * `program` - A mutable reference to the program for the `repeat` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main(){
    ///     let mut program = MidenProgram::new();
    ///     let mut repeat_program = EmptyProgram::new();
    ///     repeat_program.push(1);
    ///     repeat_program.push(2);
    ///     repeat_program.add();
    ///     program.repeat(5, &mut repeat_program);
    /// }
    /// ```
    pub fn repeat<'a, T>(&mut self, n: usize, program: &mut T)
    where
        T: Program + 'a,
    {
        let mut instructions = program.get_instructions();
        instructions.push_front(Instruction::REPEAT(n));
        instructions.push_back(Instruction::END);
        self.add_operands(instructions);
    }

    /// Constructs a new `if` block in the Miden program.
    ///
    /// # Arguments
    ///
    /// * `program` - A mutable reference to the program for the `if` block.
    ///
    /// # Example
    ///
    /// ```
    /// use rust_masm::{MidenProgram, EmptyProgram};
    ///
    /// fn main(){
    ///     let mut program = MidenProgram::new();
    ///     let mut if_program = EmptyProgram::new();
    ///     if_program.push(1);
    ///     if_program.push(2);
    ///     if_program.add();
    ///     program.if_block(&mut if_program);
    /// }
    /// ```
    pub fn if_block<'a, T>(&mut self, program: &mut T)
    where
        T: Program + 'a,
    {
        let mut instructions = program.get_instructions();
        instructions.push_front(Instruction::IF);
        instructions.push_back(Instruction::END);
        self.add_operands(instructions);
    }

    /// Pushes a print instruction to the stack with a message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to print.
    pub fn print(&mut self, message: &str) {
        self.add_instruction(Instruction::PRINT(message.to_string()));
    }

    /// Pushes `Drop` instruction onto the stack.
    pub fn drop(&mut self) {
        self.add_instruction(Instruction::Drop);
    }

    /// Pushes `Swap` instruction onto the stack.
    pub fn swap(&mut self) {
        self.add_instruction(Instruction::Swap(1));
    }

    /// Pushes `Swap` instruction with value `n` onto the stack.
    ///     
    /// # Arguments
    ///
    /// * `n` - The value to swap
    pub fn swap_n(&mut self, n: usize) {
        self.add_instruction(Instruction::Swap(n));
    }

    /// Pushes `Dup` instruction onto the stack.
    pub fn dup(&mut self) {
        self.add_instruction(Instruction::Dup(0));
    }

    /// Pushes `Dup` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to duplicate.
    pub fn dup_n(&mut self, n: usize) {
        self.add_instruction(Instruction::Dup(n));
    }

    /// Pushes `SwapW` instruction onto the stack.
    pub fn swapw(&mut self) {
        self.add_instruction(Instruction::SwapW(1));
    }

    /// Pushes `SwapW` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to swap.
    pub fn swapw_n(&mut self, n: usize) {
        self.add_instruction(Instruction::SwapW(n));
    }

    /// Pushes `PadW` instruction onto the stack.
    pub fn padw(&mut self) {
        self.add_instruction(Instruction::PadW);
    }

    /// Pushes `MovUp` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move up.
    pub fn movup_n(&mut self, n: usize) {
        self.add_instruction(Instruction::MovUp(n));
    }

    /// Pushes `MovUpW` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move up.
    pub fn movupw_n(&mut self, n: usize) {
        self.add_instruction(Instruction::MovUpW(n));
    }

    /// Pushes `MovDn` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move down.
    pub fn movdn_n(&mut self, n: usize) {
        self.add_instruction(Instruction::MovDn(n));
    }

    /// Pushes `MovDnW` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to move down.
    pub fn movdnw_n(&mut self, n: usize) {
        self.add_instruction(Instruction::MovDnW(n));
    }

    /// Pushes `Add` instruction onto the stack.
    pub fn add(&mut self) {
        self.add_instruction(Instruction::Add);
    }

    /// Pushes `AddImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to add.
    pub fn add_n(&mut self, n: u64) {
        self.add_instruction(Instruction::AddImm(BaseElement::from(n)));
    }

    /// Pushes `Sub` instruction onto the stack.
    pub fn sub(&mut self) {
        self.add_instruction(Instruction::Sub);
    }

    /// Pushes `SubImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to subtract.
    pub fn sub_n(&mut self, n: u64) {
        self.add_instruction(Instruction::SubImm(BaseElement::from(n)));
    }

    /// Pushes `Mul` instruction onto the stack.
    pub fn mul(&mut self) {
        self.add_instruction(Instruction::Mul);
    }

    /// Pushes `MulImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to multiply.
    pub fn mul_n(&mut self, n: u64) {
        self.add_instruction(Instruction::MulImm(BaseElement::from(n)));
    }

    /// Pushes `Div` instruction onto the stack.
    pub fn div(&mut self) {
        self.add_instruction(Instruction::Div);
    }

    /// Pushes `DivImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to divide.
    pub fn div_n(&mut self, n: u64) {
        self.add_instruction(Instruction::DivImm(BaseElement::from(n)));
    }

    /// Pushes `Neg` instruction onto the stack.
    pub fn neg(&mut self) {
        self.add_instruction(Instruction::Neg);
    }

    /// Pushes `Inv` instruction onto the stack.
    pub fn inv(&mut self) {
        self.add_instruction(Instruction::Inv);
    }

    /// Pushes `Pow2` instruction onto the stack.
    pub fn pow2(&mut self) {
        self.add_instruction(Instruction::Pow2);
    }

    /// Pushes `Exp` instruction onto the stack.
    pub fn exp(&mut self) {
        self.add_instruction(Instruction::Exp);
    }

    /// Pushes `ExpImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to exponentiate.
    pub fn exp_n(&mut self, n: u64) {
        self.add_instruction(Instruction::ExpImm(n));
    }

    /// Pushes `And` instruction onto the stack.
    pub fn and(&mut self) {
        self.add_instruction(Instruction::And);
    }

    /// Pushes `Or` instruction onto the stack.
    pub fn or(&mut self) {
        self.add_instruction(Instruction::Or);
    }

    /// Pushes `Xor` instruction onto the stack.
    pub fn xor(&mut self) {
        self.add_instruction(Instruction::Xor);
    }

    pub fn not(&mut self) {
        self.add_instruction(Instruction::Not);
    }

    /// Pushes `Eq` instruction onto the stack.
    pub fn eq(&mut self) {
        self.add_instruction(Instruction::Eq);
    }

    /// Pushes `EqImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to compare for equality.
    pub fn eq_n(&mut self, n: u64) {
        self.add_instruction(Instruction::EqImm(BaseElement::from(n)));
    }

    /// Pushes `Neq` instruction onto the stack.
    pub fn neq(&mut self) {
        self.add_instruction(Instruction::Neq);
    }

    /// Pushes `NeqImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to compare for inequality.
    pub fn neq_n(&mut self, n: u64) {
        self.add_instruction(Instruction::NeqImm(BaseElement::from(n)));
    }

    /// Pushes `Lt` instruction onto the stack.
    pub fn lt(&mut self) {
        self.add_instruction(Instruction::Lt);
    }

    /// Pushes `Lte` instruction onto the stack.
    pub fn lte(&mut self) {
        self.add_instruction(Instruction::Lte);
    }

    /// Pushes `Gt` instruction onto the stack.
    pub fn gt(&mut self) {
        self.add_instruction(Instruction::Gt);
    }

    /// Pushes `Gte` instruction onto the stack.
    pub fn gte(&mut self) {
        self.add_instruction(Instruction::Gte);
    }

    /// Pushes `IsOdd` instruction onto the stack.
    pub fn is_odd(&mut self) {
        self.add_instruction(Instruction::IsOdd);
    }

    /// Pushes `Eqw` instruction onto the stack.
    pub fn eqw(&mut self) {
        self.add_instruction(Instruction::EqW);
    }

    /// Pushes `MemLoad` instruction onto the stack.
    pub fn mem_load(&mut self) {
        self.add_instruction(Instruction::MemLoad);
    }

    /// Pushes `MemLoadImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from memory.
    pub fn mem_load_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemLoadImm(n));
    }

    /// Pushes `MemLoadW` instruction onto the stack.
    pub fn mem_load_w(&mut self) {
        self.add_instruction(Instruction::MemLoadW);
    }

    /// Pushes `MemLoadWImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from memory.
    pub fn mem_load_w_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemLoadWImm(n));
    }

    /// Pushes `MemStore` instruction onto the stack.
    pub fn mem_store(&mut self) {
        self.add_instruction(Instruction::MemStore);
    }

    /// Pushes `MemStoreImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in memory.
    pub fn mem_store_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemStoreImm(n));
    }

    /// Pushes `MemStoreW` instruction onto the stack.
    pub fn mem_store_w(&mut self) {
        self.add_instruction(Instruction::MemStoreW);
    }

    /// Pushes `MemStoreWImm` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in memory.
    pub fn mem_store_w_n(&mut self, n: u32) {
        self.add_instruction(Instruction::MemStoreWImm(n));
    }

    /// Pushes `LocLoad` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from local storage.
    pub fn loc_load(&mut self, n: u16) {
        self.add_instruction(Instruction::LocLoad(n));
    }

    /// Pushes `LocLoadW` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to load from local storage.
    pub fn loc_load_w(&mut self, n: u16) {
        self.add_instruction(Instruction::LocLoadW(n));
    }

    /// Pushes `LocStore` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in local storage.
    pub fn loc_store(&mut self, n: u16) {
        self.add_instruction(Instruction::LocStore(n));
    }

    /// Pushes `LocStoreW` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The address to store in local storage.
    pub fn loc_store_w(&mut self, n: u16) {
        self.add_instruction(Instruction::LocStoreW(n));
    }

    /// Pushes `Push` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to push onto the stack.
    pub fn push(&mut self, n: u64) {
        self.add_instruction(Instruction::Push(BaseElement::from(n)));
    }

    /// Pushes `AdvPush` instruction with value `n` onto the stack.
    ///
    /// # Arguments
    ///
    /// * `n` - The value to push onto the stack.
    pub fn adv_push(&mut self, n: usize) {
        self.add_instruction(Instruction::AdvPush(n));
    }

    pub fn exec(&mut self, name: &str) {
        self.add_instruction(Instruction::Exec(name.to_string()));
    }

    /// Pushes `Increment` instruction onto the stack.
    pub fn increment(&mut self) {
        self.add_instruction(Instruction::Increment);
    }

    /// Pushes `Decrement` instruction onto the stack.
    pub fn decrement(&mut self) {
        self.add_instruction(Instruction::Decrement);
    }

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

    pub fn add_program<F>(&mut self, program: F)
    where
        F: FnOnce() -> VecDeque<Instruction>,
    {
        self.add_operands(program());
    }
}

impl Program for EmptyProgram {
    fn get_instructions(&self) -> VecDeque<Instruction> {
        self.instructions.clone()
    }
}
