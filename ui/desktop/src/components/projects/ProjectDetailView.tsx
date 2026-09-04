import React, { useEffect, useState, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router';
import { ArrowLeft, FolderOpen, Archive, Trash2, Calendar, HardDrive } from 'lucide-react';
import { Button } from '../ui/button';
import { AgentListView } from './AgentListView';
import { platformService } from '../../services/platform.service';
import type { Project } from '../../types/platform.types';

export const ProjectDetailView: React.FC = () => {
  const { projectId } = useParams<{ projectId: string }>();
  const navigate = useNavigate();
  const [project, setProject] = useState<Project | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadProject = useCallback(async () => {
    if (!projectId) return;
    setLoading(true);
    setError(null);
    try {
      const data = await platformService.getProject(projectId);
      setProject(data);
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Could not fetch project details';
      setError(msg);
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  useEffect(() => {
    void loadProject();
  }, [loadProject]);

  const handleOpenDirectory = useCallback(() => {
    if (project?.working_directory) {
      window.electron.openDirectoryInExplorer(project.working_directory);
    }
  }, [project]);

  const handleArchive = useCallback(async () => {
    if (!projectId) return;
    try {
      await platformService.archiveProject(projectId);
      await loadProject();
    } catch (err) {
      console.error('Failed to archive project:', err);
    }
  }, [projectId, loadProject]);

  const handleDelete = useCallback(async () => {
    if (!projectId) return;
    if (!confirm('Are you sure you want to delete this project and all its agent definitions?')) {
      return;
    }
    try {
      await platformService.deleteProject(projectId);
      navigate('/projects');
    } catch (err) {
      console.error('Failed to delete project:', err);
    }
  }, [projectId, navigate]);

  if (loading) {
    return (
      <div className="flex h-full items-center justify-center p-8 text-xs text-text-subtle">
        Loading project details...
      </div>
    );
  }

  if (error || !project) {
    return (
      <div className="p-8 space-y-4">
        <Button variant="ghost" size="sm" onClick={() => navigate('/projects')}>
          <ArrowLeft className="size-4 mr-1.5" />
          Back to Projects
        </Button>
        <div className="rounded-lg bg-background-danger/10 border border-background-danger/20 p-4 text-sm text-text-danger">
          {error || 'Project not found'}
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full overflow-y-auto p-8 space-y-8 max-w-5xl mx-auto">
      <div>
        <Button variant="ghost" size="sm" onClick={() => navigate('/projects')} className="mb-4">
          <ArrowLeft className="size-4 mr-1.5" />
          Back to Projects
        </Button>

        <div className="flex items-start justify-between">
          <div>
            <div className="flex items-center gap-3">
              <h1 className="text-2xl font-bold text-text-primary">{project.name}</h1>
              <span
                className={`rounded-full px-2.5 py-0.5 text-xs font-semibold uppercase tracking-wide ${
                  project.status === 'active'
                    ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400'
                    : 'bg-zinc-500/10 text-zinc-500'
                }`}
              >
                {project.status}
              </span>
            </div>
            {project.description && (
              <p className="mt-2 text-sm text-text-secondary">{project.description}</p>
            )}
          </div>

          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" onClick={handleOpenDirectory}>
              <FolderOpen className="size-4 mr-1.5" />
              Open Folder
            </Button>
            <Button variant="ghost" size="sm" onClick={handleArchive}>
              <Archive className="size-4 mr-1.5" />
              Archive
            </Button>
            <Button
              variant="ghost"
              size="sm"
              className="text-text-danger hover:bg-background-danger/10"
              onClick={handleDelete}
            >
              <Trash2 className="size-4 mr-1.5" />
              Delete
            </Button>
          </div>
        </div>

        <div className="mt-6 flex flex-wrap gap-4 text-xs text-text-subtle border-y border-border-subtle py-3">
          <div className="flex items-center gap-1.5">
            <HardDrive className="size-4" />
            <span className="font-mono text-text-secondary">{project.working_directory}</span>
          </div>
          <div className="flex items-center gap-1.5">
            <Calendar className="size-4" />
            <span>Created {new Date(project.created_at).toLocaleDateString()}</span>
          </div>
        </div>
      </div>

      <AgentListView projectId={project.id} />
    </div>
  );
};
