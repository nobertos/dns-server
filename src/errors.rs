type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn index_out_of_bound() -> Error {
    "Error: Index out of bound!".into()
}

pub fn jumps_limit(limit: u8) -> Error {
    format!("Error: Exceeded limit of {} jumps!", limit).into()
}

pub fn label_len_limit() -> Error {
    "Error: Single label exceeds 63 characters limit".into()
}

pub fn failed_json_parse<'a>() -> &'a str {
    "Failed to parse JSON string"
}

pub fn failed_path_read(path: &str) -> String {
    format!("Failed to read from {}", path)
}

pub fn failed_env_parse(value: &str) -> String {
    format!(
        "{} is not a supported environment. User either `local` or `production`.",
        value
    )
}

pub fn failed_config_read<'a>() -> &'a str {
    "Failed to read configuration."
}

pub fn failed_socket_bind<'a>() -> &'a str {
    "Failed to bind socket address to `UdpSocket`"
}

pub fn failed_cdn_down<'a>() -> &'a str {
    "The CDN is currently down, Try again later!"
}
