use anyhow::Result;
use clap::Subcommand;
use platform_core::persistence::{PlatformDatabase, ProjectRepository};
use platform_core::project::model::{CreateProjectInput, ProjectStatus, UpdateProjectInput};
use platform_core::project::ProjectService;
use std::path::PathBuf;

#[derive(Subcommand, Debug, Clone)]
pub enum ProjectCommand {
    /// Create a new project
    Create {
        /// Project name
        #[arg(short, long)]
        name: String,

        /// Project working directory path
        #[arg(short, long)]
        working_dir: PathBuf,

        /// Optional project description
        #[arg(short, long)]
        description: Option<String>,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// List all projects
    List {
        /// Include archived projects in output
        #[arg(long)]
        include_archived: bool,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Show project details
    Show {
        /// Project ID
        id: String,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Update an existing project
    Update {
        /// Project ID to update
        id: String,

        /// New project name
        #[arg(short, long)]
        name: Option<String>,

        /// New project description
        #[arg(short, long)]
        description: Option<String>,

        /// New project working directory
        #[arg(short, long)]
        working_dir: Option<PathBuf>,

        /// Set status (active, archived, suspended)
        #[arg(long)]
        status: Option<String>,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Archive a project
    Archive {
        /// Project ID to archive
        id: String,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },

    /// Delete a project permanently
    Delete {
        /// Project ID to delete
        id: String,

        /// Skip confirmation
        #[arg(long)]
        force: bool,

        /// Output result as JSON
        #[arg(long)]
        json: bool,
    },
}

async fn get_service() -> Result<ProjectService> {
    let db = PlatformDatabase::default_local().await?;
    let repo = ProjectRepository::new(db.pool().clone());
    Ok(ProjectService::new(repo))
}

pub async fn handle_project_command(command: ProjectCommand) -> Result<()> {
    let service = get_service().await?;

    match command {
        ProjectCommand::Create {
            name,
            working_dir,
            description,
            json,
        } => {
            let input = CreateProjectInput {
                name,
                working_directory: working_dir,
                description,
                settings: None,
            };
            let project = service.create_project(input).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&project)?);
            } else {
                println!("Project created successfully!");
                println!("  ID:          {}", project.id);
                println!("  Name:        {}", project.name);
                println!("  Status:      {}", project.status);
                println!("  Working Dir: {}", project.working_directory.display());
                if let Some(desc) = &project.description {
                    println!("  Description: {}", desc);
                }
            }
        }

        ProjectCommand::List {
            include_archived,
            json,
        } => {
            let projects = service.list_projects(include_archived).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&projects)?);
            } else if projects.is_empty() {
                println!("No projects found.");
            } else {
                println!("{:<36}  {:<25}  {:<10}  {}", "PROJECT ID", "NAME", "STATUS", "WORKING DIRECTORY");
                println!("{:-<36}  {:-<25}  {:-<10}  {:-<30}", "", "", "", "");
                for p in projects {
                    println!(
                        "{:<36}  {:<25}  {:<10}  {}",
                        p.id,
                        if p.name.len() > 25 { format!("{}...", &p.name[..22]) } else { p.name },
                        p.status,
                        p.working_directory.display()
                    );
                }
            }
        }

        ProjectCommand::Show { id, json } => {
            let project = service.get_project(&id).await?
                .ok_or_else(|| anyhow::anyhow!("Project not found: {}", id))?;

            if json {
                println!("{}", serde_json::to_string_pretty(&project)?);
            } else {
                println!("Project Details:");
                println!("  ID:          {}", project.id);
                println!("  Name:        {}", project.name);
                println!("  Status:      {}", project.status);
                println!("  Working Dir: {}", project.working_directory.display());
                println!("  Created At:  {}", project.created_at);
                println!("  Updated At:  {}", project.updated_at);
                if let Some(archived) = project.archived_at {
                    println!("  Archived At: {}", archived);
                }
                if let Some(desc) = &project.description {
                    println!("  Description: {}", desc);
                }
            }
        }

        ProjectCommand::Update {
            id,
            name,
            description,
            working_dir,
            status,
            json,
        } => {
            let parsed_status = match status {
                Some(s) => Some(s.parse::<ProjectStatus>().map_err(|e| anyhow::anyhow!(e))?),
                None => None,
            };

            let input = UpdateProjectInput {
                name,
                description: description.map(Some),
                working_directory: working_dir,
                status: parsed_status,
                settings: None,
            };

            let updated = service.update_project(&id, input).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&updated)?);
            } else {
                println!("Project updated successfully!");
                println!("  ID:          {}", updated.id);
                println!("  Name:        {}", updated.name);
                println!("  Status:      {}", updated.status);
                println!("  Working Dir: {}", updated.working_directory.display());
            }
        }

        ProjectCommand::Archive { id, json } => {
            let archived = service.archive_project(&id).await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&archived)?);
            } else {
                println!("Project '{}' ({}) archived successfully.", archived.name, archived.id);
            }
        }

        ProjectCommand::Delete { id, force, json } => {
            if !force {
                eprintln!("Warning: Deleting a project will permanently delete all associated agent definitions.");
                eprintln!("Run with --force to confirm deletion of project '{}'.", id);
                anyhow::bail!("Deletion aborted without --force flag");
            }

            let deleted = service.delete_project(&id).await?;
            if json {
                println!("{{\"deleted\": {}, \"id\": \"{}\"}}", deleted, id);
            } else if deleted {
                println!("Project '{}' deleted successfully.", id);
            } else {
                println!("Project '{}' not found.", id);
            }
        }
    }

    Ok(())
}
