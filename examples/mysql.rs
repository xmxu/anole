
use anole::{engine::Engine, task::db::{mysql::{MysqlTask, DBTask}, DBClientOption}, capture};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("startup");
    Engine::new()
        .with_mysql(MysqlTask::new()
            .options(DBClientOption::new().url("mysql://root:hsq847@localhost/anole").max_connections(5))
            .with_task(DBTask::new("SELECT count(*) as count FROM tbl_order")
                .capture(vec![
                    capture::column("count", "order_count", capture::CapValueType::Size)
                ]))
            .with_task(DBTask::new("SELECT name, create_time FROM tbl_order LIMIT #order_count#")
            .param("order_count")
            .capture(vec![
                capture::column("name", "order_name", capture::CapValueType::Str),
                capture::column("create_time", "order_date", capture::CapValueType::Date),
            ]))
        )
        .run().await.unwrap();
    
}