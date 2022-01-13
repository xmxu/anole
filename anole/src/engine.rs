use std::time::Instant;

use log::{debug, info};

use crate::{context::Context, task::http::HttpTask};
use crate::task::db::mysql::MysqlTask;

#[derive(Debug)]
pub struct Engine<'a> {
    ctx: Box<Context>,
    tasks: Vec<HttpTask<'a>>,
    db_task: Option<MysqlTask<'a>>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Engine { ctx: Box::new(Context::default()), tasks: vec![], db_task: None }
    }

    pub fn with_http(mut self, t: HttpTask<'a>) -> Self {
        self.tasks.push(t);
        self
    }

    pub fn with_mysql(mut self, t: MysqlTask<'a>) -> Self {
        self.db_task = Some(t);
        self
    }

    pub async fn run(mut self) -> crate::Result<()> {
        let cost = Instant::now();
        for ele in self.tasks.into_iter() {
            match ele.execute(self.ctx.as_mut()).await {
                Ok(_) => continue,
                Err(e) => return Err(e)
            };
        }

        if let Some(db_t) = self.db_task {
            match db_t.execute(self.ctx.as_mut()).await {
                Ok(_) => (),
                Err(e) => return Err(e)
            };
        }

        debug!("store:{:?}", self.ctx.store);
        self.ctx.store.clear();
        info!("execute completed! cost_time:{:?}", cost.elapsed());
        Ok(())
    }
}

impl <'a> Default for Engine<'a> {
    fn default() -> Self {
        Self::new()
    }
}