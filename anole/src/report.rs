use std::sync;


//exporter

#[derive(Debug)]
pub struct ReportItem {
    pub(crate) task_id: String,
    /// Reporter description, include error message when task is failed.
    pub description: String,
    pub(crate) success: bool,
}

impl ReportItem {

    pub(crate) fn success(task_id: &str, description: String) -> Self {
        ReportItem {
            task_id: task_id.to_string(), description, success: true
        }
    }

    pub(crate) fn failed(task_id: &str, description: String) -> Self {
        ReportItem {
            task_id: task_id.to_string(), description, success: false
        }
    }

}

impl std::fmt::Display for ReportItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.success {
            write!(f, "{} SUCCESS", self.task_id)
        } else{
            write!(f, "{} FAILED [{}]", self.task_id, self.description)
        }
    }
}

/// Task execute result reporter
pub trait Reporter {

    fn report(&self, item: ReportItem);
}

/// Reporter using mpsc
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