#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MidenProgramError {
    LocStoreInBegin,
    LocLoadInBegin,
    DuplicateProcName(String),
    LocalProcNotFound(u16, String),
    ParamOutOfBounds(u64, u64, u64),
    AdviceStackReadOutOfBounds(usize, usize),
    DivideByZero,
    ModulusByZero,
    DivModByZero,
    NotBinaryValue(u64),
    InvalidParameter(String, usize, usize, usize),
    Pow2Overflow(u64),
    NotU32Value(u64),
    U32Overflow(u64),
    U32InvalidSubtraction(u64, u64),
    TopValueInvalid(String, usize, usize, usize),
}

impl std::fmt::Display for MidenProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalProcNotFound( proc_idx, module_path) => write!(
                f,
                "procedure at index {proc_idx} not found in module {module_path}"
            ),
            Self::DivideByZero => write!(f, "The top value on the stack must be greater than 0"),
            Self::ModulusByZero => write!(f, "The top value on the stack must be greater than 0"),
            Self::DivModByZero => write!(f, "The top value on the stack must be greater than 0"),
            Self::ParamOutOfBounds( value, min, max) => write!(f, "Parameter value must be greater than or equal to {min} and less than or equal to {max}, but was {value}"),
            Self::DuplicateProcName( name) => write!(f, "Duplicate procedure name {name}"),
            Self::LocStoreInBegin => write!(f, "Cannot store to local in BEGIN block"),
            Self::LocLoadInBegin => write!(f, "Cannot load from local in BEGIN block"),
            Self::AdviceStackReadOutOfBounds( value, max) => write!(f, "Advice stack read out of bounds: {value} > {max}"),
            Self::NotBinaryValue( value) => write!(f, "NotBinaryValue({value}), {value} is not binary"),
            Self::InvalidParameter(op, value, min, max) => write!(f, "InvalidParameter({value}), {op} is invalid, must be between {min} and {max}"),
            Self::TopValueInvalid(op, value, min, max) => write!(f, "TopValueInvalid({value}), {op} is invalid, must be between {min} and {max}"),
            Self::Pow2Overflow(value) => write!(f, "Pow2Overflow({value}), {value} is too large to be an exponent of 2"),
            Self::NotU32Value(value) => write!(f, "NotU32Value({value}), {value} is not a u32 value"),
            Self::U32Overflow(value) => write!(f, "U32Overflow({value}), {value} is too large to be a u32 value"),
            Self::U32InvalidSubtraction(a, b) => write!(f, "U32InvalidSubtraction({a}), {a} is less than {b}"),

        }
    }
}
