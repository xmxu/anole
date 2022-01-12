use std::{collections::HashMap, time::Duration};
use reqwest::Response;

use crate::{value::{Value, self, Body}, capture::Capture, context::Context};

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

impl From<&Method> for reqwest::Method {
    fn from(val: &Method) -> Self {
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
        .connection_verbose(self.config.verbose)
        .user_agent(format!("anole_client_{}", env!("CARGO_PKG_VERSION"))).build().unwrap();

        let mut url = url::Url::parse(&self.config.url).unwrap();
        if let Some(mut path_segments) = url.path_segments() {
            let mut paths: Vec<String> = vec![];
            for p in path_segments.by_ref() {
                if p.starts_with(':') {
                    let k = &p[1..p.len()];
                    if let Some(v) = ctx.store.get(k.to_string()) {
                        let vv = v.as_str();
                        paths.push(vv);
                    }
                } else {
                    paths.push(p.to_string());
                }
            }
            let path = paths.into_iter().reduce(|mut p, x| {
                p.push('/');
                p.push_str(&x);
                p
            } ).unwrap();
            url.set_path(&path);
        }
        let method: reqwest::Method = reqwest::Method::from(&self.config.method);
        let mut request_builder = client.request(method, url.as_str());

        //header
        if let Some(h) = &self.config.header {
            let mut h = h.to_owned();
            for (k, v) in h.iter_mut() {
                if let Some(wildcard) = v.as_wildcard() {
                    if let Some(wv) = ctx.store.get(wildcard) {
                        *v = wv.to_owned();
                    }
                }
                let vstr = v.as_str();
                request_builder = request_builder.header(k.as_str(), &vstr);
            }
        }

        //query
        if let Some(q) = &self.config.query {
            let mut q = q.to_owned();
            for (_, v) in q.iter_mut() {
                if let Some(wildcard) = v.as_wildcard() {
                    if let Some(wv) = ctx.store.get(wildcard) {
                        *v = wv.to_owned();
                    }
                }
            }
            request_builder = request_builder.query(&q);
        }
        //form
        if let Some(f) = &self.config.form {
            let mut f = f.to_owned();
            for (_, v) in f.iter_mut() {
                if let Some(wildcard) = v.as_wildcard() {
                    if let Some(wv) = ctx.store.get(wildcard) {
                        *v = wv.to_owned();
                    }
                }
            }
            request_builder = request_builder.form(&f);
        }
        //body
        if let Some(b) = &self.config.body {
            if let Some(bb) = b.as_bytes(ctx) {
                request_builder = request_builder.body(bb);
            }
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
                let headers = &_rsp.headers().to_owned();
                let body_value = _rsp.json::<serde_json::Value>().await.unwrap();
                for _cap in _caps {
                    let _ = match _cap {
                        Capture::Header(_c) => {
                            if let Some(v) = headers.get(&_c.key) {
                                ctx.store.set(_c.save_key, Value::Str(v.to_str().unwrap().to_string()));
                            }
                        },
                        Capture::Json(_c) => {
                            if !body_value.is_null() {
                                if let Some(cv) = value::parse_json_value(&body_value, _c.key) {
                                    if !cv.is_null() {
                                        ctx.store.set(_c.save_key, Value::from(&cv));
                                    }
                                }                               
                            }
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
    pub(crate) header: Option<HashMap<String, Value>>,
    pub(crate) query: Option<HashMap<String, Value>>,
    pub(crate) form: Option<HashMap<String, Value>>,
    pub(crate) body: Option<Body>,
    pub(crate) capture: Option<Vec<Capture>>,
    pub(crate) verbose: bool,
}

impl HttpTaskBuilder {
    pub fn new() -> Self {
        HttpTaskBuilder {
            url: String::new(),
            method: Method::Get,
            serializer: Serializer::Json,
            header: None,
            query: None,
            form: None,
            body: None,
            capture: None,
            verbose: false
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

    pub fn header(mut self, header: (String, Value)) -> Self {
        if self.header.is_none() {
            self.header = Some(HashMap::new())
        }
        if let Some(ref mut h) = self.header {
            h.insert(header.0, header.1);
        }
        self
    }

    pub fn query(mut self, query: (String, Value)) -> Self {
        if self.query.is_none() {
            self.query = Some(HashMap::new())
        }
        if let Some(ref mut q) = self.query {
            q.insert(query.0, query.1);
        }
        self
    }

    pub fn form(mut self, form: (String, Value)) -> Self {
        if self.form.is_none() {
            self.form = Some(HashMap::new())
        }
        if let Some(ref mut f) = self.form {
            f.insert(form.0, form.1);
        }
        self
    }

    pub fn body(mut self, body: Body) ->Self {
        self.body = Some(body);
        self
    }

    pub fn capture(mut self, capture: Vec<Capture>) -> Self {
        self.capture = Some(capture);
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
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
