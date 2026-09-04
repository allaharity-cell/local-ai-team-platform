use crate::error::{PlatformError, Result};
use std::path::{Path, PathBuf};

pub fn validate_and_normalize_working_dir(path: &Path) -> Result<PathBuf> {
    if path.as_os_str().is_empty() {
        return Err(PlatformError::InvalidWorkingDirectory(
            "Working directory cannot be empty".to_string(),
        ));
    }

    // Check for obvious illegal root paths on Windows / Unix
    let path_str = path.to_string_lossy();
    if path_str == "/" || path_str == "\\" || path_str == "C:\\" || path_str == "C:/" {
        return Err(PlatformError::InvalidWorkingDirectory(
            "System root cannot be used as a project working directory".to_string(),
        ));
    }

    // Reject paths targeting Windows system directories
    if path_str.to_ascii_uppercase().starts_with("C:\\WINDOWS")
        || path_str.to_ascii_uppercase().starts_with("C:/WINDOWS")
    {
        return Err(PlatformError::InvalidWorkingDirectory(
            "System directory cannot be used as a project working directory".to_string(),
        ));
    }

    // Resolve or normalize
    let normalized = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| PlatformError::InvalidWorkingDirectory(format!("Cannot resolve current dir: {e}")))?
            .join(path)
    };

    // Ensure directory exists or create it
    if !normalized.exists() {
        std::fs::create_dir_all(&normalized).map_err(|e| {
            PlatformError::InvalidWorkingDirectory(format!(
                "Failed to create working directory '{}': {e}",
                normalized.display()
            ))
        })?;
    } else if !normalized.is_dir() {
        return Err(PlatformError::InvalidWorkingDirectory(format!(
            "Path exists but is not a directory: '{}'",
            normalized.display()
        )));
    }

    let canonical = normalized.canonicalize().map_err(|e| {
        PlatformError::InvalidWorkingDirectory(format!(
            "Failed to canonicalize working directory '{}': {e}",
            normalized.display()
        ))
    })?;

    Ok(canonical)
}

pub fn validate_project_name(name: &str) -> Result<()> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(PlatformError::InvalidProject(
            "Project name cannot be empty".to_string(),
        ));
    }

    if trimmed.len() > 100 {
        return Err(PlatformError::InvalidProject(
            "Project name cannot exceed 100 characters".to_string(),
        ));
    }

    if trimmed.chars().any(|c| c.is_control()) {
        return Err(PlatformError::InvalidProject(
            "Project name cannot contain control characters".to_string(),
        ));
    }

    Ok(())
}
