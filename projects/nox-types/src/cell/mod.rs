use std::time::SystemTime;

pub struct NotebookCell {
    language: String,
    create_time: SystemTime,
    run_time: SystemTime,
    modified_time: SystemTime,
}


impl NotebookCell {
    // human friendly time, no localization
    pub fn cost_time(&self) -> String {
        let cost = self.run_time.duration_since(self.create_time).unwrap();
        format!("{:?}", cost)
    }
}