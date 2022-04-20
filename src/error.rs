use tokio::task::JoinError;

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    #[error("retrieve error {0}")]
    RetrieveError(#[from] reqwest::Error),

    #[error("io error {0}")]
    IoError(#[from] std::io::Error),

    #[error("join error {0}")]
    JoinError(#[from] JoinError),
}