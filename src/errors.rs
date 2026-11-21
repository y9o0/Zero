use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZeroErr {
    #[error("Illegal move happend at {} due to {}", at, why)]
    Illegal { at: String, why: String },
    #[error("Unexpected : {}", message)]
    Unexpected { message: String },
}

pub type ZeroResult<T> = std::result::Result<T, ZeroErr>;
