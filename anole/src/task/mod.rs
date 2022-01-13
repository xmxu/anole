use crate::context::Context;

use self::{http::HttpTask, db::mysql::MysqlTask};


pub mod chain;
pub mod http;
pub mod db;

#[derive(Debug)]
pub enum Task<'a> {
    Http(HttpTask<'a>),
    Mysql(MysqlTask<'a>),
}

impl<'a> Task<'a> {
    pub async fn execute(self, ctx: &mut Context) -> crate::Result<()> {
        match self {
            Self::Http(t) => t.execute(ctx).await,
            Self::Mysql(t) => t.execute(ctx).await
        }
    }
}

impl<'a> From<HttpTask<'a>> for Task<'a> {
    fn from(t: HttpTask<'a>) -> Self {
        Task::Http(t)
    }
}

impl<'a> From<MysqlTask<'a>> for Task<'a> {
    fn from(t: MysqlTask<'a>) -> Self {
        Task::Mysql(t)
    }
}

