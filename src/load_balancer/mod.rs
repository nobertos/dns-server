pub mod connection;

use self::connection::Connection;
pub struct LoadBalancer {
    pub connections: Vec<Connection>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        Self {
            connections: vec![],
        }
    }
    pub fn iter_servers<'a>(&'a self, client: &'a str) -> impl Iterator<Item = &'a str> {
        self.connections.iter().filter_map(move |connection| {
            if connection.client == client {
                return Some(connection.server.as_str());
            }
            None
        })
    }
}

impl From<Vec<Connection>> for LoadBalancer {
    fn from(value: Vec<Connection>) -> Self {
        Self { connections: value }
    }
}
