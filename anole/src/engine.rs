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

    pub async fn run(mut self) {
        let cost = Instant::now();
        for ele in self.tasks.into_iter() {
            ele.execute(self.ctx.as_mut()).await;
        }

        debug!("store:{:?}", self.ctx.store);
        self.ctx.store.clear();
        info!("execute completed! cost_time:{:?}", cost.elapsed());
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}