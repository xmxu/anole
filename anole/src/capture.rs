
/// 捕获参数
/// 
#[derive(Debug)]
pub enum Capture {
    Header(Cap),
    //for body
    Json(Cap),
    Xml(Cap),
}

#[derive(Debug)]
pub struct Cap {
    pub key: String,
}

pub fn header(key: String) -> Capture {
    Capture::Header(Cap {key})
}

pub fn json(key: String) -> Capture {
    Capture::Json(Cap {key})
}

pub fn xml(key: String) -> Capture {
    Capture::Xml(Cap {key})
}