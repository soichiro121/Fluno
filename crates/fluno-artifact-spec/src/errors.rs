use thiserror::Error;

pub type ArtifactSpecResult<T> = Result<T, ArtifactSpecError>;

#[derive(Debug, Error)]
pub enum ArtifactSpecError {
    #[error("invalid artifact output mode: {0}")]
    InvalidOutputMode(String),
    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),
}
