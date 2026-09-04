import React, { useState, useCallback } from 'react';
import { useNavigate } from 'react-router';
import { FolderGit2, Plus, FolderOpen, ArrowRight, Archive, Trash2 } from 'lucide-react';
import { Button } from '../ui/button';
import { useProjects } from '../../hooks/useProjects';
import { CreateProjectModal } from './CreateProjectModal';
import type { CreateProjectInput } from '../../types/platform.types';

export const ProjectsView: React.FC = () => {
  const navigate = useNavigate();
  const { projects, loading, error, createProject, archiveProject, deleteProject } = useProjects();
  const [isCreateOpen, setIsCreateOpen] = useState(false);

  const handleCreate = useCallback(
    async (input: CreateProjectInput) => {
      const created = await createProject(input);
      navigate(`/projects/${created.id}`);
    },
    [createProject, navigate]
  );

  return (
    <div className="flex flex-col h-full overflow-y-auto p-8 space-y-6 max-w-5xl mx-auto">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-text-primary">Projects</h1>
          <p className="text-sm text-text-subtle mt-1">
            Isolate your multi-agent teams, working directories, and specialized agent definitions.
          </p>
        </div>
        <Button variant="default" size="sm" onClick={() => setIsCreateOpen(true)}>
          <Plus className="size-4 mr-1.5" />
          New Project
        </Button>
      </div>

      {error && (
        <div className="rounded-lg bg-background-danger/10 border border-background-danger/20 p-4 text-sm text-text-danger">
          {error}
        </div>
      )}

      {loading ? (
        <div className="flex items-center justify-center p-12 text-sm text-text-subtle">
          Loading platform projects...
        </div>
      ) : projects.length === 0 ? (
        <div className="flex flex-col items-center justify-center rounded-2xl border border-dashed border-border-subtle p-12 text-center bg-background-secondary/30">
          <FolderGit2 className="size-12 text-text-subtle mb-3 opacity-40" />
          <h3 className="text-base font-medium text-text-primary">No projects yet</h3>
          <p className="text-sm text-text-subtle mt-1 mb-6 max-w-sm">
            Create a project to bind your codebase to dedicated agents and orchestrate automated workflows.
          </p>
          <Button variant="default" size="default" onClick={() => setIsCreateOpen(true)}>
            <Plus className="size-4 mr-1.5" />
            Create Your First Project
          </Button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {projects.map((project) => (
            <div
              key={project.id}
              className="flex flex-col justify-between rounded-xl border border-border-subtle bg-background-secondary/40 p-5 transition-all hover:border-border hover:shadow-sm"
            >
              <div>
                <div className="flex items-start justify-between">
                  <div className="flex items-center gap-2.5">
                    <div className="flex size-9 items-center justify-center rounded-lg bg-background-tertiary text-text-primary">
                      <FolderGit2 className="size-5" />
                    </div>
                    <div>
                      <h3 className="text-base font-medium text-text-primary">{project.name}</h3>
                      <span
                        className={`rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wider ${
                          project.status === 'active'
                            ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400'
                            : 'bg-zinc-500/10 text-zinc-500'
                        }`}
                      >
                        {project.status}
                      </span>
                    </div>
                  </div>
                </div>

                {project.description && (
                  <p className="mt-3 text-xs text-text-secondary line-clamp-2">{project.description}</p>
                )}

                <div className="mt-4 rounded-md bg-background-tertiary/60 px-2.5 py-1.5 font-mono text-[11px] text-text-subtle truncate">
                  {project.working_directory}
                </div>
              </div>

              <div className="mt-5 flex items-center justify-between border-t border-border-subtle pt-3">
                <div className="flex items-center gap-1">
                  <Button
                    variant="ghost"
                    size="xs"
                    onClick={() => window.electron.openDirectoryInExplorer(project.working_directory)}
                    title="Open Directory in Explorer"
                  >
                    <FolderOpen className="size-3.5 mr-1" />
                    Folder
                  </Button>
                  <Button
                    variant="ghost"
                    size="xs"
                    onClick={() => archiveProject(project.id)}
                    title="Archive Project"
                  >
                    <Archive className="size-3.5 mr-1" />
                    Archive
                  </Button>
                  <Button
                    variant="ghost"
                    size="xs"
                    className="text-text-danger hover:bg-background-danger/10"
                    onClick={() => deleteProject(project.id)}
                    title="Delete Project"
                  >
                    <Trash2 className="size-3.5 mr-1" />
                    Delete
                  </Button>
                </div>

                <Button
                  variant="default"
                  size="sm"
                  onClick={() => navigate(`/projects/${project.id}`)}
                >
                  Open
                  <ArrowRight className="size-3.5 ml-1" />
                </Button>
              </div>
            </div>
          ))}
        </div>
      )}

      <CreateProjectModal
        isOpen={isCreateOpen}
        onClose={() => setIsCreateOpen(false)}
        onSubmit={handleCreate}
      />
    </div>
  );
};
