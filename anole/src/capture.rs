
/// 捕获参数
/// 
#[derive(Debug)]
pub enum Capture<'a> {
    Header(Cap<'a>),
    //for body
    Json(Cap<'a>),
    Xml(Cap<'a>),
}

impl <'a> Capture<'a> {
    pub(crate) fn is_header(&self) -> bool {
        matches!(self, Self::Header(_))
    }

    pub(crate) fn is_json(&self) -> bool {
        matches!(self, Self::Json(_))
    }

    pub(crate) fn is_xml(&self) -> bool {
        matches!(self, Self::Xml(_))
    }
}

#[derive(Debug)]
pub struct Cap<'a> {
    pub key: &'a str,
    pub save_key: &'a str,
}

pub fn header<'a>(key: &'a str, save_key: &'a str) -> Capture<'a> {
    Capture::Header(Cap {key, save_key})
}

pub fn json<'a>(key: &'a str, save_key: &'a str) -> Capture<'a> {
    Capture::Json(Cap {key, save_key})
}

pub fn xml<'a>(key: &'a str, save_key: &'a str) -> Capture<'a> {
    Capture::Xml(Cap {key, save_key})
}