
use std::{sync, time::Instant};

use anole::{engine::Engine, task::db::{mysql::{MysqlTask, DBTask}, DBClientOption}, capture, value, report::{ReportItem, self}};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("startup");

    let cost = Instant::now();

    let (sender, recv) = sync::mpsc::channel::<ReportItem>();

    let reporter_printer = tokio::spawn(async {
        for r in recv {
            info!("{}", r);
        }
    });

    Engine::new()
        .with_reporter(Box::new(report::StdReporter::new(sender)))
        .with_mysql(MysqlTask::default()
            .options(DBClientOption::builder().url("mysql://root:hsq847@localhost/anole").max_connections(5))
            .with_task(DBTask::new("SELECT count(*) as count FROM tbl_order")
                .capture(vec![
                    capture::column("count", "order_count", capture::CapValueType::Size)
                ]).expect(("order_count", value::Value::I64(1))))
            .with_task(DBTask::new("SELECT name, create_time FROM tbl_order LIMIT :order_count")
            .param("order_count")
            .capture(vec![
                capture::column("name", "order_name", capture::CapValueType::Str),
                capture::column("create_time", "order_date", capture::CapValueType::Date),
            ]))
        )
        .run().await.unwrap();

    reporter_printer.await.unwrap();
    
    info!("execute completed! cost_time:{:?}", cost.elapsed());
}