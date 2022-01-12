use anole::{engine::Engine, task::http::{HttpTaskBuilder, Method}, capture};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("startup");
    Engine::new()
        .with_http(HttpTaskBuilder::new()
            .url("https://tvapi.dykkan.com/v1/tags".to_string())
            .method(Method::Get)
            .capture(vec![
                capture::json("data.1".to_string(), "tag".to_string()),
                capture::header("content-length".to_string(), "cl".to_string()),
            ])
            .build())
        .with_http(HttpTaskBuilder::new()
            .url("https://tvapi.dykkan.com/v1/tag/:tag".to_string())
            .method(Method::Get)
            .verbose(false)
            .build())
        .run().await.unwrap();
    
}