use reqwest::Client;

pub mod config;
pub mod health_check;

pub struct Request<'a> {
    addr: &'a str,
    port: u16,
    client: Client,
}

impl<'a> Request<'a> {
    fn new(port: u16) -> Self {
        Self {
            addr: "",
            client: Client::new(),
            port,
        }
    }

    fn socket_addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
    fn set_addr(&mut self, addr: &'a str) {
        self.addr = addr
    }
}
