export type ProjectStatus = 'active' | 'archived';

export interface ProjectSettings {
  default_model?: string;
  default_provider?: string;
  [key: string]: unknown;
}

export interface Project {
  id: string;
  name: string;
  description?: string;
  working_directory: string;
  status: ProjectStatus;
  settings: ProjectSettings;
  created_at: string;
  updated_at: string;
  archived_at?: string;
}

export interface CreateProjectInput {
  name: string;
  working_directory: string;
  description?: string;
  settings?: ProjectSettings;
}

export interface UpdateProjectInput {
  name?: string;
  description?: string;
  working_directory?: string;
  status?: ProjectStatus;
  settings?: ProjectSettings;
}

export type AgentStatus = 'active' | 'archived';

export interface ModelSettings {
  provider: string;
  model: string;
  temperature?: number;
  max_tokens?: number;
}

export interface McpServerReference {
  name: string;
  command: string;
  args: string[];
  env: Record<string, string>;
}

export interface McpConfig {
  servers: McpServerReference[];
}

export interface PermissionPolicyConfig {
  default_action: 'ask' | 'allow' | 'deny';
  allowed_tools: string[];
  denied_tools: string[];
}

export interface DelegationPolicy {
  allow_delegation: boolean;
  allowed_targets: string[];
}

export interface ApprovalPolicy {
  require_approval_for_write: boolean;
  require_approval_for_shell: boolean;
}

export interface AgentDefinition {
  id: string;
  project_id: string;
  name: string;
  role: string;
  description?: string;
  system_prompt?: string;
  model_settings: ModelSettings;
  enabled_tools: string[];
  mcp_config: McpConfig;
  permission_policy: PermissionPolicyConfig;
  delegation_policy: DelegationPolicy;
  approval_policy: ApprovalPolicy;
  status: AgentStatus;
  created_at: string;
  updated_at: string;
}

export interface CreateAgentInput {
  project_id: string;
  name: string;
  role: string;
  model_settings: ModelSettings;
  description?: string;
  system_prompt?: string;
  enabled_tools?: string[];
  mcp_config?: McpConfig;
  permission_policy?: PermissionPolicyConfig;
  delegation_policy?: DelegationPolicy;
  approval_policy?: ApprovalPolicy;
}

export interface UpdateAgentInput {
  name?: string;
  role?: string;
  description?: string;
  system_prompt?: string;
  model_settings?: ModelSettings;
  enabled_tools?: string[];
  mcp_config?: McpConfig;
  permission_policy?: PermissionPolicyConfig;
  delegation_policy?: DelegationPolicy;
  approval_policy?: ApprovalPolicy;
  status?: AgentStatus;
}
