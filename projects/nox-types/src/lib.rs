use std::time::SystemTime;

pub use crate::cell::NotebookCellData;
pub use crate::errors::*;
pub use crate::users::UserID;

mod errors;
mod cell;
mod server;
mod client;
mod users;


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
