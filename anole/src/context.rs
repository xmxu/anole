use crate::store::Store;

#[derive(Debug)]
pub struct Context {
    pub store: Store,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            store: Store::new()
        }
    }
}
