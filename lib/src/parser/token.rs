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

    String(String),

    Number(u64),
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
