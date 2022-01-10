
use std::ops::Range;

use rand::Rng;
use uuid::Uuid;

pub fn random() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn random_range(m: Range<i32>) -> i32 {
    rand::thread_rng().gen_range(m)
}

pub fn random_bool()-> bool {
    rand::thread_rng().gen_bool(1.0 / 2.0)
}

pub fn uuid_v4() -> String {
    Uuid::new_v4().to_string()
}

pub fn uuid_v5(name: &[u8]) -> String {
    Uuid::new_v5(&Uuid::NAMESPACE_DNS, name).to_string()
}

pub fn imei() -> String {
    "".to_string()
}

pub fn oaid() -> String {
    todo!()
}

pub fn ipv4() -> String {
    todo!()
}

pub fn ipv6() -> String {
    todo!()
}

pub fn mac_address() -> String {
    todo!()
}











