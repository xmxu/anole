use anole::{engine::Engine, task::http::{HttpTaskBuilder, Method, Deserializer}, capture};

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
        .with_http(HttpTaskBuilder::new()
            .url("http://192.168.168.3:9998/repository/maven-releases/com/husky/unity/plugin/core/1.0.3/core-1.0.3.pom".to_string())
            .deserializer(Deserializer::Xml)
            .capture(vec![
                capture::xml("modelVersion".to_string(), "model_version".to_string()),
                capture::xml("modelVersion".to_string(), "model_version".to_string()),
                capture::xml("dependencies.dependency|1.groupId".to_string(), "group_id".to_string()),
            ])
            .build())
        .run().await.unwrap();
    
}