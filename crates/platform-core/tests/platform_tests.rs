use platform_core::agent_definition::model::{
    AgentStatus, CreateAgentInput, DelegationPolicy, ModelSettings, UpdateAgentInput,
};
use platform_core::agent_definition::AgentDefinitionService;
use platform_core::error::PlatformError;
use platform_core::persistence::{
    AgentDefinitionRepository, PlatformDatabase, ProjectRepository,
};
use platform_core::project::model::{
    CreateProjectInput, ProjectSettings, ProjectStatus, UpdateProjectInput,
};
use platform_core::project::ProjectService;
use std::path::PathBuf;

async fn setup_test_services() -> (ProjectService, AgentDefinitionService, tempfile::TempDir) {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let db = PlatformDatabase::in_memory().await.expect("init db");
    let project_repo = ProjectRepository::new(db.pool().clone());
    let agent_repo = AgentDefinitionRepository::new(db.pool().clone());

    let project_service = ProjectService::new(project_repo.clone());
    let agent_service = AgentDefinitionService::new(agent_repo, project_repo);

    (project_service, agent_service, temp_dir)
}

#[tokio::test]
async fn test_project_crud() {
    let (project_service, _, temp_dir) = setup_test_services().await;
    let proj_dir = temp_dir.path().join("my-project");

    // 1. Create
    let input = CreateProjectInput {
        name: "Test Research Project".to_string(),
        working_directory: proj_dir.clone(),
        description: Some("Deep market analysis".to_string()),
        settings: Some(ProjectSettings {
            default_provider: Some("anthropic".to_string()),
            default_model: Some("claude-3-7-sonnet".to_string()),
            allowed_tools: vec!["web_search".to_string(), "pdf_reader".to_string()],
            custom_instructions: Some("Always cite sources".to_string()),
        }),
    };

    let created = project_service.create_project(input).await.expect("create project");
    assert_eq!(created.name, "Test Research Project");
    assert_eq!(created.status, ProjectStatus::Active);
    assert!(created.working_directory.exists());

    // 2. Get
    let fetched = project_service.get_project(&created.id).await.expect("get project");
    assert!(fetched.is_some());
    let fetched = fetched.unwrap();
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.settings.default_provider.as_deref(), Some("anthropic"));

    // 3. Update
    let updated = project_service
        .update_project(
            &created.id,
            UpdateProjectInput {
                name: Some("Renamed Research Project".to_string()),
                description: Some(Some("Updated description".to_string())),
                ..Default::default()
            },
        )
        .await
        .expect("update project");
    assert_eq!(updated.name, "Renamed Research Project");
    assert_eq!(updated.description.as_deref(), Some("Updated description"));

    // 4. Archive
    let archived = project_service.archive_project(&created.id).await.expect("archive project");
    assert_eq!(archived.status, ProjectStatus::Archived);
    assert!(archived.archived_at.is_some());

    // 5. List with and without archived
    let active_list = project_service.list_projects(false).await.expect("list active");
    assert!(active_list.is_empty());

    let all_list = project_service.list_projects(true).await.expect("list all");
    assert_eq!(all_list.len(), 1);

    // 6. Delete
    let deleted = project_service.delete_project(&created.id).await.expect("delete project");
    assert!(deleted);
    let after_delete = project_service.get_project(&created.id).await.expect("get deleted");
    assert!(after_delete.is_none());
}

#[tokio::test]
async fn test_invalid_working_directories() {
    let (project_service, _, _) = setup_test_services().await;

    // Empty path
    let res = project_service
        .create_project(CreateProjectInput {
            name: "Invalid Dir".to_string(),
            working_directory: PathBuf::from(""),
            description: None,
            settings: None,
        })
        .await;
    assert!(matches!(res, Err(PlatformError::InvalidWorkingDirectory(_))));

    // System root
    let res = project_service
        .create_project(CreateProjectInput {
            name: "Root Dir".to_string(),
            working_directory: PathBuf::from("C:\\"),
            description: None,
            settings: None,
        })
        .await;
    assert!(matches!(res, Err(PlatformError::InvalidWorkingDirectory(_))));

    // System Windows dir
    let res = project_service
        .create_project(CreateProjectInput {
            name: "Windows Dir".to_string(),
            working_directory: PathBuf::from("C:\\Windows"),
            description: None,
            settings: None,
        })
        .await;
    assert!(matches!(res, Err(PlatformError::InvalidWorkingDirectory(_))));
}

#[tokio::test]
async fn test_agent_crud_and_project_isolation() {
    let (project_service, agent_service, temp_dir) = setup_test_services().await;

    // Create Project A
    let proj_a = project_service
        .create_project(CreateProjectInput {
            name: "Project Alpha".to_string(),
            working_directory: temp_dir.path().join("alpha"),
            description: None,
            settings: None,
        })
        .await
        .expect("create proj alpha");

    // Create Project B
    let proj_b = project_service
        .create_project(CreateProjectInput {
            name: "Project Beta".to_string(),
            working_directory: temp_dir.path().join("beta"),
            description: None,
            settings: None,
        })
        .await
        .expect("create proj beta");

    // Create Agent in Project A
    let agent_a = agent_service
        .create_agent(CreateAgentInput {
            project_id: proj_a.id.clone(),
            name: "Alpha Researcher".to_string(),
            role: "Principal Investigator".to_string(),
            provider: "anthropic".to_string(),
            model: "claude-3-7-sonnet".to_string(),
            description: Some("Alpha investigator".to_string()),
            system_instructions: Some("Strict academic tone".to_string()),
            model_settings: Some(ModelSettings {
                temperature: Some(0.2),
                top_p: Some(0.9),
                max_tokens: Some(4096),
                reasoning_effort: Some("high".to_string()),
            }),
            enabled_tools: Some(vec!["fetch_paper".to_string()]),
            mcp_config: None,
            permission_policy: None,
            max_turns: Some(30),
            delegation_policy: Some(DelegationPolicy {
                can_delegate: true,
                allowed_roles: vec!["Analyst".to_string()],
            }),
            approval_policy: None,
        })
        .await
        .expect("create agent in alpha");

    // Create Agent in Project B
    let agent_b = agent_service
        .create_agent(CreateAgentInput {
            project_id: proj_b.id.clone(),
            name: "Beta Writer".to_string(),
            role: "Copywriter".to_string(),
            provider: "ollama".to_string(),
            model: "llama3.3:70b".to_string(),
            description: Some("Beta writer".to_string()),
            system_instructions: Some("Creative tone".to_string()),
            model_settings: None,
            enabled_tools: None,
            mcp_config: None,
            permission_policy: None,
            max_turns: Some(15),
            delegation_policy: None,
            approval_policy: None,
        })
        .await
        .expect("create agent in beta");

    // CRITICAL: Project isolation check
    let alpha_agents = agent_service
        .list_agents_by_project(&proj_a.id, true)
        .await
        .expect("list alpha agents");
    assert_eq!(alpha_agents.len(), 1);
    assert_eq!(alpha_agents[0].id, agent_a.id);
    assert_eq!(alpha_agents[0].name, "Alpha Researcher");

    let beta_agents = agent_service
        .list_agents_by_project(&proj_b.id, true)
        .await
        .expect("list beta agents");
    assert_eq!(beta_agents.len(), 1);
    assert_eq!(beta_agents[0].id, agent_b.id);
    assert_eq!(beta_agents[0].name, "Beta Writer");

    // Ensure Agent A is NEVER returned in Project B list
    assert!(!alpha_agents.iter().any(|a| a.id == agent_b.id));
    assert!(!beta_agents.iter().any(|a| a.id == agent_a.id));

    // Update Agent A
    let updated_agent = agent_service
        .update_agent(
            &agent_a.id,
            UpdateAgentInput {
                name: Some("Lead Alpha Researcher".to_string()),
                model: Some("claude-3-7-opus".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("update agent");
    assert_eq!(updated_agent.name, "Lead Alpha Researcher");
    assert_eq!(updated_agent.model, "claude-3-7-opus");

    // Archive Agent A
    let archived_agent = agent_service.archive_agent(&agent_a.id).await.expect("archive agent");
    assert_eq!(archived_agent.status, AgentStatus::Archived);

    // List without archived
    let active_alpha = agent_service.list_agents_by_project(&proj_a.id, false).await.expect("list active");
    assert!(active_alpha.is_empty());

    // Delete Agent A
    let deleted = agent_service.delete_agent(&agent_a.id).await.expect("delete agent");
    assert!(deleted);
    let after_delete = agent_service.get_agent(&agent_a.id).await.expect("get agent");
    assert!(after_delete.is_none());
}

#[tokio::test]
async fn test_cannot_create_agent_for_nonexistent_project() {
    let (_, agent_service, _) = setup_test_services().await;

    let res = agent_service
        .create_agent(CreateAgentInput {
            project_id: "non-existent-uuid".to_string(),
            name: "Orphan Agent".to_string(),
            role: "Worker".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4o".to_string(),
            description: None,
            system_instructions: None,
            model_settings: None,
            enabled_tools: None,
            mcp_config: None,
            permission_policy: None,
            max_turns: None,
            delegation_policy: None,
            approval_policy: None,
        })
        .await;

    assert!(matches!(res, Err(PlatformError::ProjectNotFound(_))));
}

#[tokio::test]
async fn test_file_database_persistence_across_reconnect() {
    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let db_path = temp_dir.path().join("test_platform.db");
    let proj_dir = temp_dir.path().join("persisted_proj");

    let created_proj_id = {
        let db = PlatformDatabase::open(&db_path).await.expect("open db");
        let project_repo = ProjectRepository::new(db.pool().clone());
        let project_service = ProjectService::new(project_repo);

        let proj = project_service
            .create_project(CreateProjectInput {
                name: "Persistent Project".to_string(),
                working_directory: proj_dir.clone(),
                description: Some("Saved to disk".to_string()),
                settings: None,
            })
            .await
            .expect("create project");

        proj.id
    };

    // Reopen database from disk
    {
        let db = PlatformDatabase::open(&db_path).await.expect("reopen db");
        let project_repo = ProjectRepository::new(db.pool().clone());
        let project_service = ProjectService::new(project_repo);

        let fetched = project_service
            .get_project(&created_proj_id)
            .await
            .expect("fetch after reopen");

        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched.name, "Persistent Project");
        assert_eq!(fetched.description.as_deref(), Some("Saved to disk"));
    }
}
