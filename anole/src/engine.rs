

use log::debug;

use crate::report::Reporter;
use crate::task::Task;
use crate::{context::Context, task::http::HttpTask};
use crate::task::db::mysql::MysqlTask;

pub struct Engine<'a> {
    ctx: Box<Context>,
    tasks: Vec<Task<'a>>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Engine { ctx: Box::new(Context::new()), tasks: vec![] }
    }

    pub fn with_http(mut self, t: HttpTask<'a>) -> Self {
        self.tasks.push(t.into());
        self
    }

    pub fn with_mysql(mut self, t: MysqlTask<'a>) -> Self {
        self.tasks.push(t.into());
        self
    }

    pub fn with_reporter(mut self, r: Box<dyn Reporter>) -> Self {
        self.ctx.with_reporter(r);
        self
    }

    pub async fn run(mut self) -> crate::Result<()> {
        for mut ele in self.tasks.into_iter() {
            match ele.execute(self.ctx.as_mut()).await {
                Ok(_) => continue,
                Err(e) => return Err(e)
            };
        }

        debug!("store:{:?}", self.ctx.store);
        self.ctx.store.clear();
        Ok(())
    }
}

impl <'a> Default for Engine<'a> {
    fn default() -> Self {
        Self::new()
    }
}