
use anole::faker;

fn main() {
    println!("uuid_v4:{:?}", faker::uuid_v4());
    println!("uuid_v5:{:?}", faker::uuid_v5("Anole".as_bytes()));
    println!("random_10-100:{:?}", faker::random_range(10..100));
    println!("random_bool:{:?}", faker::random_bool());
    println!("random_0-1:{:?}", faker::random());
    println!("ipv4:{:?}", faker::ipv4());
    println!("ipv6:{:?}", faker::ipv6());
    println!("mac:{:?}", faker::mac_address());
    println!("imei:{:?}", faker::imei());
}