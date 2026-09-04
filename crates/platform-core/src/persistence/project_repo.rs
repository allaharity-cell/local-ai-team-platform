use crate::error::{PlatformError, Result};
use crate::project::model::{Project, ProjectSettings, ProjectStatus};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Row, Sqlite};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone)]
pub struct ProjectRepository {
    pool: Pool<Sqlite>,
}

impl ProjectRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, project: &Project) -> Result<()> {
        let settings_json = serde_json::to_string(&project.settings)?;
        let status_str = project.status.to_string();
        let working_dir_str = project.working_directory.to_string_lossy().to_string();
        let created_at_str = project.created_at.to_rfc3339();
        let updated_at_str = project.updated_at.to_rfc3339();
        let archived_at_str = project.archived_at.map(|dt| dt.to_rfc3339());

        sqlx::query(
            r#"
            INSERT INTO platform_projects (
                id, name, description, working_directory, status, settings, created_at, updated_at, archived_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
        )
        .bind(&project.id)
        .bind(&project.name)
        .bind(&project.description)
        .bind(&working_dir_str)
        .bind(&status_str)
        .bind(&settings_json)
        .bind(&created_at_str)
        .bind(&updated_at_str)
        .bind(&archived_at_str)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Project>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, working_directory, status, settings, created_at, updated_at, archived_at
            FROM platform_projects
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(self.row_to_project(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn list(&self, include_archived: bool) -> Result<Vec<Project>> {
        let rows = if include_archived {
            sqlx::query(
                r#"
                SELECT id, name, description, working_directory, status, settings, created_at, updated_at, archived_at
                FROM platform_projects
                ORDER BY updated_at DESC
                "#,
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT id, name, description, working_directory, status, settings, created_at, updated_at, archived_at
                FROM platform_projects
                WHERE status != 'archived'
                ORDER BY updated_at DESC
                "#,
            )
            .fetch_all(&self.pool)
            .await?
        };

        let mut projects = Vec::with_capacity(rows.len());
        for row in &rows {
            projects.push(self.row_to_project(row)?);
        }

        Ok(projects)
    }

    pub async fn update(&self, project: &Project) -> Result<()> {
        let settings_json = serde_json::to_string(&project.settings)?;
        let status_str = project.status.to_string();
        let working_dir_str = project.working_directory.to_string_lossy().to_string();
        let updated_at_str = project.updated_at.to_rfc3339();
        let archived_at_str = project.archived_at.map(|dt| dt.to_rfc3339());

        let result = sqlx::query(
            r#"
            UPDATE platform_projects
            SET name = ?1,
                description = ?2,
                working_directory = ?3,
                status = ?4,
                settings = ?5,
                updated_at = ?6,
                archived_at = ?7
            WHERE id = ?8
            "#,
        )
        .bind(&project.name)
        .bind(&project.description)
        .bind(&working_dir_str)
        .bind(&status_str)
        .bind(&settings_json)
        .bind(&updated_at_str)
        .bind(&archived_at_str)
        .bind(&project.id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(PlatformError::ProjectNotFound(project.id.clone()));
        }

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM platform_projects WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    fn row_to_project(&self, row: &sqlx::sqlite::SqliteRow) -> Result<Project> {
        let id: String = row.get("id");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        let working_dir_str: String = row.get("working_directory");
        let status_str: String = row.get("status");
        let settings_json: String = row.get("settings");
        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");
        let archived_at_str: Option<String> = row.get("archived_at");

        let status = ProjectStatus::from_str(&status_str)
            .unwrap_or(ProjectStatus::Active);

        let settings: ProjectSettings = serde_json::from_str(&settings_json)
            .unwrap_or_default();

        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let archived_at = archived_at_str.and_then(|s| {
            DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&Utc))
                .ok()
        });

        Ok(Project {
            id,
            name,
            description,
            working_directory: PathBuf::from(working_dir_str),
            status,
            settings,
            created_at,
            updated_at,
            archived_at,
        })
    }
}
