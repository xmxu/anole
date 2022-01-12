use std::time::Instant;

use log::{debug, info};

use crate::{context::Context, task::http::HttpTask};

#[derive(Debug)]
pub struct Engine {
    ctx: Box<Context>,
    tasks: Vec<HttpTask>
}

impl Engine {
    pub fn new() -> Self {
        Engine { ctx: Box::new(Context::default()), tasks: vec![] }
    }

    pub fn with_http(mut self, t: HttpTask) -> Self {
        self.tasks.push(t);
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

        debug!("store:{:?}", self.ctx.store);
        self.ctx.store.clear();
        info!("execute completed! cost_time:{:?}", cost.elapsed());
        Ok(())
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}