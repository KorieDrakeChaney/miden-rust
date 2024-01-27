#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MidenProgramError {
    LocStoreInBegin,
    LocLoadInBegin,
    DuplicateProcName(String),
    LocalProcNotFound(u16, String),
    ParamOutOfBounds(u64, u64, u64),
    AdviceStackReadOutOfBounds(usize, usize),
    DivisionByZero,
    DivideByZero,
    NotBinaryValue(u64),
    InvalidParameter(String, usize, usize, usize),
    Pow2Overflow(u64),
}

impl std::fmt::Display for MidenProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalProcNotFound( proc_idx, module_path) => write!(
                f,
                "procedure at index {proc_idx} not found in module {module_path}"
            ),
            Self::DivisionByZero => write!(f, "InvalidParameter(0), div.0 is invalid, must be greater than 0"),
            Self::DivideByZero => write!(f, "The top value on the stack must be greater than 0"),
            Self::ParamOutOfBounds( value, min, max) => write!(f, "Parameter value must be greater than or equal to {min} and less than or equal to {max}, but was {value}"),
            Self::DuplicateProcName( name) => write!(f, "Duplicate procedure name {name}"),
            Self::LocStoreInBegin => write!(f, "Cannot store to local in BEGIN block"),
            Self::LocLoadInBegin => write!(f, "Cannot load from local in BEGIN block"),
            Self::AdviceStackReadOutOfBounds( value, max) => write!(f, "Advice stack read out of bounds: {value} > {max}"),
            Self::NotBinaryValue( value) => write!(f, "NotBinaryValue({value}), {value} is not binary"),
            Self::InvalidParameter(op, value, min, max) => write!(f, "InvalidParameter({value}), {op}.{value} is invalid, must be between {min} and {max}"),
            Self::Pow2Overflow(value) => write!(f, "Pow2Overflow({value}), {value} is too large to be an exponent of 2"),
        }
    }
}
