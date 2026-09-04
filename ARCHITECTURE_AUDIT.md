# Architecture Audit: Goose Codebase Analysis

## 1. Executive Summary

This document provides an exhaustive architectural assessment of the upstream Goose codebase (`v1.49.0`) to evaluate its suitability as the foundation for the **Local AI Team Platform**.

**Verdict**: Goose is an **exceptionally strong foundation** for a local multi-agent platform. Its architecture is modular, strictly typed in modern Rust, features production-grade Model Context Protocol (MCP) and Agent Client Protocol (ACP) implementations, supports 15+ LLM providers out of the box, and provides an extensible Desktop (Electron/React 19/Tailwind) and CLI interface. 

Critically, Goose already possesses rudimentary primitives for subagent delegation (`summon`), multi-agent session coordination (`orchestrator`), session persistence in SQLite, and local memory (`goose-mcp/memory`). Transforming this into our target platform requires building domain-level abstractions (Workspaces, Projects, Teams, Visual Builders) on top of Goose's execution harness rather than rewriting core runtime logic.

---

## 2. Cargo Workspace & Crate Topography

The repository is structured as a Cargo workspace with 15 crates:

| Crate | Primary Role | Key Dependencies | Reusability Rating |
| :--- | :--- | :--- | :--- |
| **`goose`** | Core runtime library, agent state machine, tool harness, ACP/MCP integration, platform extensions | `tokio`, `sqlx`, `rmcp`, `agent-client-protocol`, `keyring` | **100% Reuse** (Core engine) |
| **`goose-providers`** | LLM provider abstractions and client implementations | `reqwest`, `serde_json`, `async-trait`, `eventsource-stream` | **100% Reuse** (Providers) |
| **`goose-provider-types`** | Shared data structures, token tracking, thinking types | `serde`, `chrono` | **100% Reuse** |
| **`goose-mcp`** | Built-in MCP server implementations (memory, auto-visualizer, computer controller) | `rmcp`, `indoc`, `schemars` | **100% Reuse & Extend** |
| **`goose-agent`** | High-level agent configuration and builder wrappers | `goose`, `goose-providers` | **Extend** |
| **`goose-cli`** | Interactive terminal UI and command handlers | `clap`, `dialoguer`, `tokio` | **Extend** for multi-agent CLI |
| **`goose-context-management`**| Token counting, conversation compaction, and summarization | `candle-core`, `tokenizers`, `tiktoken-rs` | **100% Reuse** |
| **`goose-local-inference`** | Local model inference engine via HuggingFace Candle | `candle-core`, `candle-nn` | **100% Reuse** |
| **`goose-sdk`** | TypeScript/Rust client bindings for embedding Goose | Node/Wasm bindings | **Reuse & Extend** |
| **`goose-sdk-types`** | Custom request and response definitions across ACP boundaries | `serde`, `schemars` | **Extend** |
| **`goose-roaming`** | P2P agent roaming and telemetry relay via Iroh | `iroh`, `iroh-relay` | **Isolate / Optional** |
| **`goose-acp-macros`** | Procedural macros for ACP RPC serialization | `syn`, `quote`, `proc-macro2`| **100% Reuse** |
| **`goose-download-manager`** | Platform binary and extension asset fetcher | `reqwest`, `zip`, `sha2` | **100% Reuse** |
| **`goose-test` / `support`** | Automated testing harnesses and mock providers | `wiremock`, `tempfile` | **100% Reuse for CI** |

---

## 3. Subsystem Deep-Dive

### 3.1. Agent Execution Loop (`crates/goose/src/agents/`)
- **State Machine Architecture**: Centered around `StateMachine` in `agents/state_machine/`. It handles an explicit transition loop: `EntryHook` -> `InferenceRunner` -> `ToolApproval` -> `ToolExecution` -> `Compaction` -> `StopHook`.
- **Harness & Sandboxing**: Tools execute asynchronously with strict timeout handling (`DEFAULT_ON_FAILURE_TIMEOUT_SECONDS = 30s`).
- **Steering Queue**: Supports runtime message injection (`SteerOperation`), allowing an external supervisor or user to steer an ongoing agent turn without killing the process.
- **Suitability**: Excellent. It is fully decoupled from the UI and can be driven headless or via RPC.

### 3.2. Provider & Model Abstraction (`crates/goose-providers/`)
- **Supported Providers**: Anthropic, OpenAI, Google Gemini, Ollama (Local), OpenRouter, Azure Foundry, Databricks, Snowflake, Local Candle inference, and generic OpenAI-compatible endpoints.
- **Design**: Implements the `Provider` trait (`providers/base.rs`), normalizing streaming responses, tool call formatting, reasoning/thinking effort tags, and token usage statistics.
- **Suitability**: Perfectly matches all required model providers. No new provider code needed for MVP.

### 3.3. Subagent & Delegation System (`agents/platform_extensions/summon.rs`)
- **`summon` Platform Extension**: Implements a delegate tool allowing an agent to spawn subagents.
- **Parameters**: Allows customizing instructions, provider, model, temperature, max_turns, working_dir, extensions, and asynchronous execution (`async: true`).
- **Task Tracking**: Tracks `BackgroundTask` and `CompletedTask` with token usage and elapsed duration.
- **Suitability**: Provides the direct technical primitives required for our Manager/Orchestrator and Team delegation.

### 3.4. Multi-Agent Orchestration (`agents/platform_extensions/orchestrator.rs`)
- **Existing Capabilities**: Defines MCP tools to `list_sessions`, `view_session`, `start_agent`, `send_message`, and `interrupt_agent`.
- **`AgentManager` (`execution/manager.rs`)**: Manages an LRU pool of running `Agent` instances with thread-safe `creation_locks` and cancellation tokens.
- **Suitability**: Proves Goose was already transitioning toward multi-agent session coordination. We can build our Team Orchestrator directly on this foundation.

### 3.5. Model Context Protocol (MCP) Integration (`crates/goose/src/agents/mcp_client.rs`)
- Built on `rmcp 3.0.0`. Supports standard STDIO subprocess transport, Server-Sent Events (SSE), and streamable HTTP.
- Dynamically discovers tools, resources, and prompt templates from connected MCP servers.
- Normalizes JSON Schema definitions and manages runtime tool confirmation dialogues.

### 3.6. Permissions & Security (`crates/goose/src/permission/` & `security/`)
- **Permission Judge**: Multi-tier policy system (Prompt, Allow, Deny) with persistence via `PermissionStore`.
- **Tool Confirmation Coordinator**: Suspends execution turns until UI/user authorizes sensitive actions.
- **Security Inspectors**: Includes egress inspection, adversarial injection detection, and secret pattern scanners.

### 3.7. Persistence & Sessions (`crates/goose/src/session/session_manager.rs`)
- **Storage Engine**: SQLite database (`sessions.db`) located in OS application data directories via `sqlx`.
- **Session Schema**: Persists session ID, working directory, metadata, tokens, cost, recipes, and full chronological `Conversation` history.
- **Existing Project Primitive**: Notably, the `Session` struct already contains `pub project_id: Option<String>` and `pub parent_session_id: Option<String>`.

### 3.8. Desktop Application (`ui/desktop/`)
- **Frontend Stack**: Electron 34 + Vite + React 19 + Tailwind CSS + Radix UI + Framer Motion.
- **Architecture**: Communicates with the background `goose-server` process via the standardized Agent Client Protocol (ACP) over HTTP/SSE.
- **Components**: Rich ecosystem of modular chat bubbles, tool approval modals, extension managers, and session lists.

---

## 4. Reusability Assessment

### 4.1. Directly Reusable (Untouched)
- **`crates/goose-providers`**: Complete LLM integration layer.
- **`crates/goose/src/agents/state_machine/`**: Low-level execution, retry, and compaction loop.
- **`crates/goose/src/agents/mcp_client.rs`**: High-performance MCP client implementation.
- **`crates/goose/src/security/`**: Safety scanners, pattern matchers, and token validators.
- **`crates/goose-context-management`**: Token counting and local LLM context compaction.
- **`ui/desktop/src/components/conversation/`**: Chat UI rendering, Markdown, LaTeX, syntax highlighting.

### 4.2. Extend via Adapters & New Modules
- **`crates/goose/src/session/`**: Extend `SessionStorage` to support multi-project scoping, team associations, and task states.
- **`crates/goose-mcp/src/memory/`**: Extend the memory server from directory-scoped storage to project-level vector and relational memory.
- **`crates/goose/src/agents/platform_extensions/summon.rs`**: Wrap `summon` with a high-level `TeamOrchestrator` adapter that coordinates typed DAG workflows.
- **`ui/desktop/src/`**: Add views for Workspace Switcher, Project Navigator, Visual Agent Builder, and Visual Team Canvas.

### 4.3. Modifiable with Care (Upstream Sensitivity)
- **`crates/goose/src/execution/manager.rs`**: Add multi-agent synchronization barriers and team routing without breaking existing single-session semantics.
- **`crates/goose/src/config/`**: Add project-scoped settings schemas without invalidating global `config.yaml`.

---

## 5. Missing Capabilities (To Build)

1. **Workspace & Project Scoping**: Goose currently assumes a flat session list tied to arbitrary working directories. We must introduce explicit `Workspace` and `Project` entities.
2. **First-Class Agent Definitions**: Agents are currently ephemeral configurations of recipes + system prompts. We need persistent `AgentDefinition` entities (id, name, persona, model_config, tool_ids, permission_policy).
3. **AI Team Topologies**: No current concept of an `AITeam` bundling a manager with worker agents, inter-agent communication channels, and shared team objectives.
4. **Task DAG & Monitoring**: No structured task scheduler tracking dependency graphs, retry limits, parallel branches, and visual task cards.
5. **Visual Builders**: No UI exists in Goose Desktop for graphically configuring an agent (prompt, model, tools) or wiring a team workflow.
6. **Domain Templates**: Goose is configured primarily with programming presets. We must provide business templates (Research, Marketing, Real Estate, Restaurant Ops).

---

## 6. Upstream Maintenance Risk Matrix

| Risk Factor | Severity | Mitigation Strategy |
| :--- | :--- | :--- |
| **Upstream Core Refactoring** | Medium | Isolate platform features into separate crates (`platform-core`, `platform-teams`, `platform-ui`) and communicate with Goose via public Rust APIs and ACP. |
| **Database Schema Migrations** | High | Never alter existing Goose SQLite tables (`sessions`, `messages`). Maintain our own isolated tables (`projects`, `teams`, `agent_definitions`, `tasks`) in the database with foreign keys to `sessions.id`. |
| **ACP Protocol Changes** | Low | Goose conforms to the open Agent Client Protocol specification (`@agentclientprotocol/sdk`). Custom platform messages will be carried via namespaced custom notifications. |
| **Desktop UI Drift** | Medium | House new views in distinct React route boundaries (`/projects`, `/teams`, `/builder`) while embedding existing Goose chat components inside our project layout. |
