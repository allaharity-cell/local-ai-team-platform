import { useCallback, useEffect, useState } from 'react';
import type { Project, CreateProjectInput } from '../types/platform.types';
import { platformService, PlatformServiceError } from '../services/platform.service';

interface UseProjectsReturn {
  projects: Project[];
  loading: boolean;
  error: string | null;
  refresh: () => Promise<void>;
  createProject: (input: CreateProjectInput) => Promise<Project>;
  archiveProject: (id: string) => Promise<void>;
  deleteProject: (id: string) => Promise<void>;
}

export const useProjects = (includeArchived = false): UseProjectsReturn => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const list = await platformService.listProjects(includeArchived);
      setProjects(list);
    } catch (err) {
      const msg = err instanceof PlatformServiceError ? err.message : 'Could not retrieve projects list';
      setError(msg);
    } finally {
      setLoading(false);
    }
  }, [includeArchived]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const createProject = useCallback(
    async (input: CreateProjectInput): Promise<Project> => {
      setError(null);
      try {
        const created = await platformService.createProject(input);
        await refresh();
        return created;
      } catch (err) {
        const msg = err instanceof PlatformServiceError ? err.message : 'Failed to create project';
        setError(msg);
        throw err;
      }
    },
    [refresh]
  );

  const archiveProject = useCallback(
    async (id: string): Promise<void> => {
      setError(null);
      try {
        await platformService.archiveProject(id);
        await refresh();
      } catch (err) {
        const msg = err instanceof PlatformServiceError ? err.message : 'Failed to archive project';
        setError(msg);
        throw err;
      }
    },
    [refresh]
  );

  const deleteProject = useCallback(
    async (id: string): Promise<void> => {
      setError(null);
      try {
        await platformService.deleteProject(id);
        await refresh();
      } catch (err) {
        const msg = err instanceof PlatformServiceError ? err.message : 'Failed to delete project';
        setError(msg);
        throw err;
      }
    },
    [refresh]
  );

  return {
    projects,
    loading,
    error,
    refresh,
    createProject,
    archiveProject,
    deleteProject,
  };
};
