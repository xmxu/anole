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
            .url("https://tvapi.dykkan.com/v1/tags")
            .method(Method::Get)
            .capture(vec![
                capture::json("data|1", "tag"),
                capture::header("content-length", "cl"),
                capture::json("code", "code")
            ])
            .expect(("tag", Value::Str("language".to_string())))
            .build())
        .with_http(HttpTaskBuilder::new()
            .url("https://tvapi.dykkan.com/v1/tag/:tag")
            .method(Method::Get)
            .verbose(false)
            .build())
        .with_http(HttpTaskBuilder::new()
            .url("http://192.168.168.3:9998/repository/maven-releases/com/husky/unity/plugin/core/1.0.3/core-1.0.3.pom")
            .deserializer(Deserializer::Xml)
            .capture(vec![
                capture::xml("modelVersion", "model_version"),
                capture::xml("modelVersion", "model_version"),
                capture::xml("dependencies.dependency|1.groupId", "group_id"),
            ])
            .build())
        .run().await.unwrap();

        reporter_printer.await.unwrap();

        info!("execute completed! cost_time:{:?}", cost.elapsed());
    
    
}