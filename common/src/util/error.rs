use std::fmt;
use std::error::Error;
use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, DTError>;

#[derive(Debug)]
pub enum DTError {
    VerifyError(String),
    InternalError(String),
    RuntimeError(String),
    WithContext{
        context: String,
        cause: Box<DTError>
    }
}

impl fmt::Display for DTError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DTError::VerifyError(msg) => write!(f, "{msg}"),
            DTError::InternalError(msg) => write!(f, "Internal Error! {msg}"),
            DTError::RuntimeError(msg) => write!(f, "{msg}"),
            DTError::WithContext {context, cause} => {
                writeln!(f, "{context}")?;
                write!(f, "with: {cause}")
            }
        }
    }
}

impl Error for DTError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DTError::VerifyError(_) => None,
            DTError::InternalError(_) => None,
            DTError::RuntimeError(_) => None,
            DTError::WithContext {cause, .. } => Some(cause),
        }
    }
}

pub(crate) mod macros {
    macro_rules! verify_error {
        ($($arg:tt)*) => {
            Err(DTError::VerifyError(format!($($arg)*)))
        };
    }

    macro_rules! internal_error {
        ($($arg:tt)*) => {
            Err(DTError::InternalError(format!($($arg)*)))
        };
    }

    macro_rules! runtime_error {
        ($($arg:tt)*) => {
            Err(DTError::RuntimeError(format!($($arg)*)))
        };
    }

    macro_rules! error_with {
        ($err:ident, $($arg:tt)*) => {
            Err(DTError::WithContext {
                context: format!($($arg)*),
                cause: Box::new($err)
            })
        };
    }

    pub(crate) use verify_error;
    pub(crate) use internal_error;
    pub(crate) use runtime_error;
    pub(crate) use error_with;
}