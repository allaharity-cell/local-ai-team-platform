import { app, ipcMain } from 'electron';
import fs from 'node:fs';
import path from 'node:path';
import { v4 as uuidv4 } from 'uuid';
import type {
  Project,
  CreateProjectInput,
  UpdateProjectInput,
  AgentDefinition,
  CreateAgentInput,
  UpdateAgentInput,
} from '../types/platform.types';

interface PlatformStorageData {
  version: number;
  projects: Record<string, Project>;
  agents: Record<string, AgentDefinition>;
}

const getPlatformStoragePath = (): string => {
  return path.join(app.getPath('userData'), 'platform_storage.json');
};

const loadStorage = (): PlatformStorageData => {
  const filePath = getPlatformStoragePath();
  try {
    if (fs.existsSync(filePath)) {
      const raw = fs.readFileSync(filePath, 'utf-8');
      return JSON.parse(raw);
    }
  } catch (err) {
    console.error('Failed to load platform storage, initializing fresh:', err);
  }
  return { version: 1, projects: {}, agents: {} };
};

const saveStorage = (data: PlatformStorageData): void => {
  const filePath = getPlatformStoragePath();
  const dir = path.dirname(filePath);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
  fs.writeFileSync(filePath, JSON.stringify(data, null, 2), 'utf-8');
};

export const registerPlatformIpcHandlers = (): void => {
  ipcMain.handle('platform:list-projects', async (_event, includeArchived = false) => {
    const data = loadStorage();
    const list = Object.values(data.projects);
    const filtered = includeArchived ? list : list.filter((p) => p.status !== 'archived');
    return filtered.sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime());
  });

  ipcMain.handle('platform:get-project', async (_event, id: string) => {
    const data = loadStorage();
    return data.projects[id] ?? null;
  });

  ipcMain.handle('platform:create-project', async (_event, input: CreateProjectInput) => {
    if (!input.name || input.name.trim().length === 0) {
      throw new Error('Project name cannot be empty');
    }
    if (input.name.trim().length > 100) {
      throw new Error('Project name cannot exceed 100 characters');
    }
    if (!input.working_directory || input.working_directory.trim().length === 0) {
      throw new Error('Working directory must be provided');
    }

    const resolvedPath = path.resolve(input.working_directory.trim());
    if (!fs.existsSync(resolvedPath)) {
      fs.mkdirSync(resolvedPath, { recursive: true });
    }

    const data = loadStorage();
    const now = new Date().toISOString();
    const project: Project = {
      id: uuidv4(),
      name: input.name.trim(),
      description: input.description?.trim(),
      working_directory: resolvedPath,
      status: 'active',
      settings: input.settings || {},
      created_at: now,
      updated_at: now,
    };

    data.projects[project.id] = project;
    saveStorage(data);
    return project;
  });

  ipcMain.handle('platform:update-project', async (_event, id: string, input: UpdateProjectInput) => {
    const data = loadStorage();
    const project = data.projects[id];
    if (!project) {
      throw new Error(`Project ${id} not found`);
    }

    if (input.name !== undefined) {
      if (input.name.trim().length === 0) throw new Error('Project name cannot be empty');
      if (input.name.trim().length > 100) throw new Error('Project name cannot exceed 100 characters');
      project.name = input.name.trim();
    }
    if (input.description !== undefined) {
      project.description = input.description.trim();
    }
    if (input.working_directory !== undefined) {
      const resolved = path.resolve(input.working_directory.trim());
      if (!fs.existsSync(resolved)) fs.mkdirSync(resolved, { recursive: true });
      project.working_directory = resolved;
    }
    if (input.status !== undefined) {
      project.status = input.status;
      if (input.status === 'archived') project.archived_at = new Date().toISOString();
    }
    if (input.settings !== undefined) {
      project.settings = input.settings;
    }
    project.updated_at = new Date().toISOString();

    data.projects[id] = project;
    saveStorage(data);
    return project;
  });

  ipcMain.handle('platform:archive-project', async (_event, id: string) => {
    const data = loadStorage();
    const project = data.projects[id];
    if (!project) throw new Error(`Project ${id} not found`);
    project.status = 'archived';
    project.archived_at = new Date().toISOString();
    project.updated_at = new Date().toISOString();
    data.projects[id] = project;
    saveStorage(data);
    return project;
  });

  ipcMain.handle('platform:delete-project', async (_event, id: string) => {
    const data = loadStorage();
    if (!data.projects[id]) return false;
    delete data.projects[id];
    // Cascade deletion to scoped agents
    for (const [agentId, agent] of Object.entries(data.agents)) {
      if (agent.project_id === id) {
        delete data.agents[agentId];
      }
    }
    saveStorage(data);
    return true;
  });

  ipcMain.handle('platform:list-agents', async (_event, projectId: string, includeArchived = false) => {
    const data = loadStorage();
    const agents = Object.values(data.agents).filter((a) => a.project_id === projectId);
    const filtered = includeArchived ? agents : agents.filter((a) => a.status !== 'archived');
    return filtered.sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime());
  });

  ipcMain.handle('platform:get-agent', async (_event, id: string) => {
    const data = loadStorage();
    return data.agents[id] ?? null;
  });

  ipcMain.handle('platform:create-agent', async (_event, input: CreateAgentInput) => {
    const data = loadStorage();
    if (!data.projects[input.project_id]) {
      throw new Error(`Project ${input.project_id} does not exist`);
    }
    if (!input.name || input.name.trim().length === 0) throw new Error('Agent name cannot be empty');
    if (!input.role || input.role.trim().length === 0) throw new Error('Agent role cannot be empty');
    if (!input.model_settings?.provider || !input.model_settings?.model) {
      throw new Error('Agent model settings must specify both provider and model');
    }

    const now = new Date().toISOString();
    const agent: AgentDefinition = {
      id: uuidv4(),
      project_id: input.project_id,
      name: input.name.trim(),
      role: input.role.trim(),
      description: input.description?.trim(),
      system_prompt: input.system_prompt?.trim(),
      model_settings: input.model_settings,
      enabled_tools: input.enabled_tools || [],
      mcp_config: input.mcp_config || { servers: [] },
      permission_policy: input.permission_policy || { default_action: 'ask', allowed_tools: [], denied_tools: [] },
      delegation_policy: input.delegation_policy || { allow_delegation: false, allowed_targets: [] },
      approval_policy: input.approval_policy || { require_approval_for_write: true, require_approval_for_shell: true },
      status: 'active',
      created_at: now,
      updated_at: now,
    };

    data.agents[agent.id] = agent;
    saveStorage(data);
    return agent;
  });

  ipcMain.handle('platform:update-agent', async (_event, id: string, input: UpdateAgentInput) => {
    const data = loadStorage();
    const agent = data.agents[id];
    if (!agent) throw new Error(`Agent ${id} not found`);

    if (input.name !== undefined) {
      if (input.name.trim().length === 0) throw new Error('Agent name cannot be empty');
      agent.name = input.name.trim();
    }
    if (input.role !== undefined) {
      if (input.role.trim().length === 0) throw new Error('Agent role cannot be empty');
      agent.role = input.role.trim();
    }
    if (input.description !== undefined) agent.description = input.description.trim();
    if (input.system_prompt !== undefined) agent.system_prompt = input.system_prompt.trim();
    if (input.model_settings !== undefined) agent.model_settings = input.model_settings;
    if (input.enabled_tools !== undefined) agent.enabled_tools = input.enabled_tools;
    if (input.mcp_config !== undefined) agent.mcp_config = input.mcp_config;
    if (input.permission_policy !== undefined) agent.permission_policy = input.permission_policy;
    if (input.delegation_policy !== undefined) agent.delegation_policy = input.delegation_policy;
    if (input.approval_policy !== undefined) agent.approval_policy = input.approval_policy;
    if (input.status !== undefined) agent.status = input.status;

    agent.updated_at = new Date().toISOString();
    data.agents[id] = agent;
    saveStorage(data);
    return agent;
  });

  ipcMain.handle('platform:archive-agent', async (_event, id: string) => {
    const data = loadStorage();
    const agent = data.agents[id];
    if (!agent) throw new Error(`Agent ${id} not found`);
    agent.status = 'archived';
    agent.updated_at = new Date().toISOString();
    data.agents[id] = agent;
    saveStorage(data);
    return agent;
  });

  ipcMain.handle('platform:delete-agent', async (_event, id: string) => {
    const data = loadStorage();
    if (!data.agents[id]) return false;
    delete data.agents[id];
    saveStorage(data);
    return true;
  });
};
