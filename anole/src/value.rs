use serde::Serialize;

use crate::error;


#[derive(Debug)]
pub enum Value {
    I32(i32),
    U32(u32),
    F64(f64),
    Bool(bool),
    Str(String),
}

impl Value {
    pub fn as_i32(self) -> crate::Result<i32> {
        match self {
            Self::I32(v) => Ok(v),
            Self::U32(v) => Ok(v as i32),
            Self::F64(v) => Ok(v as i32),
            Self::Bool(v) => Ok(v as i32),
            Self::Str(v) => match v.parse::<i32>() {
                Ok(i) => Ok(i),
                Err(e) => Err(error::parse_value(e.into()))
            },
        }
    }

    pub fn as_u32(self) -> crate::Result<u32> {
        match self {
            Self::I32(v) => Ok(v as u32),
            Self::U32(v) => Ok(v),
            Self::F64(v) => Ok(v as u32),
            Self::Bool(v) => Ok(v as u32),
            Self::Str(v) => match v.parse::<u32>() {
                Ok(u) => Ok(u),
                Err(e) => Err(error::parse_value(e.into()))
            },
        }
    }

    pub fn as_f(self) -> crate::Result<f64> {
        match self {
            Self::I32(v) => Ok(v as f64),
            Self::U32(v) => Ok(v as f64),
            Self::F64(v) => Ok(v),
            Self::Bool(v) => {
                if v {
                    return Ok(1.0);
                }
                Ok(0.0)
            },
            Self::Str(v) => match v.parse::<f64>() {
                Ok(f) => Ok(f),
                Err(e) => Err(error::parse_value(e.into()))
            }
        }
    }

    pub fn as_bool(self) -> crate::Result<bool> {
        match self {
            Self::I32(i) => Ok(i > 0),
            Self::U32(u) => Ok(u > 0),
            Self::F64(f) => Ok(f > 0.0),
            Self::Bool(b) => Ok(b),
            Self::Str(s) => match s.parse::<bool>() {
                Ok(b) => Ok(b),
                Err(e) => Err(error::parse_value(e.into()))
            },
        }
    }

    pub fn as_str(self) -> String {
        match self {
            Self::Str(s) => s,
            Self::I32(i) => i.to_string(),
            Self::U32(u) => u.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::F64(f) => f.to_string(),
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

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::I32(v) => serializer.serialize_i32(*v),
            Value::U32(v) => serializer.serialize_u32(*v),
            Value::F64(f) => serializer.serialize_f64(*f),
            Value::Str(s) => serializer.serialize_str(s),
        }
    }
}