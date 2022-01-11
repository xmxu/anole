

use std::{ops::Range, net::{Ipv4Addr, Ipv6Addr}};


use rand::Rng;
use uuid::Uuid;

/// 生成0-1范围内的随机数
pub fn random() -> f64 {
    rand::thread_rng().gen::<f64>()
}

/// 生成指定范围内的i32随机数
pub fn random_range(m: Range<i32>) -> i32 {
    rand::thread_rng().gen_range(m)
}

/// 生成0-255范围内的随机数
fn random_u8() -> u8 {
    rand::thread_rng().gen_range(0..=u8::MAX)
}

/// 生成0-65535范围内的随机数
fn random_u16() -> u16 {
    rand::thread_rng().gen_range(0..=u16::MAX)
}

/// 随机生成false或者true
pub fn random_bool()-> bool {
    rand::thread_rng().gen_bool(1.0 / 2.0)
}

/// 生成v4版本的uuid
pub fn uuid_v4() -> String {
    Uuid::new_v4().to_string()
}

/// uuid v5版本的命名空间类型
pub enum UuidNameSpace {
    Dns,
    Oid,
    Url,
    X500
}

/// 生成指定命名空间v5版本的uuid
pub fn uuid_v5(namespace: UuidNameSpace, name: &[u8]) -> String {
    let ns = match namespace {
        UuidNameSpace::Dns => &Uuid::NAMESPACE_DNS,
        UuidNameSpace::Oid => &Uuid::NAMESPACE_OID,
        UuidNameSpace::Url => &Uuid::NAMESPACE_URL,
        UuidNameSpace::X500 => &Uuid::NAMESPACE_X500
    };
    Uuid::new_v5(ns, name).to_string()
}

/// 随机生成15位的imei
pub fn imei() -> String {
    let mut numbers: Vec<u8> = vec![];
    let mut sum = 0;
    for i in 0..14 {
        let mut x: u8 = random_range(0..10) as u8;
        numbers.push(x);
        if i % 2 == 0 {
            x *= 2;
            if x > 9 {
                x -= 9;
            }
        } 
        sum += x;
    }
    numbers.push(sum % 10);
    numbers.iter().map(|x| x.to_string()).collect::<String>()
}

pub fn oaid() -> String {
    todo!()
}

/// 随机生成ipv4地址
pub fn ipv4() -> String {
    Ipv4Addr::new(random_u8(), random_u8(), random_u8(), random_u8()).to_string()
}

/// 随机生成ipv6地址
pub fn ipv6() -> String {
    Ipv6Addr::new(random_u16(), random_u16(), random_u16(), random_u16(), random_u16(), random_u16(), random_u16(), random_u16()).to_string()
}

/// 随机生成mac地址
pub fn mac_address() -> String {
    format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", random_u8(), random_u8(), random_u8(), random_u8(), random_u8(), random_u8())
}

/// 字符串源类型
pub enum StrSource {
    Digit,
    Alpha,
    AlphaAll,
    DigitAlpha,
}

const DIGIT_SOURCE: &str = "0123456789";
const ALPHA_SOURCE: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHA_ALL_SOURCE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGIT_ALPHA_SOURCE: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// 根据指定的源类型和长度，随机生成字符串
pub fn random_str(source: StrSource, len: usize) -> String {
    if len == 0 {
        return "".to_string();
    }
    let s = match source {
        StrSource::Digit => DIGIT_SOURCE,
        StrSource::Alpha => ALPHA_SOURCE,
        StrSource::AlphaAll => ALPHA_ALL_SOURCE,
        StrSource::DigitAlpha => DIGIT_ALPHA_SOURCE,
    };
    let max_len = s.len();
    let mut str = String::with_capacity(len);
    for _ in 0..len {
        let idx = rand::thread_rng().gen_range(0..max_len);
        str.push_str(&s[idx..idx+1]);
    }
    str
}