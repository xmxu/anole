use sqlx::{Decode, Database, Type};

use crate::value::Value;

/// 捕获参数
/// 
#[derive(Debug)]
pub enum Capture<'a> {
    Header(Cap<'a>),
    //for body
    Json(Cap<'a>),
    Xml(Cap<'a>),
    Column(Cap<'a>),
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

    pub(crate) fn is_column(&self) -> bool {
        matches!(self, Self::Column(_))
    }
}

#[derive(Debug)]
pub enum CapValueType {
    Size,
    I32,
    U32,
    I64,
    U64,
    Bool,
    Str,
    Date,
    Time,
}

#[derive(Debug)]
pub struct Cap<'a> {
    pub key: &'a str,
    pub save_key: &'a str,
    pub data_type: Option<CapValueType>,
}

impl <'a> Cap<'a> {

    pub(crate) fn is_usize(&self) -> bool {
        matches!(self.data_type, Some(CapValueType::Size))
    }
    
    pub(crate) fn is_i32(&self) -> bool {
        matches!(self.data_type, Some(CapValueType::I32))
    }

    pub(crate) fn is_u32(&self) -> bool {
        matches!(self.data_type, Some(CapValueType::U32))
    }

    pub(crate) fn is_i64(&self) -> bool {
        matches!(self.data_type, Some(CapValueType::I64))
    }

    pub(crate) fn is_u64(&self) -> bool {
        matches!(self.data_type, Some(CapValueType::U64))
    }

    pub(crate) fn is_bool(&self) -> bool {
        matches!(self.data_type, Some(CapValueType::Bool))
    }

    pub(crate) fn is_str(&self) -> bool {
        matches!(self.data_type, Some(CapValueType::Str))
    }

    pub(crate) fn is_date(&self) -> bool {
        matches!(self.data_type, Some(CapValueType::Date))
    }

    pub(crate) fn is_time(&self) -> bool {
        matches!(self.data_type, Some(CapValueType::Time))
    }
}

pub fn header<'a>(key: &'a str, save_key: &'a str) -> Capture<'a> {
    Capture::Header(Cap {key, save_key, data_type: None})
}

pub fn json<'a>(key: &'a str, save_key: &'a str) -> Capture<'a> {
    Capture::Json(Cap {key, save_key, data_type: None})
}

pub fn xml<'a>(key: &'a str, save_key: &'a str) -> Capture<'a> {
    Capture::Xml(Cap {key, save_key, data_type: None})
}

pub fn column<'a>(key: &'a str, save_key: &'a str, t: CapValueType) -> Capture<'a> {
    Capture::Column(Cap {key, save_key, data_type: Some(t)})
}