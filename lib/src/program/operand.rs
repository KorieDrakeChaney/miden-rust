use math::fields::f64::BaseElement;

use super::error::MidenProgramError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    // assertions and tests
    Assert,
    AssertZ,
    AssertEq,
    AssertEqW,

    Push(BaseElement),
    Drop,
    DropW,
    Dup(usize),   // 1-15
    DupW(usize),  // 1-3
    Swap(usize),  // 1-15
    SwapW(usize), // 1-3
    SwapDw(usize),
    PadW,

    MovUp(usize),  // 2-15
    MovUpW(usize), // 2-3

    MovDn(usize),  // 2-15
    MovDnW(usize), // 2-3

    // Arithmetic and Boolean operations
    Add,
    AddImm(BaseElement),
    Sub,
    SubImm(BaseElement),
    Mul,
    MulImm(BaseElement),
    Div,
    DivImm(BaseElement),
    Neg,
    Inv,
    Incr,
    Pow2,
    Exp,
    ExpImm(u64),
    ExpBitLength(u8),
    Not,
    And,
    Or,
    Xor,

    // Comparison operations
    Eq,
    EqImm(BaseElement),
    Neq,
    NeqImm(BaseElement),
    EqW,
    Lt,
    Lte,
    Gt,
    Gte,
    IsOdd,

    // Extension Field Operations
    Ext2Add,
    Ext2Sub,
    Ext2Mul,
    Ext2Neg,
    Ext2Inv,
    Ext2Div,

    AdvPush(usize),
    AdvLoadW,
    AdvPipe,

    WHILE,
    IF,
    END,
    ELSE,
    REPEAT(usize),
    BEGIN,

    PRINT(String),

    MemLoad,
    MemLoadImm(u32),
    MemLoadW,
    MemLoadWImm(u32),
    LocLoad(u16),
    LocLoadW(u16),

    MemStore,
    MemStoreImm(u32),
    LocStore(u16),
    MemStoreW,
    MemStoreWImm(u32),
    LocStoreW(u16),

    Increment,
    Decrement,

    Exec(String),

    Error(MidenProgramError),

    // u32 arithmetic operations
    U32CheckedAdd,
    U32CheckedAddImm(u32),

    U32OverflowingAdd,
    U32OverflowingAddImm(u32),

    U32WrappingAdd,
    U32WrappingAddImm(u32),

    U32OverflowingAdd3,
    U32WrappingAdd3,

    U32CheckedSub,
    U32CheckedSubImm(u32),

    U32OverflowingSub,
    U32OverflowingSubImm(u32),

    U32WrappingSub,
    U32WrappingSubImm(u32),

    U32CheckedMul,
    U32CheckedMulImm(u32),

    U32OverflowingMul,
    U32OverflowingMulImm(u32),

    U32WrappingMul,
    U32WrappingMulImm(u32),

    U32OverflowingMadd,
    U32WrappingMadd,

    U32CheckedDiv,
    U32CheckedDivImm(u32),

    U32UncheckedDiv,
    U32UncheckedDivImm(u32),

    U32CheckedMod,
    U32CheckedModImm(u32),

    U32UncheckedMod,
    U32UncheckedModImm(u32),

    U32CheckedDivMod,
    U32CheckedDivModImm(u32),

    U32UncheckedDivMod,
    U32UncheckedDivModImm(u32),

    // u32 bitwise operations
    U32CheckedAnd,

    U32CheckedOr,

    U32CheckedXor,

    U32CheckedNot,

    U32CheckedShl,
    U32CheckedShlImm(u32),

    U32UncheckedShl,
    U32UncheckedShlImm(u32),

    U32CheckedShr,
    U32CheckedShrImm(u32),

    U32UncheckedShr,
    U32UncheckedShrImm(u32),

    U32CheckedRotl,
    U32CheckedRotlImm(u32),

    U32UncheckedRotl,
    U32UncheckedRotlImm(u32),

    U32CheckedRotr,
    U32CheckedRotrImm(u32),

    U32UncheckedRotr,
    U32UncheckedRotrImm(u32),

    U32CheckedPopcnt,
    U32UncheckedPopcnt,

    // u32 comparison operations
    U32CheckedEq,
    U32CheckedEqImm(u32),

    U32CheckedNeq,
    U32CheckedNeqImm(u32),

    U32CheckedLt,
    U32UncheckedLt,

    U32CheckedLte,
    U32UncheckedLte,

    U32CheckedGt,
    U32UncheckedGt,

    U32CheckedGte,
    U32UncheckedGte,

    U32CheckedMin,
    U32UncheckedMin,

    U32CheckedMax,
    U32UncheckedMax,

    CommentedOut(String),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // assertions and tests
            Self::Assert => write!(f, "assert"),
            Self::AssertZ => write!(f, "assertz"),
            Self::AssertEq => write!(f, "assert_eq"),
            Self::AssertEqW => write!(f, "assert_eqw"),
            // Manipulation operations
            Self::Push(value) => write!(f, "push.{value}"),
            Self::Drop => write!(f, "drop"),
            Self::DropW => write!(f, "dropw"),
            Self::Dup(value) => write!(f, "dup.{value}"),
            Self::DupW(value) => write!(f, "dupw.{value}"),
            Self::Swap(value) => write!(f, "swap.{value}"),
            Self::SwapW(value) => write!(f, "swapw.{value}"),
            Self::SwapDw(value) => write!(f, "swapdw.{value}"),
            Self::PadW => write!(f, "padw"),
            Self::MovUp(value) => write!(f, "movup.{value}"),
            Self::MovUpW(value) => write!(f, "movupw.{value}"),
            Self::MovDn(value) => write!(f, "movdn.{value}"),
            Self::MovDnW(value) => write!(f, "movdnw.{value}"),

            // Arithmetic and Boolean operations
            Self::Add => write!(f, "add"),
            Self::AddImm(value) => write!(f, "add.{value}"),
            Self::Sub => write!(f, "sub"),
            Self::SubImm(value) => write!(f, "sub.{value}"),
            Self::Mul => write!(f, "mul"),
            Self::MulImm(value) => write!(f, "mul.{value}"),
            Self::Div => write!(f, "div"),
            Self::DivImm(value) => write!(f, "div.{value}"),
            Self::Neg => write!(f, "neg"),
            Self::Inv => write!(f, "inv"),
            Self::Incr => write!(f, "add.1"),
            Self::Pow2 => write!(f, "pow2"),
            Self::Exp => write!(f, "exp"),
            Self::ExpImm(value) => write!(f, "exp.{value}"),
            Self::ExpBitLength(value) => write!(f, "exp.u{value}"),

            // Comparison operations
            Self::Not => write!(f, "not"),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
            Self::Eq => write!(f, "eq"),
            Self::EqImm(value) => write!(f, "eq.{value}"),
            Self::Neq => write!(f, "neq"),
            Self::NeqImm(value) => write!(f, "neq.{value}"),
            Self::EqW => write!(f, "eqw"),
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),
            Self::IsOdd => write!(f, "is_odd"),

            // Extension Field Operations
            Self::Ext2Add => write!(f, "ext2add"),
            Self::Ext2Sub => write!(f, "ext2sub"),
            Self::Ext2Mul => write!(f, "ext2mul"),
            Self::Ext2Neg => write!(f, "ext2neg"),
            Self::Ext2Inv => write!(f, "ext2inv"),
            Self::Ext2Div => write!(f, "ext2div"),

            Self::MemLoad => write!(f, "mem_load"),
            Self::MemLoadImm(value) => write!(f, "mem_load.{value}"),
            Self::MemLoadW => write!(f, "mem_loadw"),
            Self::MemLoadWImm(value) => write!(f, "mem_loadw.{value}"),
            Self::LocLoad(value) => write!(f, "loc_load.{value}"),
            Self::LocLoadW(value) => write!(f, "loc_loadw.{value}"),

            Self::MemStore => write!(f, "mem_store"),
            Self::MemStoreImm(value) => write!(f, "mem_store.{value}"),
            Self::LocStore(value) => write!(f, "loc_store.{value}"),
            Self::MemStoreW => write!(f, "mem_storew"),
            Self::MemStoreWImm(value) => write!(f, "mem_storew.{value}"),
            Self::LocStoreW(value) => write!(f, "loc_storew.{value}"),

            Self::AdvPush(value) => write!(f, "adv_push.{value}"),
            Self::AdvLoadW => write!(f, "adv_loadw"),
            Self::AdvPipe => write!(f, "adv_pipe"),

            Self::WHILE => write!(f, "while.true"),
            Self::IF => write!(f, "if.true"),
            Self::END => write!(f, "end"),
            Self::ELSE => write!(f, "else"),
            Self::REPEAT(value) => write!(f, "repeat.{value}"),
            Self::BEGIN => write!(f, "begin"),
            Self::Exec(value) => write!(f, "exec.{value}"),
            Self::PRINT(_) => write!(f, ""),

            Self::Increment => write!(f, "add.1"),
            Self::Decrement => write!(f, "sub.1"),

            Self::Error(e) => write!(f, "{}", e),
            Self::CommentedOut(message) => write!(f, "#{}", message),

            // u32 arithmetic operations
            Self::U32CheckedAdd => write!(f, "u32checked_add"),
            Self::U32CheckedAddImm(value) => write!(f, "u32checked_add.{value}"),

            Self::U32OverflowingAdd => write!(f, "u32overflowing_add"),
            Self::U32OverflowingAddImm(value) => write!(f, "u32overflowing_add.{value}"),

            Self::U32WrappingAdd => write!(f, "u32wrapping_add"),
            Self::U32WrappingAddImm(value) => write!(f, "u32wrapping_add.{value}"),

            Self::U32OverflowingAdd3 => write!(f, "u32overflowing_add3"),
            Self::U32WrappingAdd3 => write!(f, "u32wrapping_add3"),

            Self::U32CheckedSub => write!(f, "u32checked_sub"),
            Self::U32CheckedSubImm(value) => write!(f, "u32checked_sub.{value}"),

            Self::U32OverflowingSub => write!(f, "u32overflowing_sub"),
            Self::U32OverflowingSubImm(value) => write!(f, "u32overflowing_sub.{value}"),

            Self::U32WrappingSub => write!(f, "u32wrapping_sub"),
            Self::U32WrappingSubImm(value) => write!(f, "u32wrapping_sub.{value}"),

            Self::U32CheckedMul => write!(f, "u32checked_mul"),
            Self::U32CheckedMulImm(value) => write!(f, "u32checked_mul.{value}"),

            Self::U32OverflowingMul => write!(f, "u32overflowing_mul"),
            Self::U32OverflowingMulImm(value) => write!(f, "u32overflowing_mul.{value}"),

            Self::U32WrappingMul => write!(f, "u32wrapping_mul"),
            Self::U32WrappingMulImm(value) => write!(f, "u32wrapping_mul.{value}"),

            Self::U32OverflowingMadd => write!(f, "u32overflowing_madd"),
            Self::U32WrappingMadd => write!(f, "u32wrapping_madd"),

            Self::U32CheckedDiv => write!(f, "u32checked_div"),
            Self::U32CheckedDivImm(value) => write!(f, "u32checked_div.{value}"),

            Self::U32UncheckedDiv => write!(f, "u32unchecked_div"),
            Self::U32UncheckedDivImm(value) => write!(f, "u32unchecked_div.{value}"),

            Self::U32CheckedMod => write!(f, "u32checked_mod"),
            Self::U32CheckedModImm(value) => write!(f, "u32checked_mod.{value}"),

            Self::U32UncheckedMod => write!(f, "u32unchecked_mod"),
            Self::U32UncheckedModImm(value) => write!(f, "u32unchecked_mod.{value}"),

            Self::U32CheckedDivMod => write!(f, "u32checked_divmod"),
            Self::U32CheckedDivModImm(value) => write!(f, "u32checked_divmod.{value}"),

            Self::U32UncheckedDivMod => write!(f, "u32unchecked_divmod"),
            Self::U32UncheckedDivModImm(value) => write!(f, "u32unchecked_divmod.{value}"),

            // u32 bitwise operations
            Self::U32CheckedAnd => write!(f, "u32checked_and"),

            Self::U32CheckedOr => write!(f, "u32checked_or"),

            Self::U32CheckedXor => write!(f, "u32checked_xor"),

            Self::U32CheckedNot => write!(f, "u32checked_not"),

            Self::U32CheckedShl => write!(f, "u32checked_shl"),
            Self::U32CheckedShlImm(value) => write!(f, "u32checked_shl.{value}"),

            Self::U32UncheckedShl => write!(f, "u32unchecked_shl"),
            Self::U32UncheckedShlImm(value) => write!(f, "u32unchecked_shl.{value}"),

            Self::U32CheckedShr => write!(f, "u32checked_shr"),
            Self::U32CheckedShrImm(value) => write!(f, "u32checked_shr.{value}"),

            Self::U32UncheckedShr => write!(f, "u32unchecked_shr"),
            Self::U32UncheckedShrImm(value) => write!(f, "u32unchecked_shr.{value}"),

            Self::U32CheckedRotl => write!(f, "u32checked_rotl"),
            Self::U32CheckedRotlImm(value) => write!(f, "u32checked_rotl.{value}"),

            Self::U32UncheckedRotl => write!(f, "u32unchecked_rotl"),
            Self::U32UncheckedRotlImm(value) => write!(f, "u32unchecked_rotl.{value}"),

            Self::U32CheckedRotr => write!(f, "u32checked_rotr"),
            Self::U32CheckedRotrImm(value) => write!(f, "u32checked_rotr.{value}"),

            Self::U32UncheckedRotr => write!(f, "u32unchecked_rotr"),
            Self::U32UncheckedRotrImm(value) => write!(f, "u32unchecked_rotr.{value}"),

            Self::U32CheckedPopcnt => write!(f, "u32checked_popcnt"),
            Self::U32UncheckedPopcnt => write!(f, "u32unchecked_popcnt"),

            // u32 comparison operations
            Self::U32CheckedEq => write!(f, "u32checked_eq"),
            Self::U32CheckedEqImm(value) => write!(f, "u32checked_eq.{value}"),

            Self::U32CheckedNeq => write!(f, "u32checked_neq"),
            Self::U32CheckedNeqImm(value) => write!(f, "u32checked_neq.{value}"),

            Self::U32CheckedLt => write!(f, "u32checked_lt"),
            Self::U32UncheckedLt => write!(f, "u32unchecked_lt"),

            Self::U32CheckedLte => write!(f, "u32checked_lte"),
            Self::U32UncheckedLte => write!(f, "u32unchecked_lte"),

            Self::U32CheckedGt => write!(f, "u32checked_gt"),
            Self::U32UncheckedGt => write!(f, "u32unchecked_gt"),

            Self::U32CheckedGte => write!(f, "u32checked_gte"),
            Self::U32UncheckedGte => write!(f, "u32unchecked_gte"),

            Self::U32CheckedMin => write!(f, "u32checked_min"),
            Self::U32UncheckedMin => write!(f, "u32unchecked_min"),

            Self::U32CheckedMax => write!(f, "u32checked_max"),
            Self::U32UncheckedMax => write!(f, "u32unchecked_max"),
        }
    }
}
