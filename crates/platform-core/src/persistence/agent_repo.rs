use crate::agent_definition::model::{
    AgentDefinition, AgentStatus, ApprovalPolicy, DelegationPolicy, McpServerReference,
    ModelSettings, PermissionPolicyConfig,
};
use crate::error::{PlatformError, Result};
use chrono::{DateTime, Utc};
use sqlx::{Pool, Row, Sqlite};
use std::str::FromStr;

#[derive(Clone)]
pub struct AgentDefinitionRepository {
    pool: Pool<Sqlite>,
}

impl AgentDefinitionRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, agent: &AgentDefinition) -> Result<()> {
        let model_settings_json = serde_json::to_string(&agent.model_settings)?;
        let enabled_tools_json = serde_json::to_string(&agent.enabled_tools)?;
        let mcp_config_json = serde_json::to_string(&agent.mcp_config)?;
        let permission_policy_json = serde_json::to_string(&agent.permission_policy)?;
        let delegation_policy_json = serde_json::to_string(&agent.delegation_policy)?;
        let approval_policy_json = serde_json::to_string(&agent.approval_policy)?;
        let status_str = agent.status.to_string();
        let created_at_str = agent.created_at.to_rfc3339();
        let updated_at_str = agent.updated_at.to_rfc3339();
        let max_turns_i64 = agent.max_turns.map(|n| n as i64);

        sqlx::query(
            r#"
            INSERT INTO platform_agent_definitions (
                id, project_id, name, description, role, system_instructions,
                provider, model, model_settings, enabled_tools, mcp_config,
                permission_policy, max_turns, delegation_policy, approval_policy,
                status, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)
            "#,
        )
        .bind(&agent.id)
        .bind(&agent.project_id)
        .bind(&agent.name)
        .bind(&agent.description)
        .bind(&agent.role)
        .bind(&agent.system_instructions)
        .bind(&agent.provider)
        .bind(&agent.model)
        .bind(&model_settings_json)
        .bind(&enabled_tools_json)
        .bind(&mcp_config_json)
        .bind(&permission_policy_json)
        .bind(max_turns_i64)
        .bind(&delegation_policy_json)
        .bind(&approval_policy_json)
        .bind(&status_str)
        .bind(&created_at_str)
        .bind(&updated_at_str)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<AgentDefinition>> {
        let row = sqlx::query(
            r#"
            SELECT id, project_id, name, description, role, system_instructions,
                   provider, model, model_settings, enabled_tools, mcp_config,
                   permission_policy, max_turns, delegation_policy, approval_policy,
                   status, created_at, updated_at
            FROM platform_agent_definitions
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(self.row_to_agent(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn list_by_project(
        &self,
        project_id: &str,
        include_archived: bool,
    ) -> Result<Vec<AgentDefinition>> {
        let rows = if include_archived {
            sqlx::query(
                r#"
                SELECT id, project_id, name, description, role, system_instructions,
                       provider, model, model_settings, enabled_tools, mcp_config,
                       permission_policy, max_turns, delegation_policy, approval_policy,
                       status, created_at, updated_at
                FROM platform_agent_definitions
                WHERE project_id = ?1
                ORDER BY updated_at DESC
                "#,
            )
            .bind(project_id)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query(
                r#"
                SELECT id, project_id, name, description, role, system_instructions,
                       provider, model, model_settings, enabled_tools, mcp_config,
                       permission_policy, max_turns, delegation_policy, approval_policy,
                       status, created_at, updated_at
                FROM platform_agent_definitions
                WHERE project_id = ?1 AND status != 'archived'
                ORDER BY updated_at DESC
                "#,
            )
            .bind(project_id)
            .fetch_all(&self.pool)
            .await?
        };

        let mut agents = Vec::with_capacity(rows.len());
        for row in &rows {
            agents.push(self.row_to_agent(row)?);
        }

        Ok(agents)
    }

    pub async fn update(&self, agent: &AgentDefinition) -> Result<()> {
        let model_settings_json = serde_json::to_string(&agent.model_settings)?;
        let enabled_tools_json = serde_json::to_string(&agent.enabled_tools)?;
        let mcp_config_json = serde_json::to_string(&agent.mcp_config)?;
        let permission_policy_json = serde_json::to_string(&agent.permission_policy)?;
        let delegation_policy_json = serde_json::to_string(&agent.delegation_policy)?;
        let approval_policy_json = serde_json::to_string(&agent.approval_policy)?;
        let status_str = agent.status.to_string();
        let updated_at_str = agent.updated_at.to_rfc3339();
        let max_turns_i64 = agent.max_turns.map(|n| n as i64);

        let result = sqlx::query(
            r#"
            UPDATE platform_agent_definitions
            SET name = ?1,
                description = ?2,
                role = ?3,
                system_instructions = ?4,
                provider = ?5,
                model = ?6,
                model_settings = ?7,
                enabled_tools = ?8,
                mcp_config = ?9,
                permission_policy = ?10,
                max_turns = ?11,
                delegation_policy = ?12,
                approval_policy = ?13,
                status = ?14,
                updated_at = ?15
            WHERE id = ?16
            "#,
        )
        .bind(&agent.name)
        .bind(&agent.description)
        .bind(&agent.role)
        .bind(&agent.system_instructions)
        .bind(&agent.provider)
        .bind(&agent.model)
        .bind(&model_settings_json)
        .bind(&enabled_tools_json)
        .bind(&mcp_config_json)
        .bind(&permission_policy_json)
        .bind(max_turns_i64)
        .bind(&delegation_policy_json)
        .bind(&approval_policy_json)
        .bind(&status_str)
        .bind(&updated_at_str)
        .bind(&agent.id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(PlatformError::AgentNotFound(agent.id.clone()));
        }

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM platform_agent_definitions WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    fn row_to_agent(&self, row: &sqlx::sqlite::SqliteRow) -> Result<AgentDefinition> {
        let id: String = row.get("id");
        let project_id: String = row.get("project_id");
        let name: String = row.get("name");
        let description: Option<String> = row.get("description");
        let role: String = row.get("role");
        let system_instructions: String = row.get("system_instructions");
        let provider: String = row.get("provider");
        let model: String = row.get("model");
        let model_settings_json: String = row.get("model_settings");
        let enabled_tools_json: String = row.get("enabled_tools");
        let mcp_config_json: String = row.get("mcp_config");
        let permission_policy_json: String = row.get("permission_policy");
        let max_turns_i64: Option<i64> = row.get("max_turns");
        let delegation_policy_json: String = row.get("delegation_policy");
        let approval_policy_json: String = row.get("approval_policy");
        let status_str: String = row.get("status");
        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");

        let status = AgentStatus::from_str(&status_str)
            .unwrap_or(AgentStatus::Active);

        let model_settings: ModelSettings = serde_json::from_str(&model_settings_json)
            .unwrap_or_default();

        let enabled_tools: Vec<String> = serde_json::from_str(&enabled_tools_json)
            .unwrap_or_default();

        let mcp_config: Vec<McpServerReference> = serde_json::from_str(&mcp_config_json)
            .unwrap_or_default();

        let permission_policy: PermissionPolicyConfig = serde_json::from_str(&permission_policy_json)
            .unwrap_or_default();

        let delegation_policy: DelegationPolicy = serde_json::from_str(&delegation_policy_json)
            .unwrap_or_default();

        let approval_policy: ApprovalPolicy = serde_json::from_str(&approval_policy_json)
            .unwrap_or_default();

        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(AgentDefinition {
            id,
            project_id,
            name,
            description,
            role,
            system_instructions,
            provider,
            model,
            model_settings,
            enabled_tools,
            mcp_config,
            permission_policy,
            max_turns: max_turns_i64.map(|n| n as u32),
            delegation_policy,
            approval_policy,
            status,
            created_at,
            updated_at,
        })
    }
}
