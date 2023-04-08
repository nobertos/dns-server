use serde::Deserialize;
use serde::Serialize;

use crate::errors::failed_json_parse;
use crate::errors::failed_path_read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub client: String,
    pub server: String,
}

impl Connection {
    pub fn read_connections(path: &str) -> Vec<Self> {
        let json_file = std::fs::read_to_string(path).expect(&failed_path_read(path));

        let connections =
            serde_json::from_str::<Vec<Connection>>(&json_file).expect(failed_json_parse());
        connections
    }
}
