use crate::NoxError;

impl From<tungstenite::Error> for NoxError {
    fn from(value: tungstenite::Error) -> Self {
        NoxError::runtime_error(value.to_string())
    }
}