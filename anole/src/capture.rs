
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
    pub save_key: String,
}

pub fn header(key: String, save_key: String) -> Capture {
    Capture::Header(Cap {key, save_key})
}

pub fn json(key: String, save_key: String) -> Capture {
    Capture::Json(Cap {key, save_key})
}

pub fn xml(key: String, save_key: String) -> Capture {
    Capture::Xml(Cap {key, save_key})
}