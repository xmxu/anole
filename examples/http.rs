use anole::{engine::Engine, task::http::HttpTaskBuilder, capture};

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("startup");
    Engine::new()
        .with_http(HttpTaskBuilder::new()
        .url("https://tvapi.dykkan.com/v1/tags".to_string())
        .capture(vec![
            capture::header("host".to_string(), "host".to_string()),
            capture::header("date".to_string(), "header_date".to_string()),
            capture::header("content-type".to_string(), "header_content-type".to_string()),
            capture::json("code".to_string(), "code".to_string()),
            capture::json("data.0".to_string(), "data.0".to_string()),
        ])
        .build())
        .run().await;

    
}