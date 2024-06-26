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
    HostError(String),          // Errors causing by host/port.
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
                write!(f, "* With: {cause}")
            },
            DTError::RemoteError(msg) => write!(f, "[Remote] {msg}"),
            DTError::HostError(msg) => writeln!(f, "{msg}")
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
            DTError::HostError(_) => None,
        }
    }
}

#[allow(unused)]
pub(crate) mod macros {
    macro_rules! verify_error {
        ($($arg:tt)*) => {
            Err($crate::util::error::DTError::VerifyError(format!($($arg)*)))
        };
    }

    macro_rules! internal_error {
        ($($arg:tt)*) => {
            Err($crate::util::error::DTError::InternalError(format!($($arg)*)))
        };
    }

    macro_rules! runtime_error {
        ($($arg:tt)*) => {
            Err($crate::util::error::DTError::RuntimeError(format!($($arg)*)))
        };
    }

    macro_rules! network_error {
        ($($arg:tt)*) => {
            Err($crate::util::error::DTError::NetworkError(format!($($arg)*)))
        };
    }

    macro_rules! error_with {
        ($err:ident, $($arg:tt)*) => {
            Err($crate::util::error::DTError::WithContext {
                context: format!($($arg)*),
                cause: Box::new($err)
            })
        };
    }

    macro_rules! remote_error {
        ($($arg:tt)*) => {
            Err($crate::util::error::DTError::RemoteError(format!($($arg)*)))
        };
    }

    macro_rules! host_error {
        ($($arg:tt)*) => {
            Err($crate::util::error::DTError::HostError(format!($($arg)*)))
        };
    }

    pub(crate) use verify_error;
    pub(crate) use internal_error;
    pub(crate) use runtime_error;
    pub(crate) use network_error;
    pub(crate) use error_with;
    pub(crate) use remote_error;
    pub(crate) use host_error;
}