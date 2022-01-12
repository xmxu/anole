use std::{path, ops::Index};

use log::{warn, debug};
use quick_xml::{Reader, events::{Event, BytesStart}};

use crate::value::Value;


//xml deserializer base on quick-xml

pub struct De<'a> {
    pub buf: &'a str,
    pub paths: &'a str,
    attr_state: Option<AttrState>,
    arr_state: Option<ArrState>
}

impl De<'_> {
    
    pub fn get(b: &str, paths: &str) -> crate::Result<crate::value::Value> {
        let de = De {
            buf: b,
            attr_state: None,
            arr_state: None,
            paths
        };
        de.decode()
    } 

    fn decode(mut self) -> crate::Result<crate::value::Value> {
        let mut reader = Reader::from_str(self.buf);
        reader.trim_text(true);
        let paths: Vec<&str> = self.paths.split('.').collect();
        let mut idx: usize = 0;
        let mut buf = Vec::new();
        let mut capture = String::new();
        let mut found = false;
        self.is_attr(paths[idx].to_string());
        debug!("attr_state:{:?}", self.attr_state);
        self.is_arr(paths[idx].to_string());
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if idx == paths.len() {
                        break;
                    }
                    found = false;
                    if let Ok(tag) = String::from_utf8(e.name().to_vec()) {
                        if let Some(ref _attr_state) = self.attr_state {
                            if tag == _attr_state.tag {
                                match _attr_state.find(e) {
                                    Ok(_attr) => {
                                        capture = _attr;
                                        idx += 1;
                                        break;
                                    },
                                    Err(e) => return Err(e)
                                }
                            }
                        } else if let Some(ref mut _arr_state) = self.arr_state {
                            if tag == _arr_state.tag {
                                _arr_state.reduce();
                                found = true;
                                if _arr_state.captured() {
                                    idx += 1;
                                    if idx < paths.len() {
                                        self.is_attr(paths[idx].to_string());
                                        self.is_arr(paths[idx].to_string());
                                    }
                                }
                            }
                        } else if tag == paths[idx] {
                            debug!("found:{}", tag);
                            found = true;
                            idx += 1;
                            if idx < paths.len() {
                                self.is_attr(paths[idx].to_string());
                                self.is_arr(paths[idx].to_string());
                            }
                        }
                        
                    }
                },
                Ok(Event::End(_)) => {
                    
                },
                Ok(Event::Text(ref t)) => {
                    if found {
                        if let Ok(s) = t.unescape_and_decode(&reader) {
                            capture = s;
                        }
                    }
                    if idx == paths.len() {
                        break;
                    }

                },
                Ok(Event::Eof) => break,
                Err(e) => return Err(crate::error::decode(e.into())),
                _ => (),
            }
            buf.clear();
        }
       
        let mut arr_captured = true;
        if let Some(ref _arr_state) = self.arr_state {
            arr_captured = _arr_state.captured();
        }
        debug!("xml_capture:{:?}", capture);
        if idx < paths.len() - 1 || !arr_captured {
            return Err(crate::error::decode("Not Found".into()));
        }
        Ok(Value::Str(capture))
    }

    fn is_attr<'a>(&mut self, p: String) {
        if let Some(idx) = p.find('#') {
            debug!("is_attr:{:?}", idx);
            self.attr_state = Some(AttrState {
                tag: p[..idx].to_string(),
                key: p[idx+1..].to_string(),
            });
        } else {
            self.attr_state.take();
        }
        
        
    }

    fn is_arr<'a>(&mut self, p: String) {
        if let Some(idx) = p.find(':') {
            self.arr_state = Some(ArrState {
                tag: p[..idx].to_string(),
                idx: p[idx+1..].parse::<usize>().unwrap()
            });
        } else {
            self.arr_state.take();
        }
    
    }
}

#[derive(Debug, Clone)]
struct AttrState {
    tag: String,
    key: String,
}

impl AttrState {

    fn find(&self, e: &BytesStart) -> crate::Result<String> {
        if let Some(a) = e.attributes().find(|a| {
            match a {
                Ok(_attr) => {
                    if let Ok(k) = String::from_utf8(_attr.key.to_vec()) {
                        k == self.key
                    } else {
                        false
                    }
                },
                Err(e) => false,
            };
            false
        }) {
            match a {
                Ok(_a) => {
                    match String::from_utf8(_a.value.to_vec()) {
                        Ok(s) => return Ok(s),
                        Err(e) => return Err(crate::error::decode(e.into()))
                    };
                },
                Err(e) => return Err(crate::error::decode(e.into())) 
            }
        }
        Err(crate::error::decode("empty attribute".into()))
    }
}

#[derive(Debug, Clone)]
struct ArrState {
    tag: String,
    idx: usize,
}

impl ArrState {
    fn captured(&self) -> bool {
        self.idx == 0
    }

    fn reduce(&mut self) {
        self.idx -= 1;
    }

}