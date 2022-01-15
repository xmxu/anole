use std::sync;

use anole::{engine::Engine, task::http::{HttpTaskBuilder, Method, Deserializer}, capture, report::{ReportItem, StdReporter}};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("startup");

    let (sender, recv) = sync::mpsc::channel::<ReportItem>();

    tokio::spawn(async {
        for r in recv {
            info!("report:{:?}", r);
        }
    });

    Engine::new()
        .with_reporter(Box::new(StdReporter::new(sender)))
        .with_http(HttpTaskBuilder::new()
            .url("https://tvapi.dykkan.com/v1/tags")
            .method(Method::Get)
            .capture(vec![
                capture::json("data.1", "tag"),
                capture::header("content-length", "cl"),
            ])
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

    
    
}