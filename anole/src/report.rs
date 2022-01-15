use std::sync;


//exporter

#[derive(Debug)]
pub struct ReportItem {
    pub task_id: String,
    pub code: i32,
    pub description: String,
}

impl ReportItem {
    pub fn new(task_id: String, code: i32, description: String) -> Self {
        ReportItem {
            task_id, code, description
        }
    }
}

pub trait Reporter {

    fn report(&self, item: ReportItem);
}

pub struct StdReporter {
    pub sender: sync::mpsc::Sender<ReportItem>,
}

impl StdReporter {
    pub fn new(s: sync::mpsc::Sender<ReportItem>) -> Self {
        StdReporter {
            sender: s
        }
    }
}

impl Reporter for StdReporter {

    fn report(&self, item: ReportItem) {
        self.sender.send(item).unwrap();
    }
    
}