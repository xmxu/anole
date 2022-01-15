use crate::{store::Store, report::{Reporter, ReportItem}};

pub struct Context {
    pub store: Store,
    pub reporter: Option<Box<dyn Reporter>>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Context {
            store: Store::new(),
            reporter: None
        }
    }

    pub(crate) fn with_reporter(&mut self, r: Box<dyn Reporter>) {
        self.reporter = Some(r);
    }

    pub(crate) fn report(&self, r: ReportItem) {
        if let Some(reporter) = &self.reporter {
            reporter.report(r);
        }
    }
}
