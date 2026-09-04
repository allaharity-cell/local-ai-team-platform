import { useCallback, useEffect, useState } from 'react';
import type { AgentDefinition, CreateAgentInput } from '../types/platform.types';
import { platformService, PlatformServiceError } from '../services/platform.service';

interface UseProjectAgentsReturn {
  agents: AgentDefinition[];
  loading: boolean;
  error: string | null;
  refresh: () => Promise<void>;
  createAgent: (input: CreateAgentInput) => Promise<AgentDefinition>;
  archiveAgent: (id: string) => Promise<void>;
  deleteAgent: (id: string) => Promise<void>;
}

export const useProjectAgents = (projectId: string, includeArchived = false): UseProjectAgentsReturn => {
  const [agents, setAgents] = useState<AgentDefinition[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    if (!projectId) return;
    setLoading(true);
    setError(null);
    try {
      const list = await platformService.listAgents(projectId, includeArchived);
      setAgents(list);
    } catch (err) {
      const msg = err instanceof PlatformServiceError ? err.message : 'Could not retrieve project agents';
      setError(msg);
    } finally {
      setLoading(false);
    }
  }, [projectId, includeArchived]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const createAgent = useCallback(
    async (input: CreateAgentInput): Promise<AgentDefinition> => {
      setError(null);
      try {
        const created = await platformService.createAgent(input);
        await refresh();
        return created;
      } catch (err) {
        const msg = err instanceof PlatformServiceError ? err.message : 'Failed to create agent';
        setError(msg);
        throw err;
      }
    },
    [refresh]
  );

  const archiveAgent = useCallback(
    async (id: string): Promise<void> => {
      setError(null);
      try {
        await platformService.archiveAgent(id);
        await refresh();
      } catch (err) {
        const msg = err instanceof PlatformServiceError ? err.message : 'Failed to archive agent';
        setError(msg);
        throw err;
      }
    },
    [refresh]
  );

  const deleteAgent = useCallback(
    async (id: string): Promise<void> => {
      setError(null);
      try {
        await platformService.deleteAgent(id);
        await refresh();
      } catch (err) {
        const msg = err instanceof PlatformServiceError ? err.message : 'Failed to delete agent';
        setError(msg);
        throw err;
      }
    },
    [refresh]
  );

  return {
    agents,
    loading,
    error,
    refresh,
    createAgent,
    archiveAgent,
    deleteAgent,
  };
};
