use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Active,
    Archived,
    Disabled,
}

impl Default for AgentStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl std::fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Archived => write!(f, "archived"),
            Self::Disabled => write!(f, "disabled"),
        }
    }
}

impl std::str::FromStr for AgentStatus {
    type Err = String;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "active" => Ok(Self::Active),
            "archived" => Ok(Self::Archived),
            "disabled" => Ok(Self::Disabled),
            other => Err(format!("Unknown agent status: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelSettings {
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub reasoning_effort: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpServerReference {
    pub server_name: String,
    #[serde(default)]
    pub tools: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PermissionPolicyConfig {
    pub default_action: String,
    #[serde(default)]
    pub tool_rules: HashMap<String, String>,
}

impl Default for PermissionPolicyConfig {
    fn default() -> Self {
        Self {
            default_action: "prompt".to_string(),
            tool_rules: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DelegationPolicy {
    pub can_delegate: bool,
    #[serde(default)]
    pub allowed_roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApprovalPolicy {
    pub require_approval_for_destructive_tools: bool,
    #[serde(default)]
    pub auto_approved_tools: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentDefinition {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub role: String,
    pub system_instructions: String,
    pub provider: String,
    pub model: String,
    pub model_settings: ModelSettings,
    pub enabled_tools: Vec<String>,
    pub mcp_config: Vec<McpServerReference>,
    pub permission_policy: PermissionPolicyConfig,
    pub max_turns: Option<u32>,
    pub delegation_policy: DelegationPolicy,
    pub approval_policy: ApprovalPolicy,
    pub status: AgentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentInput {
    pub project_id: String,
    pub name: String,
    pub role: String,
    pub provider: String,
    pub model: String,
    pub description: Option<String>,
    #[serde(default)]
    pub system_instructions: Option<String>,
    #[serde(default)]
    pub model_settings: Option<ModelSettings>,
    #[serde(default)]
    pub enabled_tools: Option<Vec<String>>,
    #[serde(default)]
    pub mcp_config: Option<Vec<McpServerReference>>,
    #[serde(default)]
    pub permission_policy: Option<PermissionPolicyConfig>,
    pub max_turns: Option<u32>,
    #[serde(default)]
    pub delegation_policy: Option<DelegationPolicy>,
    #[serde(default)]
    pub approval_policy: Option<ApprovalPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateAgentInput {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub role: Option<String>,
    pub system_instructions: Option<String>,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub model_settings: Option<ModelSettings>,
    pub enabled_tools: Option<Vec<String>>,
    pub mcp_config: Option<Vec<McpServerReference>>,
    pub permission_policy: Option<PermissionPolicyConfig>,
    pub max_turns: Option<Option<u32>>,
    pub delegation_policy: Option<DelegationPolicy>,
    pub approval_policy: Option<ApprovalPolicy>,
    pub status: Option<AgentStatus>,
}
