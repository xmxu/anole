
use core::num;
use std::{ops::{Range, RangeInclusive}, net::{Ipv4Addr, Ipv6Addr}};

use rand::Rng;
use uuid::Uuid;

pub fn random() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn random_range(m: Range<i32>) -> i32 {
    rand::thread_rng().gen_range(m)
}

fn random_u8() -> u8 {
    rand::thread_rng().gen_range(0..=u8::MAX)
}

fn random_u16() -> u16 {
    rand::thread_rng().gen_range(0..=u16::MAX)
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

pub fn ipv4() -> String {
    Ipv4Addr::new(random_u8(), random_u8(), random_u8(), random_u8()).to_string()
}

pub fn ipv6() -> String {
    Ipv6Addr::new(random_u16(), random_u16(), random_u16(), random_u16(), random_u16(), random_u16(), random_u16(), random_u16()).to_string()
}

pub fn mac_address() -> String {
    format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", random_u8(), random_u8(), random_u8(), random_u8(), random_u8(), random_u8())
}

