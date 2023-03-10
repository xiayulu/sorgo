#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Redis error")]
    Redis(#[from] redis::RedisError),

    #[error("MongoDB error")]
    MongoDB(#[from] mongodb::error::Error),

    #[error("Value error: {0}")]
    Value(String),

    #[error("Service error: {0}")]
    Service(String),

    #[error("Server error: {0}")]
    Server(String),

    #[error("Auth error")]
    Auth,
}

pub type Result<T> = std::result::Result<T, Error>;
