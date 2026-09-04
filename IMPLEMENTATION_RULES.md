# Implementation Rules: Strict Engineering Discipline

These engineering rules are **mandatory and non-negotiable** for all current and future platform development. Every pull request, commit, and code modification must strictly adhere to these invariants.

---

## 1. Upstream Preservation & Architectural Modularity

1. **Never Rewrite Stable Goose Infrastructure Without Justification**: Goose provides a battle-tested agent state machine, MCP client, ACP implementation, and provider abstractions. Core logic must remain untouched unless an unavoidable upstream bug is detected.
2. **Favor Adapters Over Core Edits**: All new capabilities (Projects, Teams, Visual Builders, DAG Tasks) must be implemented in isolated crates (`crates/platform-core`, etc.) or dedicated extension modules.
3. **No Giant Architectural Rewrites**: Evolution must be incremental and testable. Refactor one layer at a time.
4. **Preserve Upstream Compatibility**: The platform must remain rebase-compatible with upstream `aaif-goose/goose`. Never remove or break upstream ACP schemas, CLI flags, or public library traits.
5. **Preserve Apache 2.0 Notices & Attribution**: Upstream license headers, copyright notices, and license files must be preserved in full across all crates and components.

---

## 2. Security, Keys & Permissions

6. **Never Hardcode API Keys or Secrets**: All credentials must be sourced strictly from the host OS keyring (macOS Keychain, Linux Secret Service, Windows Credential Manager) via `keyring` or user-supplied environment variables.
7. **Never Bypass Permissions**: Every tool call, shell command, file write, or external network request must pass through the Permission Judge and respect the agent's assigned `PermissionPolicy`.
8. **Strict Human-in-the-Loop for Mutations**: Destructive operations (overwriting files outside scratch directories, executing bash commands, dropping databases, issuing network requests) must default to requiring user confirmation unless explicitly whitelisted in project settings.
9. **Never Log Sensitive Tokens or Embeddings**: Audit logs and telemetry streams must redact API keys, bearer tokens, and sensitive headers before writing to disk or database.

---

## 3. UI/UX Standards & Operational Truth

10. **Never Create "Fake" or Synthetic UI**:
    - Every button, slider, toggle, and dropdown rendered on the screen must connect to a functional backend API, state store, or operational action.
    - Placeholder views, mock toggles that do nothing, and decorative controls are strictly forbidden.
    - If a feature is scheduled for a future milestone, do not render inactive buttons—hide or omit the control entirely until functional.
11. **Every UI Control Must Work**: Visual Agent and Team builders must immediately persist changes to SQLite, with immediate validation and clear user feedback on failure.
12. **Visual Feedback & Cinematic Polish**: Follow modern standards (8pt grid, fluid typography, micro-interactions, responsive states, no CLS, full accessibility with keyboard navigation).

---

## 4. Local-First & Memory Isolation

13. **Maintain Local-First Architecture**: User data, session conversations, task DAGs, agent definitions, and SQLite databases must reside exclusively on the user's host machine. No external telemetry or cloud sync without explicit user opt-in.
14. **Keep Agents Strictly Project-Scoped**: An agent instance executes solely within the directory context and permission boundaries of its designated project.
15. **Prevent Memory Leakage Between Projects**: Project semantic memories, vector stores, and cached conversation context must be tagged with unique `project_id`s and queried with strict tenant filtering. An agent executing in Project A must never retrieve memories from Project B.
16. **Deterministic Directory Sandboxing**: All tool execution paths must be resolved relative to the project working directory. Directory traversal outside project roots must be blocked by path validation inspectors.

---

## 5. Code Quality, Testing & Git Discipline

17. **Strict TypeScript & Rust Standards**:
    - Rust: Zero compiler warnings, idiomatic error handling with `Result<T, anyhow::Error>` / `thiserror`, strict Clippy conformance.
    - TypeScript: Strict mode enabled, no `any`, typed props, named interfaces.
18. **Run Tests After Modifications**:
    - Rust: `cargo test --workspace` must pass before any commit.
    - UI: `pnpm run typecheck` and `pnpm test` must pass before any UI commit.
19. **Use Small, Atomic Commits**:
    - Commit messages must follow conventional format (`feat:`, `fix:`, `refactor:`, `docs:`).
    - Commits must represent single logical changes that can be bisected or reviewed independently.
20. **Error Handling Is Not Optional**:
    - Every `async` block must have explicit error handling.
    - User-facing error messages must be actionable and descriptive (never "An unknown error occurred").
