use std::fmt;
use std::error::Error;
use std::fmt::Formatter;

pub type Result<T> = std::result::Result<T, DTError>;

#[derive(Debug)]
pub enum DTError {
    VerifyError(String),        // data passed is invalid.
    InternalError(String),      // due to internal reason.
    RuntimeError(String),       // error other than others.
    NetworkError(String),       // due to network reason.
    WithContext{                // wrap with extra context.
        context: String,
        cause: Box<DTError>
    },
    RemoteError(String),        // due to remote reason.
}

impl fmt::Display for DTError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DTError::VerifyError(msg) => write!(f, "{msg}"),
            DTError::InternalError(msg) => write!(f, "[Internal] {msg}"),
            DTError::RuntimeError(msg) => write!(f, "{msg}"),
            DTError::NetworkError(msg) => write!(f, "[Network] {msg}"),
            DTError::WithContext {context, cause} => {
                writeln!(f, "{context}")?;
                write!(f, "with: {cause}")
            },
            DTError::RemoteError(msg) => write!(f, "[Remote] {msg}"),
        }
    }
}

impl Error for DTError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DTError::VerifyError(_) => None,
            DTError::InternalError(_) => None,
            DTError::RuntimeError(_) => None,
            DTError::NetworkError(_) => None,
            DTError::WithContext {cause, .. } => Some(cause),
            DTError::RemoteError(_) => None,
        }
    }
}

#[allow(unused)]
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

    macro_rules! network_error {
        ($($arg:tt)*) => {
            Err(DTError::NetworkError(format!($($arg)*)))
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

    macro_rules! remote_error {
        ($($arg:tt)*) => {
            Err(DTError::RemoteError(format!($($arg)*)))
        };
    }

    pub(crate) use verify_error;
    pub(crate) use internal_error;
    pub(crate) use runtime_error;
    pub(crate) use network_error;
    pub(crate) use error_with;
    pub(crate) use remote_error;
}