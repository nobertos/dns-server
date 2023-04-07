type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn index_out_of_bound() -> Error {
    "Error: Index out of bound!".into()
}

pub fn jumps_limit(limit: u32) -> Error {
    format!("Error: Exceeded limit of {} jumps!", limit).into()
}
