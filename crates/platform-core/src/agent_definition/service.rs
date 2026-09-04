use crate::agent_definition::model::{
    AgentDefinition, AgentStatus, ApprovalPolicy, CreateAgentInput, DelegationPolicy,
    McpServerReference, ModelSettings, PermissionPolicyConfig, UpdateAgentInput,
};
use crate::error::{PlatformError, Result};
use crate::persistence::{AgentDefinitionRepository, ProjectRepository};
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AgentDefinitionService {
    repo: AgentDefinitionRepository,
    project_repo: ProjectRepository,
}

impl AgentDefinitionService {
    pub fn new(repo: AgentDefinitionRepository, project_repo: ProjectRepository) -> Self {
        Self { repo, project_repo }
    }

    pub async fn create_agent(&self, input: CreateAgentInput) -> Result<AgentDefinition> {
        // Ensure parent project exists
        let _project = self.project_repo.get_by_id(&input.project_id).await?
            .ok_or_else(|| PlatformError::ProjectNotFound(input.project_id.clone()))?;

        let name = input.name.trim().to_string();
        if name.is_empty() {
            return Err(PlatformError::InvalidAgent("Agent name cannot be empty".to_string()));
        }

        let role = input.role.trim().to_string();
        if role.is_empty() {
            return Err(PlatformError::InvalidAgent("Agent role cannot be empty".to_string()));
        }

        let provider = input.provider.trim().to_string();
        if provider.is_empty() {
            return Err(PlatformError::InvalidAgent("Agent provider cannot be empty".to_string()));
        }

        let model = input.model.trim().to_string();
        if model.is_empty() {
            return Err(PlatformError::InvalidAgent("Agent model cannot be empty".to_string()));
        }

        let now = Utc::now();
        let agent = AgentDefinition {
            id: Uuid::new_v4().to_string(),
            project_id: input.project_id,
            name,
            description: input.description.map(|d| d.trim().to_string()).filter(|d| !d.is_empty()),
            role,
            system_instructions: input.system_instructions.unwrap_or_default(),
            provider,
            model,
            model_settings: input.model_settings.unwrap_or_default(),
            enabled_tools: input.enabled_tools.unwrap_or_default(),
            mcp_config: input.mcp_config.unwrap_or_default(),
            permission_policy: input.permission_policy.unwrap_or_default(),
            max_turns: input.max_turns,
            delegation_policy: input.delegation_policy.unwrap_or_default(),
            approval_policy: input.approval_policy.unwrap_or_default(),
            status: AgentStatus::Active,
            created_at: now,
            updated_at: now,
        };

        self.repo.insert(&agent).await?;
        Ok(agent)
    }

    pub async fn get_agent(&self, id: &str) -> Result<Option<AgentDefinition>> {
        self.repo.get_by_id(id).await
    }

    pub async fn list_agents_by_project(
        &self,
        project_id: &str,
        include_archived: bool,
    ) -> Result<Vec<AgentDefinition>> {
        self.repo.list_by_project(project_id, include_archived).await
    }

    pub async fn update_agent(&self, id: &str, input: UpdateAgentInput) -> Result<AgentDefinition> {
        let mut agent = self.repo.get_by_id(id).await?
            .ok_or_else(|| PlatformError::AgentNotFound(id.to_string()))?;

        if let Some(new_name) = input.name {
            let trimmed = new_name.trim().to_string();
            if trimmed.is_empty() {
                return Err(PlatformError::InvalidAgent("Agent name cannot be empty".to_string()));
            }
            agent.name = trimmed;
        }

        if let Some(desc_opt) = input.description {
            agent.description = desc_opt.map(|d| d.trim().to_string()).filter(|d| !d.is_empty());
        }

        if let Some(new_role) = input.role {
            let trimmed = new_role.trim().to_string();
            if trimmed.is_empty() {
                return Err(PlatformError::InvalidAgent("Agent role cannot be empty".to_string()));
            }
            agent.role = trimmed;
        }

        if let Some(new_instructions) = input.system_instructions {
            agent.system_instructions = new_instructions;
        }

        if let Some(new_provider) = input.provider {
            let trimmed = new_provider.trim().to_string();
            if trimmed.is_empty() {
                return Err(PlatformError::InvalidAgent("Agent provider cannot be empty".to_string()));
            }
            agent.provider = trimmed;
        }

        if let Some(new_model) = input.model {
            let trimmed = new_model.trim().to_string();
            if trimmed.is_empty() {
                return Err(PlatformError::InvalidAgent("Agent model cannot be empty".to_string()));
            }
            agent.model = trimmed;
        }

        if let Some(new_settings) = input.model_settings {
            agent.model_settings = new_settings;
        }

        if let Some(new_tools) = input.enabled_tools {
            agent.enabled_tools = new_tools;
        }

        if let Some(new_mcp) = input.mcp_config {
            agent.mcp_config = new_mcp;
        }

        if let Some(new_policy) = input.permission_policy {
            agent.permission_policy = new_policy;
        }

        if let Some(max_turns_opt) = input.max_turns {
            agent.max_turns = max_turns_opt;
        }

        if let Some(new_delegation) = input.delegation_policy {
            agent.delegation_policy = new_delegation;
        }

        if let Some(new_approval) = input.approval_policy {
            agent.approval_policy = new_approval;
        }

        if let Some(new_status) = input.status {
            agent.status = new_status;
        }

        agent.updated_at = Utc::now();
        self.repo.update(&agent).await?;

        Ok(agent)
    }

    pub async fn archive_agent(&self, id: &str) -> Result<AgentDefinition> {
        self.update_agent(
            id,
            UpdateAgentInput {
                status: Some(AgentStatus::Archived),
                ..Default::default()
            },
        )
        .await
    }

    pub async fn delete_agent(&self, id: &str) -> Result<bool> {
        self.repo.delete(id).await
    }
}
