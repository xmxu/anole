
use anole::{engine::Engine, task::http::{HttpTaskBuilder, Method}, capture, value};

#[macro_use]
extern crate log;

#[tokio::main]
#[test]
async fn test_engine() {
    info!("startup");
    Engine::new()
        .with_http(HttpTaskBuilder::new()
            .url("https://tvapi.dykkan.com/v1/tags")
            .method(Method::Get)
            .capture(vec![
                capture::json("data|1", "tag"),
                capture::header("content-length", "cl"),
            ])
            .expect(("tag", value::Value::Str("language".to_string())))
            .build())
        .with_http(HttpTaskBuilder::new()
            .url("https://tvapi.dykkan.com/v1/tag/:tag")
            .method(Method::Get)
            .verbose(false)
            .build())
        .run().await.unwrap();
    
}