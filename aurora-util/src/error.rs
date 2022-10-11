use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuroraError {
    #[error("CONNECTION_REJECT: {0}")]
    ConnectionReject(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
