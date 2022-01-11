use std::{collections::HashMap, time::Duration};
use reqwest::Response;
use serde::ser::SerializeMap;

use crate::{value::Value, capture::Capture, context::Context};

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Head,
    Put,
    Delete,
    Patch,
}

#[derive(Debug)]
pub enum Serializer {
    Json,
    Xml,
    Protobuf,
}

#[derive(Debug)]
pub struct Query(HashMap<String, Value>);

impl serde::Serialize for Query {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
            
            let mut s = serializer.serialize_map(Some(self.0.len()))?;
            for (k, v) in &self.0 {
                s.serialize_entry(&k, &v)?;
            }
            s.end()
    }
}

impl From<Method> for reqwest::Method {
    fn from(val: Method) -> Self {
        match val {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Head => reqwest::Method::HEAD,
            Method::Delete => reqwest::Method::DELETE,
            Method::Patch => reqwest::Method::PATCH,
        }
    }
}

#[derive(Debug)]
pub struct HttpTask {
    pub(crate) config: HttpTaskBuilder,
    pub(crate) rsp: Option<Response>,
}


impl HttpTask {
    pub async fn execute(mut self, ctx: &mut Context) {
        let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(20))
        .user_agent("Anole Client").build().unwrap();

        let mut request_builder = client.request(reqwest::Method::GET, &self.config.url);
        if let Some(q) = &self.config.query {
            request_builder = request_builder.query(&q);
        }
        if let Some(f) = &self.config.form {
            request_builder = request_builder.form(&f);
        }
        if let Some(b) = &self.config.body {
            request_builder = request_builder.body(b.to_owned());
        }
        let rsp = request_builder.send().await.unwrap();
        let is_success = (&rsp.status()).is_success();
        if is_success {
            self.rsp = Some(rsp);
            self.capture(ctx).await;
        }
    }

    pub(crate) async fn capture(self, ctx: &mut Context) {
        if self.config.capture.is_none() { 
            return
        }
        if let Some(_rsp) = self.rsp {
            if let Some(_caps) = self.config.capture {
                serde_json::from_reader(_rsp.bytes().await.unwrap());
                for _cap in _caps {
                    let _ = match _cap {
                        Capture::Header(_c) => {
                            if let Some(v) = _rsp.headers().get(&_c.key) {
                                ctx.store.set(_c.key, Value::Str(v.to_str().unwrap().to_string()));
                            }
                        },
                        Capture::Json(_c) => {

                        },
                        _ => ()
                    }; 
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct HttpTaskBuilder {
    pub(crate) url: String,
    pub(crate) method: Method,
    pub(crate) serializer: Serializer,
    pub(crate) query: Option<HashMap<String, Value>>,
    pub(crate) form: Option<HashMap<String, Value>>,
    pub(crate) body: Option<bytes::Bytes>,
    pub(crate) capture: Option<Vec<Capture>>,
}

impl HttpTaskBuilder {
    pub fn new() -> Self {
        HttpTaskBuilder {
            url: String::new(),
            method: Method::Get,
            serializer: Serializer::Json,
            query: None,
            form: None,
            body: None,
            capture: None,
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn serializer(mut self, serializer: Serializer) -> Self {
        self.serializer = serializer;
        self
    }

    pub fn query(mut self, query: HashMap<String, Value>) -> Self {
        self.query = Some(query);
        self
    }

    pub fn form(mut self, form: HashMap<String, Value>) -> Self {
        self.form = Some(form);
        self
    }

    pub fn body(mut self, body: bytes::Bytes) ->Self {
        self.body = Some(body);
        self
    }

    pub fn capture(mut self, capture: Vec<Capture>) -> Self {
        self.capture = Some(capture);
        self
    }

    pub fn build(self) -> HttpTask {
        HttpTask { config: self, rsp: None }
    }

}

impl Default for HttpTaskBuilder {
    fn default() -> Self {
        Self::new()
    }
}
