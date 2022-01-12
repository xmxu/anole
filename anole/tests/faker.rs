
#[cfg(test)]
mod faker {
    use anole::faker;
    
    #[test]
    fn test_uuid_v4() {
        println!("uuid_v4:{}",faker::uuid_v4());
    }
}