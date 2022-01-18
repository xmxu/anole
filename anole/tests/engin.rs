
use anole::{engine::Engine, task::http::{HttpTaskBuilder, Method}, capture, value};

#[macro_use]
extern crate log;

#[tokio::main]
#[test]
async fn test_engine() {
    info!("startup");
    Engine::new()
        .with_http(HttpTaskBuilder::new()
            .url("https://crates.io/api/v1/crates?page=1&per_page=2&q=json")
            .method(Method::Get)
            .capture(vec![
                capture::json("crates|0.name", "crate_name"),
                capture::header("content-length", "cl"),
            ])
            .expect(("crate_name", value::Value::Str("json".to_string())))
            .build())
        .with_http(HttpTaskBuilder::new()
            .url("https://crates.io/api/v1/crates/:crate_name")
            .method(Method::Get)
            .verbose(false)
            .build())
        .run().await.unwrap();
    
}