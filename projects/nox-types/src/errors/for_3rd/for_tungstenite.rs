use tungstenite::Error;
use crate::NoxError;

impl From<Error> for NoxError {
    fn from(value: Error) -> Self {
        NoxError::runtime_error(value.to_string())
    }
}