
use serde::Serialize;
use sqlx::types::time::{self, Date, Time};

use crate::{context::Context, error};


/// Represents any capture support value.
#[derive(Debug, Clone)]
#[derive(std::cmp::PartialEq)]
pub enum Value {
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
    Str(String),
    Date(time::Date),
    Time(time::Time),
}

impl Value {
    /// If Value can parse to i32, returns an i32. Returns Err otherwise.
    pub fn as_i32(&self) -> crate::Result<i32> {
        match &self {
            Self::I32(v) => Ok(*v),
            Self::U32(v) => Ok(*v as i32),
            Self::F64(v) => Ok(*v as i32),
            Self::Bool(v) => Ok(*v as i32),
            Self::Str(v) => match v.parse::<i32>() {
                Ok(i) => Ok(i),
                Err(e) => Err(error::parse_value(e.into())),
            },
            _ => Err(error::unimplement("can not convert to i32"))
        }
    }

    /// If Value can parse to u32, returns an u32. Returns Err otherwise.
    pub fn as_u32(&self) -> crate::Result<u32> {
        match &self {
            Self::I32(v) => Ok(*v as u32),
            Self::U32(v) => Ok(*v),
            Self::F64(v) => Ok(*v as u32),
            Self::Bool(v) => Ok(*v as u32),
            Self::Str(v) => match v.parse::<u32>() {
                Ok(u) => Ok(u),
                Err(e) => Err(error::parse_value(e.into())),
            },
            _ => Err(error::unimplement("can not convert to u32"))
        }
    }

    /// If Value can parse to i64, returns an i64. Returns Err otherwise.
    pub fn as_i64(&self) -> crate::Result<i64> {
        match &self {
            Self::I32(v) => Ok(*v as i64),
            Self::U32(v) => Ok(*v as i64),
            Self::I64(v) => Ok(*v),
            Self::U64(v) => Ok(*v as i64),
            Self::F64(v) => Ok(*v as i64),
            Self::Bool(v) => Ok(*v as i64),
            Self::Str(v) => match v.parse::<i64>() {
                Ok(i) => Ok(i),
                Err(e) => Err(error::parse_value(e.into())),
            },
            _ => Err(error::unimplement("can not convert to i64"))
        }
    }

    /// If Value can parse to u64, returns an u64. Returns Err otherwise.
    pub fn as_u64(&self) -> crate::Result<u64> {
        match &self {
            Self::I32(v) => Ok(*v as u64),
            Self::U32(v) => Ok(*v as u64),
            Self::I64(v) => Ok(*v as u64),
            Self::U64(v) => Ok(*v),
            Self::F64(v) => Ok(*v as u64),
            Self::Bool(v) => Ok(*v as u64),
            Self::Str(v) => match v.parse::<u64>() {
                Ok(u) => Ok(u),
                Err(e) => Err(error::parse_value(e.into())),
            },
            _ => Err(error::unimplement("can not convert to u64"))
        }
    }

    /// If Value can parse to f64, returns an f64. Returns Err otherwise.
    pub fn as_f(&self) -> crate::Result<f64> {
        match &self {
            Self::I32(v) => Ok(*v as f64),
            Self::U32(v) => Ok(*v as f64),
            Self::F64(v) => Ok(*v),
            Self::I64(v) => Ok(*v as f64),
            Self::U64(v) => Ok(*v as f64),
            Self::Bool(v) => {
                if *v {
                    return Ok(1.0);
                }
                Ok(0.0)
            }
            Self::Str(v) => match v.parse::<f64>() {
                Ok(f) => Ok(f),
                Err(e) => Err(error::parse_value(e.into())),
            },
            _ => Err(error::unimplement("can not convert to f64"))
        }
    }

    /// If Value can parse to bool, returns an bool. Returns Err otherwise.
    pub fn as_bool(&self) -> crate::Result<bool> {
        match &self {
            Self::I32(i) => Ok(*i > 0),
            Self::U32(u) => Ok(*u > 0),
            Self::I64(u) => Ok(*u > 0),
            Self::U64(u) => Ok(*u > 0),
            Self::F64(f) => Ok(*f > 0.0),
            Self::Bool(b) => Ok(*b),
            Self::Str(s) => match s.parse::<bool>() {
                Ok(b) => Ok(b),
                Err(e) => Err(error::parse_value(e.into())),
            },
            _ => Err(error::unimplement("Date or Time can not convert to bool"))
        }
    }

    /// Returns Value parse to String.
    pub fn as_str(&self) -> String {
        match &self {
            Self::Str(s) => s.to_string(),
            Self::I32(i) => i.to_string(),
            Self::U32(u) => u.to_string(),
            Self::I64(i) => i.to_string(),
            Self::U64(u) => u.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::F64(f) => f.to_string(),
            Self::Date(d) => d.to_string(),
            Self::Time(t) => t.to_string(),
        }
    }

    /// If Value can parse to Date, returns an Date. Returns Err otherwise.
    pub fn as_date(&self) -> crate::Result<Date> {
        match &self {
            Self::Date(d) => Ok(*d),
            _ => Err(error::unimplement("can not convert to Date"))
        }
    }

    /// If Value can parse to Time, returns an Time. Returns Err otherwise.
    pub fn as_time(&self) -> crate::Result<Time> {
        match &self {
            Self::Time(t) => Ok(*t),
            _ => Err(error::unimplement("can not convert to Time"))
        }
    }

    /// If Value is wildcard, returns real String. Returns None otherwise.
    pub fn as_wildcard(&self) -> Option<String> {
        match self {
            Self::Str(s) => {
                if s.starts_with(':') {
                    return Some(s.as_str()[1..s.len()].to_string());
                }
                None
            }
            _ => None,
        }
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::I32(i)
    }
}

impl From<u32> for Value {
    fn from(u: u32) -> Self {
        Value::U32(u)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::F64(f as f64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::F64(f)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Str(s)
    }
}

impl From<&serde_json::Value> for Value {
    fn from(val: &serde_json::Value) -> Self {
        if val.is_boolean() {
            Value::Bool(val.as_bool().unwrap())
        } else if val.is_i64() {
            Value::I32(val.as_i64().unwrap() as i32)
        } else if val.is_f64() {
            Value::F64(val.as_f64().unwrap())
        } else if let Some(vv) = val.as_str() {
            Value::Str(vv.to_string())
        } else {
            Value::Str(format!("{}", val))
        }
    }
}

pub fn parse_json_value(value: &serde_json::Value, key: String) -> Option<serde_json::Value> {
    let keys = key.split('.');
    let mut cur_value = value;
    for k in keys {
        let (is_arr, k, idx) = try_as_array(k);
        cur_value = &cur_value[k];
        if is_arr {
            if cur_value.is_array() && cur_value.as_array().unwrap().len() > idx {
                cur_value = &cur_value[idx];
            } else {
                return None;
            }
        }
    }
    if cur_value != value {
        return Some(cur_value.to_owned());
    }
    None
}

fn try_as_array(k: &str) -> (bool, &str, usize) {
    if let Some(find_idx) = k.find('|') {
        if let Ok(idx) = k[find_idx+1..].parse::<usize>() {
            return (true, &k[..find_idx], idx);
        }
    }
    (false, k, 0)
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::I32(v) => serializer.serialize_i32(*v),
            Value::U32(v) => serializer.serialize_u32(*v),
            Value::I64(v) => serializer.serialize_i64(*v),
            Value::U64(v) => serializer.serialize_u64(*v),
            Value::F64(f) => serializer.serialize_f64(*f),
            Value::Str(s) => serializer.serialize_str(s),
            Value::Date(d) => serializer.serialize_str(&d.to_string()),
            Value::Time(t) => serializer.serialize_u16(t.millisecond()),
        }
    }
}

pub enum Body {
    File(String),
    Raw(bytes::Bytes),
    Replace(String, Vec<Value>),
}

impl Body {
    pub fn as_bytes(&self, ctx: &mut Context) -> Option<bytes::Bytes> {
        match self {
            Self::Raw(b) => Some(b.to_owned()),
            Self::File(path) => {
                if let Ok(file_content) = std::fs::read(path) {
                    return Some(bytes::Bytes::from(file_content));
                }
                None
            }
            Self::Replace(tmpl, values) => {
                let mut tmpl = tmpl.to_owned();
                for value in values {
                    if let Some(wildcard) = value.as_wildcard() {
                        if let Some(vv) = ctx.store.get(wildcard) {
                            tmpl = tmpl.replace(&value.as_str(), &vv.as_str());
                        }
                    }
                }
                Some(tmpl.into())
            }
        }
    }
}
