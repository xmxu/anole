use std::path;

use log::{warn, debug};
use quick_xml::{Reader, events::Event};

use crate::value::Value;


//xml deserializer base on quick-xml

pub struct De<'a> {
    pub buf: &'a str,
    pub paths: &'a str,
}

impl De<'_> {
    
    pub fn get(b: &str, paths: &str) -> crate::Result<crate::value::Value> {
        let de = De {
            buf: b,
            paths
        };
        de.decode()
    } 

    fn decode(self) -> crate::Result<crate::value::Value> {
        let mut reader = Reader::from_str(self.buf);
        reader.trim_text(true);
        let paths: Vec<&str> = self.paths.split('.').collect();
        let mut idx: usize = 0;
        let mut buf = Vec::new();
        let mut capture = String::new();
        let mut found = false;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if idx == paths.len() {
                        break;
                    }
                    if let Ok(tag) = String::from_utf8(e.name().to_vec()) {
                        let (attr, p) = self.is_attr(paths[idx]);
                        if attr {
                            if let Some(a) = e.attributes().find(|a| {
                                match a {
                                    Ok(_attr) => {
                                        if let Ok(k) = String::from_utf8(_attr.key.to_vec()) {
                                            k == p
                                        } else {
                                            false
                                        }
                                    },
                                    Err(e) => false,
                                };
                                false
                            }) {
                                
                            }
                            break;
                        } else if tag == p {
                            found = true;
                        }
                    }
                },
                Ok(Event::End(ref e)) => {
                    if let Ok(tag) = String::from_utf8(e.name().to_vec()) {
                        if tag == paths[idx] {
                            idx += 1;
                        }
                    }
                    found = false;
                },
                Ok(Event::Text(ref t)) => {
                    if found {
                        if let Ok(s) = t.unescape_and_decode(&reader) {
                            capture = s;
                        }
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) => return Err(crate::error::decode(e.into())),
                _ => (),
            }
            buf.clear();
            debug!("xml_capture:{:?}", capture);
        }
        Ok(Value::Str(capture))
    }

    fn is_attr<'a>(&self, p: &'a str) -> (bool, &'a str) {
        if let Some(stripped) = p.strip_prefix('#') {
            (true, stripped)
        } else {
            (false, p)
        }
    }
}