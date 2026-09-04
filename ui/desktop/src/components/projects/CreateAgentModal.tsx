import React, { useState, useCallback } from 'react';
import { X } from 'lucide-react';
import { Button } from '../ui/button';
import type { CreateAgentInput } from '../../types/platform.types';

interface CreateAgentModalProps {
  projectId: string;
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (input: CreateAgentInput) => Promise<void>;
}

export const CreateAgentModal: React.FC<CreateAgentModalProps> = ({ projectId, isOpen, onClose, onSubmit }) => {
  const [name, setName] = useState('');
  const [role, setRole] = useState('');
  const [provider, setProvider] = useState('anthropic');
  const [model, setModel] = useState('claude-3-7-sonnet-latest');
  const [description, setDescription] = useState('');
  const [systemPrompt, setSystemPrompt] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [validationError, setValidationError] = useState<string | null>(null);

  const handleSubmit = useCallback(
    async (e: React.FormEvent) => {
      e.preventDefault();
      setValidationError(null);

      if (!name.trim()) {
        setValidationError('Agent name is required');
        return;
      }
      if (!role.trim()) {
        setValidationError('Agent role is required');
        return;
      }
      if (!provider.trim() || !model.trim()) {
        setValidationError('Provider and Model are required');
        return;
      }

      setIsSubmitting(true);
      try {
        await onSubmit({
          project_id: projectId,
          name: name.trim(),
          role: role.trim(),
          model_settings: {
            provider: provider.trim(),
            model: model.trim(),
          },
          description: description.trim() || undefined,
          system_prompt: systemPrompt.trim() || undefined,
        });
        setName('');
        setRole('');
        setDescription('');
        setSystemPrompt('');
        onClose();
      } catch (err) {
        const msg = err instanceof Error ? err.message : 'Error creating agent';
        setValidationError(msg);
      } finally {
        setIsSubmitting(false);
      }
    },
    [projectId, name, role, provider, model, description, systemPrompt, onSubmit, onClose]
  );

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-xs p-4 overflow-y-auto">
      <div className="relative w-full max-w-lg rounded-2xl border border-border-subtle bg-background-primary p-6 shadow-2xl my-8">
        <div className="flex items-center justify-between pb-4 border-b border-border-subtle">
          <h2 className="text-lg font-semibold text-text-primary">Define New Agent</h2>
          <button
            type="button"
            onClick={onClose}
            className="rounded-full p-1 text-text-subtle hover:bg-background-secondary transition-colors"
            aria-label="Close modal"
          >
            <X className="size-5" />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="mt-4 space-y-4">
          {validationError && (
            <div className="rounded-lg bg-background-danger/10 border border-background-danger/20 p-3 text-xs text-text-danger">
              {validationError}
            </div>
          )}

          <div className="grid grid-cols-2 gap-3">
            <div>
              <label htmlFor="agent-name" className="block text-xs font-medium text-text-secondary mb-1">
                Agent Name *
              </label>
              <input
                id="agent-name"
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="e.g., Code Reviewer"
                className="w-full rounded-lg border border-border-subtle bg-background-secondary px-3 py-2 text-sm text-text-primary placeholder:text-text-subtle focus:border-border focus:outline-none transition-colors"
                disabled={isSubmitting}
              />
            </div>
            <div>
              <label htmlFor="agent-role" className="block text-xs font-medium text-text-secondary mb-1">
                Functional Role *
              </label>
              <input
                id="agent-role"
                type="text"
                value={role}
                onChange={(e) => setRole(e.target.value)}
                placeholder="e.g., Lead Reviewer"
                className="w-full rounded-lg border border-border-subtle bg-background-secondary px-3 py-2 text-sm text-text-primary placeholder:text-text-subtle focus:border-border focus:outline-none transition-colors"
                disabled={isSubmitting}
              />
            </div>
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div>
              <label htmlFor="agent-provider" className="block text-xs font-medium text-text-secondary mb-1">
                LLM Provider *
              </label>
              <input
                id="agent-provider"
                type="text"
                value={provider}
                onChange={(e) => setProvider(e.target.value)}
                placeholder="anthropic, openai, ollama..."
                className="w-full rounded-lg border border-border-subtle bg-background-secondary px-3 py-2 text-sm text-text-primary placeholder:text-text-subtle focus:border-border focus:outline-none transition-colors"
                disabled={isSubmitting}
              />
            </div>
            <div>
              <label htmlFor="agent-model" className="block text-xs font-medium text-text-secondary mb-1">
                Model ID *
              </label>
              <input
                id="agent-model"
                type="text"
                value={model}
                onChange={(e) => setModel(e.target.value)}
                placeholder="claude-3-7-sonnet, gpt-4o..."
                className="w-full rounded-lg border border-border-subtle bg-background-secondary px-3 py-2 text-sm text-text-primary placeholder:text-text-subtle focus:border-border focus:outline-none transition-colors"
                disabled={isSubmitting}
              />
            </div>
          </div>

          <div>
            <label htmlFor="agent-desc" className="block text-xs font-medium text-text-secondary mb-1">
              Description (Optional)
            </label>
            <input
              id="agent-desc"
              type="text"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Specialized agent for inspecting and testing pull requests"
              className="w-full rounded-lg border border-border-subtle bg-background-secondary px-3 py-2 text-sm text-text-primary placeholder:text-text-subtle focus:border-border focus:outline-none transition-colors"
              disabled={isSubmitting}
            />
          </div>

          <div>
            <label htmlFor="system-prompt" className="block text-xs font-medium text-text-secondary mb-1">
              System Persona Prompt
            </label>
            <textarea
              id="system-prompt"
              rows={4}
              value={systemPrompt}
              onChange={(e) => setSystemPrompt(e.target.value)}
              placeholder="You are a senior engineer specialized in security and performance audits..."
              className="w-full rounded-lg border border-border-subtle bg-background-secondary px-3 py-2 text-sm text-text-primary placeholder:text-text-subtle focus:border-border focus:outline-none transition-colors resize-none"
              disabled={isSubmitting}
            />
          </div>

          <div className="flex justify-end gap-3 pt-4 border-t border-border-subtle">
            <Button type="button" variant="ghost" onClick={onClose} disabled={isSubmitting}>
              Cancel
            </Button>
            <Button type="submit" variant="default" disabled={isSubmitting}>
              {isSubmitting ? 'Saving...' : 'Define Agent'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
};
