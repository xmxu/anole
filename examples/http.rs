use std::{sync, time::Instant};

use anole::{engine::Engine, task::http::{HttpTaskBuilder, Method, Deserializer}, capture, report::{ReportItem, StdReporter}, value::Value};

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
        .with_reporter(Box::new(StdReporter::new(sender)))
        .with_http(HttpTaskBuilder::new()
            .url("https://crates.io/api/v1/crates?page=1&per_page=2&q=json")
            .method(Method::Get)
            .capture(vec![
                capture::json("crates|0.name", "crate_name"),
                capture::header("content-length", "cl"),
            ])
            .expect(("crate_name", Value::Str("json".to_string())))
            .build())
        .with_http(HttpTaskBuilder::new()
            .url("https://crates.io/api/v1/crates/:crate_name")
            .method(Method::Get)
            .verbose(false)
            .build())
        .with_http(HttpTaskBuilder::new()
            .url("https://repo1.maven.org/maven2/com/google/code/gson/gson/2.8.9/gson-2.8.9.pom")
            .deserializer(Deserializer::Xml)
            .capture(vec![
                capture::xml("modelVersion", "model_version"),
                capture::xml("dependencies.dependency|0.groupId", "group_id"),
            ])
            .expect(("group_id", Value::Str("junit".to_string())))
            .build())
        .run().await.unwrap();

        reporter_printer.await.unwrap();

        info!("execute completed! cost_time:{:?}", cost.elapsed());
    
    
}