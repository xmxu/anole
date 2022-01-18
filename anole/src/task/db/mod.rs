pub mod mysql;

#[derive(Debug)]
pub struct DBClientOption<'a> {
    pub url: &'a str,
    pub max_connections: u32,

}

impl<'a> DBClientOption<'a> {
    
    pub fn builder() -> Self {
        DBClientOption { url: "", max_connections: 10 }
    }

    pub fn url(mut self, url: &'a str) -> Self {
        self.url = url;
        self
    }

    pub fn max_connections(mut self, connections: u32) -> Self {
        self.max_connections = connections;
        self
    }
}