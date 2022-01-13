use std::collections::HashMap;

use crate::value::Value;

/// 保存数据
#[derive(Debug)]
pub struct Store {
    pub(crate) data: HashMap<String, Value>,
}

impl <'a> Store {
    pub(crate) fn new() -> Store {
        Store { data: HashMap::new() }
    }

    pub fn get(&self, k: String) -> Option<&Value> {
        self.data.get(&k)
    }

    pub fn set(&mut self, k: String, v: Value) -> Option<Value> {
        self.data.insert(k, v)
    }

    pub fn delete(&mut self, k: String) -> Option<Value> {
        self.data.remove(&k)
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }
}