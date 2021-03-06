use std::{collections::HashMap, time::Duration};
use reqwest::Response;

use crate::{value::{Value, self, Body}, capture::Capture, context::Context, de::xml, report::ReportItem};

/// HTTP Methods.
pub enum Method {
    Get,
    Post,
    Head,
    Put,
    Delete,
    Patch,
}

/// HTTP body deserializer
pub enum Deserializer {
    /// Json
    Json,
    /// XML
    Xml,
    /// Protobuf (Not support yet).
    Protobuf(String),
}

impl Deserializer {
    pub fn json(&self, b: &str) -> crate::Result<serde_json::Value> {
        match serde_json::from_str::<serde_json::Value>(b) {
            Ok(v) => Ok(v),
            Err(e) => Err(crate::error::decode(e.into()))
        }
    }

    pub fn xml(&self) -> Option<&[u8]> {
        None
    }

    pub fn is_json(&self) -> bool {
        matches!(self, Deserializer::Json)
    }

    pub fn is_xml(&self) -> bool {
        matches!(self, Deserializer::Xml)
    }

    pub fn is_pb(&self) -> bool {
        matches!(self, Deserializer::Protobuf(_))
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

/// An HTTP task.
pub struct HttpTask<'a> {
    pub(crate) config: HttpTaskBuilder<'a>,
    pub(crate) task_id: String,
}


impl HttpTask<'_> {
    pub async fn execute(&mut self, ctx: &mut Context) -> crate::Result<()> {
        let client = match reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .connection_verbose(self.config.verbose)
        .user_agent(format!("AnoleClient/{}", env!("CARGO_PKG_VERSION"))).build() {
            Ok(c) => c,
            Err(e) => return Err(crate::error::create_client(e.into()))
        };

        let mut url = match url::Url::parse(self.config.url) {
            Ok(u) => u,
            Err(e) => return Err(crate::error::parse_value(e.into()))
        };
        if let Some(mut path_segments) = url.path_segments() {
            let mut paths: Vec<String> = vec![];
            for p in path_segments.by_ref() {
                if p.starts_with(':') {
                    let k = &p[1..p.len()];
                    if let Some(v) = ctx.store.get(k.to_string()) {
                        let vv = v.as_str();
                        paths.push(vv);
                    } else {
                        ctx.report(ReportItem::failed(&self.task_id.to_owned(), format!("{} (not found '{}' value)", url, p)));
                        return Ok(());
                    }
                } else {
                    paths.push(p.to_string());
                }
            }
            if let Some(path) = paths.into_iter().reduce(|mut p, x| {
                p.push('/');
                p.push_str(&x);
                p
            }) {
                url.set_path(&path);
            }
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
                request_builder = request_builder.header(*k, v.as_str());
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
        let rsp = match request_builder.send().await {
            Ok(r) => r,
            Err(e) => return Err(crate::error::request(e.into()))
        };
        let status_code = &rsp.status().as_u16();
        let task_id = self.task_id.to_owned();
        let is_success = (&rsp.status()).is_success();
        let mut report_item = ReportItem::failed(&task_id, format!("{} (status_code:{})", url, status_code));
        if is_success {
            match self.capture(ctx, rsp).await {
                Ok(_) => {
                    if let Some(_expect) = &self.config.expect {
                        if let Some(_capture_value) = ctx.store.get(_expect.0.to_string()) {
                            if _expect.1 == *_capture_value {
                                report_item = ReportItem::success(&task_id, format!("{} expect pass", _expect.0));
                            } else {
                                report_item = ReportItem::failed(&task_id, format!("{} ({} expect {:?} but {:?})", url, _expect.0, _expect.1, _capture_value));
                            }
                        } else {
                            report_item = ReportItem::failed(&task_id, format!("{}({} expect {:?} but not found)", url, _expect.0, _expect.1));
                        }
                    } else {
                        report_item = ReportItem::success(&task_id, format!("{} succeed", url));
                    }
                },
                Err(e) => return Err(e)
            }
        } 
        ctx.report(report_item);
        Ok(())
    }

    pub(crate) async fn capture(&mut self, ctx: &mut Context, rsp: Response) -> crate::Result<()> {
        if self.config.capture.is_none() { 
            return Ok(())
        }
        if let Some(ref _caps) = self.config.capture {
            if let Some(header_caps) = self.config.filter_caps(|c| c.is_header()) {
                let headers = rsp.headers().clone();
                for _cap in header_caps {
                    if let Capture::Header(ref _c) = _cap {
                        if let Some(v) = headers.get(_c.key) {
                            if let Ok(hv) = v.to_str() {
                                ctx.store.set(_c.save_key.to_owned(), Value::Str(hv.to_string()));
                            }
                        }
                    }
                }
            }

            if let Ok(text) = rsp.text().await {
                if self.config.deserializer.is_json() {
                    if let Some(ref json_caps) = self.config.filter_caps(|c| c.is_json()) {
                        let json_values = match self.config.deserializer.json(&text) {
                            Ok(v) => v,
                            Err(e) => return Err(e) 
                        };
                        if !json_values.is_null() {
                            for _cap in json_caps {
                                if let Capture::Json(_c) = _cap {
                                    if let Some(cv) = value::parse_json_value(&json_values, _c.key.to_owned()) {
                                        if !cv.is_null() {
                                            ctx.store.set(_c.save_key.to_owned(), Value::from(&cv));
                                        }
                                    }       
                                }
                            }
                        }
                    }
                } else if self.config.deserializer.is_xml() {
                    if let Some(ref xml_caps) = self.config.filter_caps(|c| c.is_xml()) {
                        for _cap in xml_caps {
                            if let Capture::Xml(_c) = _cap {
                                if let Ok(cv) = xml::De::get(&text, _c.key) {
                                    ctx.store.set(_c.save_key.to_owned(), cv);
                                }
                            }
                        }
                    }
                }
            }

        }
        Ok(())
    }
}

/// HTTP task builder.
/// # Example
/// 
/// ```
/// use anole::task::http::{HttpTaskBuilder, Method};
/// 
/// let http_task = HttpTaskBuilder::new()
///     .url("https://crates.io/crates/sqlx")
///     .method(Method::Get)
///     .build();
/// ```
pub struct HttpTaskBuilder<'a> {
    pub(crate) url: &'a str,
    pub(crate) method: Method,
    pub(crate) deserializer: Deserializer,
    pub(crate) header: Option<HashMap<&'a str, Value>>,
    pub(crate) query: Option<HashMap<&'a str, Value>>,
    pub(crate) form: Option<HashMap<&'a str, Value>>,
    pub(crate) body: Option<Body>,
    pub(crate) capture: Option<Vec<Capture<'a>>>,
    pub(crate) verbose: bool,
    pub(crate) expect: Option<(&'a str, Value)>,
}

impl<'a> HttpTaskBuilder<'a> {
    /// Create an builder.
    pub fn new() -> Self {
        HttpTaskBuilder {
            url: "",
            method: Method::Get,
            deserializer: Deserializer::Json,
            header: None,
            query: None,
            form: None,
            body: None,
            capture: None,
            verbose: false,
            expect: None,
        }
    }

    /// Add http url.
    pub fn url(mut self, url: &'a str) -> Self {
        self.url = url;
        self
    }

    /// Add http method.
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Specify deserializer for http response body.
    pub fn deserializer(mut self, deserializer: Deserializer) -> Self {
        self.deserializer = deserializer;
        self
    }

    /// Add http header.
    pub fn header(mut self, header: (&'a str, Value)) -> Self {
        if self.header.is_none() {
            self.header = Some(HashMap::new())
        }
        if let Some(ref mut h) = self.header {
            h.insert(header.0, header.1);
        }
        self
    }

    /// Add http query params.
    pub fn query(mut self, query: (&'a str, Value)) -> Self {
        if self.query.is_none() {
            self.query = Some(HashMap::new())
        }
        if let Some(ref mut q) = self.query {
            q.insert(query.0, query.1);
        }
        self
    }

    /// Add form for http body.
    pub fn form(mut self, form: (&'a str, Value)) -> Self {
        if self.form.is_none() {
            self.form = Some(HashMap::new())
        }
        if let Some(ref mut f) = self.form {
            f.insert(form.0, form.1);
        }
        self
    }

    /// Add http body.
    pub fn body(mut self, body: Body) ->Self {
        self.body = Some(body);
        self
    }

    /// Add captures
    pub fn capture(mut self, capture: Vec<Capture<'a>>) -> Self {
        self.capture = Some(capture);
        self
    }

    /// Whether enable debug info
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Specify an expect condition for task.
    pub fn expect(mut self, tup: (&'a str, Value)) -> Self {
        self.expect = Some(tup);
        self
    }

    pub(crate) fn filter_caps<T>(&self, f: T) -> Option<Vec<&Capture>> where T: FnMut(&&Capture) -> bool {
        if let Some(ref caps) = self.capture {
            let v = caps.iter().filter(f).collect::<Vec<&Capture>>();
            return Some(v);
        }
        None
    }

    /// Build an HttpTask use this builder.
    pub fn build(self) -> HttpTask<'a> {
        HttpTask { config: self, task_id: crate::faker::uuid_v4() }
    }

}

impl Default for HttpTaskBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}


