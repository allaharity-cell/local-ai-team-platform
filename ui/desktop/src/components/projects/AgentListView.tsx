import React, { useState, useCallback } from 'react';
import { Bot, Plus, Trash2, Archive, Cpu } from 'lucide-react';
import { Button } from '../ui/button';
import { useProjectAgents } from '../../hooks/useProjectAgents';
import { CreateAgentModal } from './CreateAgentModal';
import type { CreateAgentInput } from '../../types/platform.types';

interface AgentListViewProps {
  projectId: string;
}

export const AgentListView: React.FC<AgentListViewProps> = ({ projectId }) => {
  const { agents, loading, error, createAgent, archiveAgent, deleteAgent } = useProjectAgents(projectId);
  const [isCreateOpen, setIsCreateOpen] = useState(false);

  const handleCreateAgent = useCallback(
    async (input: CreateAgentInput) => {
      await createAgent(input);
    },
    [createAgent]
  );

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-base font-semibold text-text-primary">Project Agents</h3>
          <p className="text-xs text-text-subtle">
            Specialized agents scoped to this project with dedicated roles, models, and policies.
          </p>
        </div>
        <Button variant="default" size="sm" onClick={() => setIsCreateOpen(true)}>
          <Plus className="size-4 mr-1.5" />
          Add Agent
        </Button>
      </div>

      {error && (
        <div className="rounded-lg bg-background-danger/10 border border-background-danger/20 p-3 text-xs text-text-danger">
          {error}
        </div>
      )}

      {loading ? (
        <div className="flex items-center justify-center p-8 text-xs text-text-subtle">
          Loading project agents...
        </div>
      ) : agents.length === 0 ? (
        <div className="flex flex-col items-center justify-center rounded-xl border border-dashed border-border-subtle p-8 text-center bg-background-secondary/30">
          <Bot className="size-10 text-text-subtle mb-2 opacity-50" />
          <p className="text-sm font-medium text-text-primary">No agents defined yet</p>
          <p className="text-xs text-text-subtle mt-1 mb-4">
            Add your first specialized agent to assign responsibilities within this project.
          </p>
          <Button variant="outline" size="sm" onClick={() => setIsCreateOpen(true)}>
            <Plus className="size-4 mr-1.5" />
            Create Agent
          </Button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
          {agents.map((agent) => (
            <div
              key={agent.id}
              className="flex flex-col justify-between rounded-xl border border-border-subtle bg-background-secondary/40 p-4 transition-all hover:border-border"
            >
              <div>
                <div className="flex items-start justify-between">
                  <div className="flex items-center gap-2">
                    <div className="flex size-8 items-center justify-center rounded-lg bg-background-tertiary text-text-primary">
                      <Bot className="size-4" />
                    </div>
                    <div>
                      <h4 className="text-sm font-medium text-text-primary">{agent.name}</h4>
                      <p className="text-xs text-text-subtle">{agent.role}</p>
                    </div>
                  </div>
                  <span
                    className={`rounded-full px-2 py-0.5 text-[10px] font-medium uppercase tracking-wider ${
                      agent.status === 'active'
                        ? 'bg-emerald-500/10 text-emerald-600 dark:text-emerald-400'
                        : 'bg-zinc-500/10 text-zinc-500'
                    }`}
                  >
                    {agent.status}
                  </span>
                </div>

                {agent.description && (
                  <p className="mt-2 text-xs text-text-secondary line-clamp-2">{agent.description}</p>
                )}

                <div className="mt-3 flex flex-wrap items-center gap-2">
                  <span className="inline-flex items-center gap-1 rounded-md bg-background-tertiary px-2 py-1 text-[11px] font-mono text-text-secondary">
                    <Cpu className="size-3" />
                    {agent.model_settings.provider}:{agent.model_settings.model}
                  </span>
                  {agent.enabled_tools.length > 0 && (
                    <span className="rounded-md bg-background-tertiary px-2 py-1 text-[11px] text-text-secondary">
                      {agent.enabled_tools.length} tools
                    </span>
                  )}
                </div>
              </div>

              <div className="mt-4 flex items-center justify-end gap-1 border-t border-border-subtle pt-2">
                <Button
                  variant="ghost"
                  size="xs"
                  onClick={() => archiveAgent(agent.id)}
                  title="Archive agent"
                >
                  <Archive className="size-3 mr-1" />
                  Archive
                </Button>
                <Button
                  variant="ghost"
                  size="xs"
                  className="text-text-danger hover:bg-background-danger/10"
                  onClick={() => deleteAgent(agent.id)}
                  title="Delete agent"
                >
                  <Trash2 className="size-3 mr-1" />
                  Delete
                </Button>
              </div>
            </div>
          ))}
        </div>
      )}

      <CreateAgentModal
        projectId={projectId}
        isOpen={isCreateOpen}
        onClose={() => setIsCreateOpen(false)}
        onSubmit={handleCreateAgent}
      />
    </div>
  );
};
