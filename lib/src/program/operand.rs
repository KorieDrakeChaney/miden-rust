use math::fields::f64::BaseElement;

use super::error::MidenProgramError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
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
    Eqw,
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
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::Eqw => write!(f, "eqw"),
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
        }
    }
}
