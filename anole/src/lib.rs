
pub mod faker;
pub mod value;
pub mod error;
pub mod store;
pub mod engine;
pub mod task;
pub mod context;
pub mod capture;

type Result<T> = std::result::Result<T, crate::error::Error>;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
