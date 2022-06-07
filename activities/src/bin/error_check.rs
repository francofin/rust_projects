use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("database error")]
    DatabaseError(#[from] SqlError),
    #[error("password expired")]
    PasswordExpired,
    #[error("User not found")]
    UserNotFound,
    #[error("wrong password")]
    WrongPassword,
    #[error("network COnnection Error")]
    NetworkError(#[from] std::io::Error),
}