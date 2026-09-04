use sqlx::{Pool, Sqlite};

pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS platform_projects (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            working_directory TEXT NOT NULL,
            status TEXT NOT NULL,
            settings TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            archived_at TEXT
        );

        CREATE TABLE IF NOT EXISTS platform_agent_definitions (
            id TEXT PRIMARY KEY NOT NULL,
            project_id TEXT NOT NULL REFERENCES platform_projects(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            description TEXT,
            role TEXT NOT NULL,
            system_instructions TEXT NOT NULL,
            provider TEXT NOT NULL,
            model TEXT NOT NULL,
            model_settings TEXT NOT NULL,
            enabled_tools TEXT NOT NULL,
            mcp_config TEXT NOT NULL,
            permission_policy TEXT NOT NULL,
            max_turns INTEGER,
            delegation_policy TEXT NOT NULL,
            approval_policy TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_platform_agent_project_id
        ON platform_agent_definitions(project_id);
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
