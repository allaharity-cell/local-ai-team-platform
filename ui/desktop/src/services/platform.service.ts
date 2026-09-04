import type {
  Project,
  CreateProjectInput,
  UpdateProjectInput,
  AgentDefinition,
  CreateAgentInput,
  UpdateAgentInput,
} from '../types/platform.types';

export class PlatformServiceError extends Error {
  constructor(message: string, public readonly originalError?: unknown) {
    super(message);
    this.name = 'PlatformServiceError';
  }
}

export const platformService = {
  // Project operations
  async listProjects(includeArchived = false): Promise<Project[]> {
    try {
      return await window.electron.listProjects(includeArchived);
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Unable to fetch projects list';
      throw new PlatformServiceError(`Failed to load projects: ${msg}`, err);
    }
  },

  async getProject(id: string): Promise<Project | null> {
    try {
      return await window.electron.getProject(id);
    } catch (err) {
      const msg = err instanceof Error ? err.message : `Unable to fetch project with ID ${id}`;
      throw new PlatformServiceError(`Failed to load project details: ${msg}`, err);
    }
  },

  async createProject(input: CreateProjectInput): Promise<Project> {
    try {
      return await window.electron.createProject(input);
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Failed to create new project';
      throw new PlatformServiceError(`Could not create project: ${msg}`, err);
    }
  },

  async updateProject(id: string, input: UpdateProjectInput): Promise<Project> {
    try {
      return await window.electron.updateProject(id, input);
    } catch (err) {
      const msg = err instanceof Error ? err.message : `Failed to update project ${id}`;
      throw new PlatformServiceError(`Could not update project: ${msg}`, err);
    }
  },

  async archiveProject(id: string): Promise<Project> {
    try {
      return await window.electron.archiveProject(id);
    } catch (err) {
      const msg = err instanceof Error ? err.message : `Failed to archive project ${id}`;
      throw new PlatformServiceError(`Could not archive project: ${msg}`, err);
    }
  },

  async deleteProject(id: string): Promise<boolean> {
    try {
      return await window.electron.deleteProject(id);
    } catch (err) {
      const msg = err instanceof Error ? err.message : `Failed to delete project ${id}`;
      throw new PlatformServiceError(`Could not delete project: ${msg}`, err);
    }
  },

  // Agent operations
  async listAgents(projectId: string, includeArchived = false): Promise<AgentDefinition[]> {
    try {
      return await window.electron.listAgents(projectId, includeArchived);
    } catch (err) {
      const msg = err instanceof Error ? err.message : `Unable to fetch agents for project ${projectId}`;
      throw new PlatformServiceError(`Failed to load project agents: ${msg}`, err);
    }
  },

  async getAgent(id: string): Promise<AgentDefinition | null> {
    try {
      return await window.electron.getAgent(id);
    } catch (err) {
      const msg = err instanceof Error ? err.message : `Unable to fetch agent ${id}`;
      throw new PlatformServiceError(`Failed to load agent definition: ${msg}`, err);
    }
  },

  async createAgent(input: CreateAgentInput): Promise<AgentDefinition> {
    try {
      return await window.electron.createAgent(input);
    } catch (err) {
      const msg = err instanceof Error ? err.message : 'Failed to create agent';
      throw new PlatformServiceError(`Could not create agent: ${msg}`, err);
    }
  },

  async updateAgent(id: string, input: UpdateAgentInput): Promise<AgentDefinition> {
    try {
      return await window.electron.updateAgent(id, input);
    } catch (err) {
      const msg = err instanceof Error ? err.message : `Failed to update agent ${id}`;
      throw new PlatformServiceError(`Could not update agent: ${msg}`, err);
    }
  },

  async archiveAgent(id: string): Promise<AgentDefinition> {
    try {
      return await window.electron.archiveAgent(id);
    } catch (err) {
      const msg = err instanceof Error ? err.message : `Failed to archive agent ${id}`;
      throw new PlatformServiceError(`Could not archive agent: ${msg}`, err);
    }
  },

  async deleteAgent(id: string): Promise<boolean> {
    try {
      return await window.electron.deleteAgent(id);
    } catch (err) {
      const msg = err instanceof Error ? err.message : `Failed to delete agent ${id}`;
      throw new PlatformServiceError(`Could not delete agent: ${msg}`, err);
    }
  },
};
