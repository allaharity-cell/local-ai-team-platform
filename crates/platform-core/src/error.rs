use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Project not found: {0}")]
    ProjectNotFound(String),

    #[error("Agent not found: {0}")]
    AgentNotFound(String),

    #[error("Invalid project data: {0}")]
    InvalidProject(String),

    #[error("Invalid agent definition: {0}")]
    InvalidAgent(String),

    #[error("Working directory error: {0}")]
    InvalidWorkingDirectory(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, PlatformError>;
