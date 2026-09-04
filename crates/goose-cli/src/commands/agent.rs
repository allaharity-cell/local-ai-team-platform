use anyhow::Result;
use clap::Subcommand;
use platform_core::agent_definition::model::{
    AgentStatus, CreateAgentInput, ModelSettings, UpdateAgentInput,
};
use platform_core::agent_definition::AgentDefinitionService;
use platform_core::persistence::{
    AgentDefinitionRepository, PlatformDatabase, ProjectRepository,
};

#[derive(Subcommand, Debug, Clone)]
pub enum AgentCommand {
    /// Create a new agent definition scoped to a project
    Create {
        /// Parent project ID
        #[arg(short, long)]
        project_id: String,

        /// Agent display name
        #[arg(short, long)]
        name: String,

        /// Agent functional role (e.g., 'Lead Architect', 'Researcher')
        #[arg(short, long)]
        role: String,

        /// LLM Provider (e.g., 'anthropic', 'openai', 'google', 'ollama')
        #[arg(long)]
        provider: String,

        /// LLM Model ID (e.g., 'claude-3-7-sonnet', 'gpt-4o', 'llama3.3:70b')
        #[arg(long)]
        model: String,

        /// Agent description
        #[arg(short, long)]
        description: Option<String>,

        /// System instructions / persona prompt
        #[arg(long)]
        system_instructions: Option<String>,

        /// Maximum turns per task
        #[arg(long)]
        max_turns: Option<u32>,

        /// Comma-separated list of enabled tool names
        #[arg(long, value_delimiter = ',')]
        tools: Option<Vec<String>>,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// List all agents scoped to a specific project
    List {
        /// Project ID to list agents for
        #[arg(short, long)]
        project_id: String,

        /// Include archived agents in output
        #[arg(long)]
        include_archived: bool,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show agent definition details
    Show {
        /// Agent ID
        id: String,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Update an existing agent definition
    Update {
        /// Agent ID to update
        id: String,

        /// New agent name
        #[arg(short, long)]
        name: Option<String>,

        /// New role
        #[arg(short, long)]
        role: Option<String>,

        /// New provider
        #[arg(long)]
        provider: Option<String>,

        /// New model
        #[arg(long)]
        model: Option<String>,

        /// New description
        #[arg(short, long)]
        description: Option<String>,

        /// New system instructions
        #[arg(long)]
        system_instructions: Option<String>,

        /// Set status (active, archived, disabled)
        #[arg(long)]
        status: Option<String>,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Archive an agent definition
    Archive {
        /// Agent ID to archive
        id: String,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Delete an agent definition permanently
    Delete {
        /// Agent ID to delete
        id: String,

        /// Skip confirmation
        #[arg(long)]
        force: bool,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },
}

async fn get_service() -> Result<AgentDefinitionService> {
    let db = PlatformDatabase::default_local().await?;
    let agent_repo = AgentDefinitionRepository::new(db.pool().clone());
    let project_repo = ProjectRepository::new(db.pool().clone());
    Ok(AgentDefinitionService::new(agent_repo, project_repo))
}

pub async fn handle_agent_command(command: AgentCommand) -> Result<()> {
    let service = get_service().await?;

    match command {
        AgentCommand::Create {
            project_id,
            name,
            role,
            provider,
            model,
            description,
            system_instructions,
            max_turns,
            tools,
            json,
        } => {
            let input = CreateAgentInput {
                project_id,
                name,
                role,
                provider,
                model,
                description,
                system_instructions,
                model_settings: Some(ModelSettings::default()),
                enabled_tools: tools,
                mcp_config: None,
                permission_policy: None,
                max_turns,
                delegation_policy: None,
                approval_policy: None,
            };

            let agent = service.create_agent(input).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&agent)?);
            } else {
                println!("Agent created successfully!");
                println!("  ID:          {}", agent.id);
                println!("  Project ID:  {}", agent.project_id);
                println!("  Name:        {}", agent.name);
                println!("  Role:        {}", agent.role);
                println!("  Provider:    {}", agent.provider);
                println!("  Model:       {}", agent.model);
                println!("  Status:      {}", agent.status);
            }
        }

        AgentCommand::List {
            project_id,
            include_archived,
            json,
        } => {
            let agents = service.list_agents_by_project(&project_id, include_archived).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&agents)?);
            } else if agents.is_empty() {
                println!("No agents found for project '{}'.", project_id);
            } else {
                println!("{:<36}  {:<20}  {:<20}  {:<12}  {:<15}  {}", "AGENT ID", "NAME", "ROLE", "PROVIDER", "MODEL", "STATUS");
                println!("{:-<36}  {:-<20}  {:-<20}  {:-<12}  {:-<15}  {:-<8}", "", "", "", "", "", "");
                for a in agents {
                    println!(
                        "{:<36}  {:<20}  {:<20}  {:<12}  {:<15}  {}",
                        a.id,
                        if a.name.len() > 20 { format!("{}...", &a.name[..17]) } else { a.name },
                        if a.role.len() > 20 { format!("{}...", &a.role[..17]) } else { a.role },
                        a.provider,
                        a.model,
                        a.status
                    );
                }
            }
        }

        AgentCommand::Show { id, json } => {
            let agent = service.get_agent(&id).await?
                .ok_or_else(|| anyhow::anyhow!("Agent not found: {}", id))?;

            if json {
                println!("{}", serde_json::to_string_pretty(&agent)?);
            } else {
                println!("Agent Details:");
                println!("  ID:          {}", agent.id);
                println!("  Project ID:  {}", agent.project_id);
                println!("  Name:        {}", agent.name);
                println!("  Role:        {}", agent.role);
                println!("  Provider:    {}", agent.provider);
                println!("  Model:       {}", agent.model);
                println!("  Status:      {}", agent.status);
                println!("  Created At:  {}", agent.created_at);
                println!("  Updated At:  {}", agent.updated_at);
                if let Some(desc) = &agent.description {
                    println!("  Description: {}", desc);
                }
                if !agent.system_instructions.is_empty() {
                    println!("  System Prompt: {} characters", agent.system_instructions.len());
                }
            }
        }

        AgentCommand::Update {
            id,
            name,
            role,
            provider,
            model,
            description,
            system_instructions,
            status,
            json,
        } => {
            let parsed_status = match status {
                Some(s) => Some(s.parse::<AgentStatus>().map_err(|e| anyhow::anyhow!(e))?),
                None => None,
            };

            let input = UpdateAgentInput {
                name,
                description: description.map(Some),
                role,
                system_instructions,
                provider,
                model,
                model_settings: None,
                enabled_tools: None,
                mcp_config: None,
                permission_policy: None,
                max_turns: None,
                delegation_policy: None,
                approval_policy: None,
                status: parsed_status,
            };

            let updated = service.update_agent(&id, input).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&updated)?);
            } else {
                println!("Agent updated successfully!");
                println!("  ID:       {}", updated.id);
                println!("  Name:     {}", updated.name);
                println!("  Role:     {}", updated.role);
                println!("  Provider: {}", updated.provider);
                println!("  Model:    {}", updated.model);
                println!("  Status:   {}", updated.status);
            }
        }

        AgentCommand::Archive { id, json } => {
            let archived = service.archive_agent(&id).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&archived)?);
            } else {
                println!("Agent '{}' ({}) archived successfully.", archived.name, archived.id);
            }
        }

        AgentCommand::Delete { id, force, json } => {
            if !force {
                eprintln!("Run with --force to confirm deletion of agent '{}'.", id);
                anyhow::bail!("Deletion aborted without --force flag");
            }

            let deleted = service.delete_agent(&id).await?;
            if json {
                println!("{{\"deleted\": {}, \"id\": \"{}\"}}", deleted, id);
            } else if deleted {
                println!("Agent '{}' deleted successfully.", id);
            } else {
                println!("Agent '{}' not found.", id);
            }
        }
    }

    Ok(())
}
