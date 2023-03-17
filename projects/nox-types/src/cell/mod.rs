use std::collections::BTreeSet;
use std::time::SystemTime;

use crate::UserID;

pub struct NotebookCellData {
    // running order in kernel
    cell_id: u64,
    // all user who modify this cell
    author_id: BTreeSet<UserID>,
    language: String,
    is_pin: bool,
    source_code: String,
    outputs: Vec<NotebookOutputKind>,
    create_time: SystemTime,
    // start to run
    start_time: SystemTime,
}

pub struct NotebookOutputData {
    kind: NotebookOutputKind,
    modifier_time: SystemTime,
}

pub enum NotebookOutputKind {
    Text(Box<NotebookText>),
    Table(String),
    Plot(String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotebookTextKind {
    Normal,
    Help,
    Warning,
    Error,
    Fatal,
}

pub struct NotebookText {
    kind: NotebookTextKind,
    html: String,
}


impl NotebookCellData {
    pub fn rerun(&mut self, author: UserID) {
        self.author_id.insert(author);
        self.start_time = SystemTime::now();
    }

    // human friendly time, no localization
    pub fn cost_time(&self) -> String {
        let cost = self.start_time.duration_since(self.create_time).unwrap();
        format!("{:?}", cost)
    }
}