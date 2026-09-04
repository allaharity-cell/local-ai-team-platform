use crate::error::{PlatformError, Result};
use crate::persistence::ProjectRepository;
use crate::project::model::{CreateProjectInput, Project, ProjectStatus, UpdateProjectInput};
use crate::project::validation::{validate_and_normalize_working_dir, validate_project_name};
use chrono::Utc;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Clone)]
pub struct ProjectService {
    repo: ProjectRepository,
}

impl ProjectService {
    pub fn new(repo: ProjectRepository) -> Self {
        Self { repo }
    }

    pub async fn create_project(&self, input: CreateProjectInput) -> Result<Project> {
        validate_project_name(&input.name)?;
        let canonical_dir = validate_and_normalize_working_dir(&input.working_directory)?;

        let now = Utc::now();
        let project = Project {
            id: Uuid::new_v4().to_string(),
            name: input.name.trim().to_string(),
            description: input.description.map(|d| d.trim().to_string()).filter(|d| !d.is_empty()),
            working_directory: canonical_dir,
            status: ProjectStatus::Active,
            settings: input.settings.unwrap_or_default(),
            created_at: now,
            updated_at: now,
            archived_at: None,
        };

        self.repo.insert(&project).await?;
        Ok(project)
    }

    pub async fn get_project(&self, id: &str) -> Result<Option<Project>> {
        self.repo.get_by_id(id).await
    }

    pub async fn list_projects(&self, include_archived: bool) -> Result<Vec<Project>> {
        self.repo.list(include_archived).await
    }

    pub async fn update_project(&self, id: &str, input: UpdateProjectInput) -> Result<Project> {
        let mut project = self.repo.get_by_id(id).await?
            .ok_or_else(|| PlatformError::ProjectNotFound(id.to_string()))?;

        if let Some(new_name) = input.name {
            validate_project_name(&new_name)?;
            project.name = new_name.trim().to_string();
        }

        if let Some(desc_opt) = input.description {
            project.description = desc_opt.map(|d| d.trim().to_string()).filter(|d| !d.is_empty());
        }

        if let Some(new_dir) = input.working_directory {
            let canonical_dir = validate_and_normalize_working_dir(&new_dir)?;
            project.working_directory = canonical_dir;
        }

        if let Some(new_status) = input.status {
            if new_status == ProjectStatus::Archived && project.status != ProjectStatus::Archived {
                project.archived_at = Some(Utc::now());
            } else if new_status != ProjectStatus::Archived {
                project.archived_at = None;
            }
            project.status = new_status;
        }

        if let Some(new_settings) = input.settings {
            project.settings = new_settings;
        }

        project.updated_at = Utc::now();
        self.repo.update(&project).await?;

        Ok(project)
    }

    pub async fn archive_project(&self, id: &str) -> Result<Project> {
        self.update_project(
            id,
            UpdateProjectInput {
                status: Some(ProjectStatus::Archived),
                ..Default::default()
            },
        )
        .await
    }

    pub async fn delete_project(&self, id: &str) -> Result<bool> {
        self.repo.delete(id).await
    }

    pub fn validate_path(&self, path: &Path) -> Result<PathBuf> {
        validate_and_normalize_working_dir(path)
    }
}
