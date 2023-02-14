use super::*;

pub struct RuntimeError {
    message: String,
}

impl NoxError {
    pub fn runtime_error<S: Into<String>>(message: S) -> Self {
        Self {
            kind: NoxErrorKind::RuntimeError(Box::new(RuntimeError { message: message.into() }))
        }
    }
}