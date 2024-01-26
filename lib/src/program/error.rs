#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MidenProgramError {
    LocStoreInBegin,
    LocLoadInBegin,
    DuplicateProcName(String),
    LocalProcNotFound(u16, String),
    ParamOutOfBounds(u64, u64, u64),
    AdviceStackReadOutOfBounds(usize, usize),
    DivisionByZero,
    NotBinaryValue(u64),
}

impl std::fmt::Display for MidenProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalProcNotFound( proc_idx, module_path) => write!(
                f,
                "procedure at index {proc_idx} not found in module {module_path}"
            ),
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::ParamOutOfBounds( value, min, max) => write!(f, "parameter value must be greater than or equal to {min} and less than or equal to {max}, but was {value}"),
            Self::DuplicateProcName( name) => write!(f, "duplicate procedure name {name}"),
            Self::LocStoreInBegin => write!(f, "cannot store to local in BEGIN block"),
            Self::LocLoadInBegin => write!(f, "cannot load from local in BEGIN block"),
            Self::AdviceStackReadOutOfBounds( value, max) => write!(f, "advice stack read out of bounds: {value} > {max}"),
            Self::NotBinaryValue( value) => write!(f, "value : {value} is not binary"),
        }
    }
}
