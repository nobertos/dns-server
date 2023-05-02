use serde::Deserialize;
use serde::Serialize;

use crate::errors::failed_json_parse;
use crate::errors::failed_path_read;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Connection {
    client: String,
    server: String,
}

// TODO: MOVE ConnectionList INTO
//       ConnectionMap SO USE A HashMap
pub struct ConnectionList {
    connections: Vec<Connection>,
}

impl ConnectionList {
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
    pub fn read_connections(path: &str) -> Self {
        let json_file = std::fs::read_to_string(path).expect(&failed_path_read(path));

        let connections =
            serde_json::from_str::<Vec<Connection>>(&json_file).expect(failed_json_parse());
        Self { connections }
    }
}

impl From<Vec<Connection>> for ConnectionList {
    fn from(value: Vec<Connection>) -> Self {
        Self { connections: value }
    }
}
