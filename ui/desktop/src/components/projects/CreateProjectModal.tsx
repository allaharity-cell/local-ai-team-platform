import React, { useState, useCallback } from 'react';
import { Folder, X } from 'lucide-react';
import { Button } from '../ui/button';
import type { CreateProjectInput } from '../../types/platform.types';

interface CreateProjectModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (input: CreateProjectInput) => Promise<void>;
}

export const CreateProjectModal: React.FC<CreateProjectModalProps> = ({ isOpen, onClose, onSubmit }) => {
  const [name, setName] = useState('');
  const [workingDir, setWorkingDir] = useState('');
  const [description, setDescription] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [validationError, setValidationError] = useState<string | null>(null);

  const handleChooseDirectory = useCallback(async () => {
    try {
      const result = await window.electron.directoryChooser();
      if (!result.canceled && result.filePaths.length > 0) {
        setWorkingDir(result.filePaths[0]);
      }
    } catch (err) {
      console.error('Failed to open directory chooser:', err);
    }
  }, []);

  const handleSubmit = useCallback(
    async (e: React.FormEvent) => {
      e.preventDefault();
      setValidationError(null);

      if (!name.trim()) {
        setValidationError('Project name is required');
        return;
      }
      if (!workingDir.trim()) {
        setValidationError('Working directory is required');
        return;
      }

      setIsSubmitting(true);
      try {
        await onSubmit({
          name: name.trim(),
          working_directory: workingDir.trim(),
          description: description.trim() || undefined,
        });
        setName('');
        setWorkingDir('');
        setDescription('');
        onClose();
      } catch (err) {
        const msg = err instanceof Error ? err.message : 'Error creating project';
        setValidationError(msg);
      } finally {
        setIsSubmitting(false);
      }
    },
    [name, workingDir, description, onSubmit, onClose]
  );

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-xs p-4">
      <div className="relative w-full max-w-lg rounded-2xl border border-border-subtle bg-background-primary p-6 shadow-2xl">
        <div className="flex items-center justify-between pb-4 border-b border-border-subtle">
          <h2 className="text-lg font-semibold text-text-primary">Create New Project</h2>
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

          <div>
            <label htmlFor="project-name" className="block text-xs font-medium text-text-secondary mb-1">
              Project Name *
            </label>
            <input
              id="project-name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., Quantum Core Platform"
              className="w-full rounded-lg border border-border-subtle bg-background-secondary px-3 py-2 text-sm text-text-primary placeholder:text-text-subtle focus:border-border focus:outline-none transition-colors"
              disabled={isSubmitting}
            />
          </div>

          <div>
            <label htmlFor="working-dir" className="block text-xs font-medium text-text-secondary mb-1">
              Working Directory *
            </label>
            <div className="flex gap-2">
              <input
                id="working-dir"
                type="text"
                value={workingDir}
                onChange={(e) => setWorkingDir(e.target.value)}
                placeholder="/path/to/project/root"
                className="flex-1 rounded-lg border border-border-subtle bg-background-secondary px-3 py-2 text-sm text-text-primary placeholder:text-text-subtle focus:border-border focus:outline-none transition-colors"
                disabled={isSubmitting}
              />
              <Button
                type="button"
                variant="outline"
                size="default"
                onClick={handleChooseDirectory}
                disabled={isSubmitting}
              >
                <Folder className="size-4 mr-1.5" />
                Browse
              </Button>
            </div>
          </div>

          <div>
            <label htmlFor="description" className="block text-xs font-medium text-text-secondary mb-1">
              Description (Optional)
            </label>
            <textarea
              id="description"
              rows={3}
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Brief context regarding goals, tech stack, and scope..."
              className="w-full rounded-lg border border-border-subtle bg-background-secondary px-3 py-2 text-sm text-text-primary placeholder:text-text-subtle focus:border-border focus:outline-none transition-colors resize-none"
              disabled={isSubmitting}
            />
          </div>

          <div className="flex justify-end gap-3 pt-4 border-t border-border-subtle">
            <Button type="button" variant="ghost" onClick={onClose} disabled={isSubmitting}>
              Cancel
            </Button>
            <Button type="submit" variant="default" disabled={isSubmitting}>
              {isSubmitting ? 'Creating...' : 'Create Project'}
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
};
