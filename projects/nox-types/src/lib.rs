pub use crate::errors::*;

mod errors;


pub struct CommandOutput {
    pub command: String,
    pub args: Vec<String>,
}

#[async_trait::async_trait]
pub trait NoxProtobuf {
    fn run_command(&self, command: &str) -> NoxResult;
}