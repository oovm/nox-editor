pub use for_3rd::*;

use crate::errors::runtime::RuntimeError;

pub type NoxResult<T = ()> = Result<T, NoxError>;

pub struct NoxError {
    kind: NoxErrorKind,
}

pub enum NoxErrorKind {
    RuntimeError(Box<RuntimeError>)
}

pub mod runtime;
mod for_3rd;

