#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Arithmetic and boolean operations
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Inv,
    Pow2,
    Exp,
    And,
    Or,
    Xor,
    Not,
    IsOdd,
    // Comparison operations
    Eq,
    EqW,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    // Extension field operations
    Ext2Add,
    Ext2Sub,
    Ext2Mul,
    Ext2Neg,
    Ext2Inv,
    Ext2Div,

    // Control flow operations
    Proc,
    While,
    If,
    End,
    Else,
    Repeat,
    Begin,

    // Memory operations
    MemLoad,
    MemStore,
    MemLoadW,
    MemStoreW,
    LocLoad,
    LocStore,
    LocLoadW,
    LocStoreW,

    // Manipulation operations
    AdvPush,
    Push,
    Drop,
    Dup,
    Swap,
    SwapW,
    SwapDw,
    PadW,
    MovUp,
    MovUpW,
    MovDn,
    MovDnW,

    Exec,

    // u32 arithmetic operations
    U32CheckedAdd,

    U32OverflowingAdd,

    U32WrappingAdd,

    U32OverflowingAdd3,
    U32WrappingAdd3,

    U32CheckedSub,

    U32OverflowingSub,

    U32WrappingSub,

    U32CheckedMul,

    U32OverflowingMul,

    U32WrappingMul,

    U32OverflowingMadd,
    U32WrappingMadd,

    U32CheckedDiv,

    U32UncheckedDiv,

    U32CheckedMod,

    U32UncheckedMod,

    U32CheckedDivMod,

    U32UncheckedDivMod,

    // u32 bitwise operations
    U32CheckedAnd,

    U32CheckedOr,

    U32CheckedXor,

    U32CheckedNot,

    U32CheckedShl,

    U32UncheckedShl,

    U32CheckedShr,

    U32UncheckedShr,

    U32CheckedRotl,

    U32UncheckedRotl,

    U32CheckedRotr,

    U32UncheckedRotr,

    U32CheckedPopcnt,
    U32UncheckedPopcnt,

    // u32 comparison operations
    U32CheckedEq,

    U32CheckedNeq,

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

    String(String),

    Number(u64),

    Print,
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "proc" => Self::Proc,
            "add" => Self::Add,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "neg" => Self::Neg,
            "inv" => Self::Inv,
            "pow2" => Self::Pow2,
            "exp" => Self::Exp,
            "and" => Self::And,
            "or" => Self::Or,
            "xor" => Self::Xor,
            "not" => Self::Not,
            "eq" => Self::Eq,
            "neq" => Self::Neq,
            "lt" => Self::Lt,
            "lte" => Self::Lte,
            "gt" => Self::Gt,
            "gte" => Self::Gte,
            "ext2add" => Self::Ext2Add,
            "ext2sub" => Self::Ext2Sub,
            "ext2mul" => Self::Ext2Mul,
            "ext2neg" => Self::Ext2Neg,
            "ext2inv" => Self::Ext2Inv,
            "ext2div" => Self::Ext2Div,
            "while" => Self::While,
            "if" => Self::If,
            "end" => Self::End,
            "else" => Self::Else,
            "repeat" => Self::Repeat,
            "begin" => Self::Begin,
            "mem_load" => Self::MemLoad,
            "mem_store" => Self::MemStore,
            "mem_loadw" => Self::MemLoadW,
            "mem_storew" => Self::MemStoreW,
            "loc_load" => Self::LocLoad,
            "loc_store" => Self::LocStore,
            "loc_loadw" => Self::LocLoadW,
            "loc_storew" => Self::LocStoreW,
            "advpush" => Self::AdvPush,
            "push" => Self::Push,
            "drop" => Self::Drop,
            "dup" => Self::Dup,
            "swap" => Self::Swap,
            "swapw" => Self::SwapW,
            "swapdw" => Self::SwapDw,
            "padw" => Self::PadW,
            "movup" => Self::MovUp,
            "movupw" => Self::MovUpW,
            "movdn" => Self::MovDn,
            "movdnw" => Self::MovDnW,
            "exec" => Self::Exec,
            "is_odd" => Self::IsOdd,
            "eqw" => Self::EqW,
            "u32checked_add" => Self::U32CheckedAdd,
            "u32overflowing_add" => Self::U32OverflowingAdd,
            "u32wrapping_add" => Self::U32WrappingAdd,
            "u32overflowing_add3" => Self::U32OverflowingAdd3,
            "u32wrapping_add3" => Self::U32WrappingAdd3,
            "u32checked_sub" => Self::U32CheckedSub,
            "u32overflowing_sub" => Self::U32OverflowingSub,
            "u32wrapping_sub" => Self::U32WrappingSub,
            "u32checked_mul" => Self::U32CheckedMul,
            "u32overflowing_mul" => Self::U32OverflowingMul,
            "u32wrapping_mul" => Self::U32WrappingMul,
            "u32overflowing_madd" => Self::U32OverflowingMadd,
            "u32wrapping_madd" => Self::U32WrappingMadd,
            "u32checked_div" => Self::U32CheckedDiv,
            "u32unchecked_div" => Self::U32UncheckedDiv,
            "u32checked_mod" => Self::U32CheckedMod,
            "u32unchecked_mod" => Self::U32UncheckedMod,
            "u32checked_divmod" => Self::U32CheckedDivMod,
            "u32unchecked_divmod" => Self::U32UncheckedDivMod,
            "u32checked_and" => Self::U32CheckedAnd,
            "u32checked_or" => Self::U32CheckedOr,
            "u32checked_xor" => Self::U32CheckedXor,
            "u32checked_not" => Self::U32CheckedNot,
            "u32checked_shl" => Self::U32CheckedShl,
            "u32unchecked_shl" => Self::U32UncheckedShl,
            "u32checked_shr" => Self::U32CheckedShr,
            "u32unchecked_shr" => Self::U32UncheckedShr,
            "u32checked_rotl" => Self::U32CheckedRotl,
            "u32unchecked_rotl" => Self::U32UncheckedRotl,
            "u32checked_rotr" => Self::U32CheckedRotr,
            "u32unchecked_rotr" => Self::U32UncheckedRotr,
            "u32checked_popcnt" => Self::U32CheckedPopcnt,
            "u32unchecked_popcnt" => Self::U32UncheckedPopcnt,
            "u32checked_eq" => Self::U32CheckedEq,
            "u32checked_neq" => Self::U32CheckedNeq,
            "u32checked_lt" => Self::U32CheckedLt,
            "u32unchecked_lt" => Self::U32UncheckedLt,
            "u32checked_lte" => Self::U32CheckedLte,
            "u32unchecked_lte" => Self::U32UncheckedLte,
            "u32checked_gt" => Self::U32CheckedGt,
            "u32unchecked_gt" => Self::U32UncheckedGt,
            "u32checked_gte" => Self::U32CheckedGte,
            "u32unchecked_gte" => Self::U32UncheckedGte,
            "u32checked_min" => Self::U32CheckedMin,
            "u32unchecked_min" => Self::U32UncheckedMin,
            "u32checked_max" => Self::U32CheckedMax,
            "u32unchecked_max" => Self::U32UncheckedMax,
            "print" => Self::Print,
            _ => {
                if let Ok(number) = value.parse::<u64>() {
                    Self::Number(number)
                } else {
                    Self::String(value.to_string())
                }
            }
        }
    }
}
