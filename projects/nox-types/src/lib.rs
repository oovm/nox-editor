use std::time::SystemTime;

pub use crate::cell::NotebookCell;
pub use crate::errors::*;

mod errors;
mod cell;
mod server;
mod client;


pub struct CommandOutput {

    pub command: String,
    pub args: Vec<String>,
}

// runner task
pub struct ServerEvent {}

// web render task
pub struct ClientEvent {
    notebook: u64,
    time: SystemTime,
}
