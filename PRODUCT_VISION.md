# Product Vision: Local AI Team Platform

## 1. Executive Summary

The **Local AI Team Platform** transforms the open-source Goose agent runtime from an individual developer's CLI/desktop coding assistant into a **local-first, general-purpose, multi-agent collaboration platform**.

Modern knowledge work rarely happens in isolation. Complex deliverables—from software development and quantitative research to marketing campaigns, legal document analysis, and hospitality operations—require specialized roles collaborating under cohesive leadership. The Local AI Team Platform provides an operating system for private, on-device AI teams where specialized agents work synchronously and asynchronously toward project goals under human and automated supervision.

---

## 2. Core Philosophy & Invariants

1. **Local-First & Sovereign**: User data, project context, agent states, logs, and sensitive credentials never leave the host machine unless explicitly routed through an authenticated external model provider chosen by the user.
2. **General-Purpose Across Business Domains**: Coding is one specialty among many. The platform ships with first-class primitives for research, creative writing, data analysis, marketing, real estate, restaurant/hospitality operations, finance, and system automation.
3. **Preserve Upstream Goose Infrastructure**: We build on top of Goose's hardened agent execution loop, MCP client, ACP protocol, and provider abstractions via adapters, isolated crates, and domain layers—enabling continuous upstream synchronization without fork divergence.
4. **Autonomous Yet Accountable**: Teams of agents can coordinate and execute tasks in parallel, but every critical action (financial mutations, destructive file operations, external communications) remains subject to strict per-agent permissions and human approval.
5. **No Synthetic or "Fake" UI**: Every control, status indicator, builder canvas, and configuration toggle is backed by operational backend state and verifiable execution.

---

## 3. Supported Domains & Use Cases

While traditional AI coding assistants focus exclusively on software development, the Local AI Team Platform provides domain templates, agent personas, and specialized toolsets across a broad spectrum:

| Domain | Primary Agents | Key Capabilities & Toolchains | Example Workflow |
| :--- | :--- | :--- | :--- |
| **Software Engineering** | Architect, Frontend Dev, Backend Dev, QA/Tester | Git, AST analysis, LSP, test runners, container CLI, browser automation | Full-stack feature implementation from issue ticket to tested PR |
| **Research & Analysis** | Principal Investigator, Literature Scout, Fact Checker, Data Analyst | arXiv/bioRxiv API, web crawlers, Python REPL, Jupyter, SQLite, Pandoc | Competitive intelligence synthesis or academic paper breakdown |
| **Writing & Publishing** | Content Strategist, Drafter, Technical Editor, Copywriter | Markdown engine, style-guide checkers, tone analyzers, CMS publisher | 5-part whitepaper development from outline to final draft |
| **Data & Financial Ops** | Financial Analyst, Data Wrangler, Visualization Engineer | Python/Pandas, SQL engines, DuckDB, financial data MCPs, Matplotlib | Quarterly P&L variance breakdown and visual board report generation |
| **Marketing & Growth** | Campaign Lead, Copywriter, SEO Specialist, Social Media Planner | Keyword engines, browser automation, image generators, analytics extractors | Multi-channel product launch campaign and collateral creation |
| **Real Estate Operations** | Property Evaluator, Market Analyst, Lease Drafter | Local document parser, geospatial GIS tools, valuation models, zoning lookups | Comparative market analysis (CMA) package from raw MLS listings |
| **Restaurant Operations** | Operations Director, Recipe/Menu Costing Lead, Inventory Auditor | Spreadsheet calculators, PDF invoice ingest, supplier catalog query | Menu margin optimization and weekly ingredient procurement plan |
| **General Automation** | System Orchestrator, Shell Operator, Scheduler | OS bash/powershell, file watchers, cron scheduler, webhook relays | Overnight backup verification, log parsing, and morning health briefs |

---

## 4. Key Product Capabilities

### 4.1. Workspace & Project Hierarchy
- **Workspaces**: Organizational boundaries representing companies, business units, or independent work areas.
- **Projects**: Dedicated mission contexts containing goals, shared document repositories, local vector/keyword memories, execution logs, and team assignments.
- **Strict Boundary Isolation**: Context, memory embeddings, and ephemeral scratch files never leak across project boundaries.

### 4.2. Multi-Agent AI Teams
- **Role Specialization**: Every agent has a distinct persona, system prompt, skill bundle, and capability profile.
- **Heterogeneous Models & Providers**: Each agent within a single team can run on a different model/provider:
  - OpenAI (GPT-4o, o3-mini, reasoning models)
  - Anthropic (Claude 3.7 Sonnet, Claude 3.5 Haiku, Opus)
  - Google Gemini (Gemini 2.5 Pro, Flash)
  - Local Models via Ollama (Llama 3.3, DeepSeek R1, Qwen 2.5)
  - Existing Goose providers (OpenRouter, Azure, Databricks, Bedrock)
- **Granular Permissions & Tools**: Agent A (Researcher) can have read-only web search and PDF parsing tools, while Agent B (Writer) has local file editing, and Agent C (Deployer) has shell execution gated by explicit confirmation.

### 4.3. Orchestration & Delegation
- **Manager/Orchestrator Agent**: A meta-agent responsible for breaking high-level user directives into structured Directed Acyclic Graphs (DAGs) of tasks.
- **Intelligent Delegation**: Dynamic assignment of subtasks to specialized agents based on capability matching and cost/latency constraints.
- **Parallel Execution**: Safe concurrent execution of independent tasks across multiple agents with synchronization barriers.

### 4.4. Memory Architecture
- **Agent Working Memory**: Ephemeral, turn-level scratchpad and context compaction.
- **Agent Persona Memory**: Long-term identity, stylistic preferences, and learned operational heuristics.
- **Project Semantic Memory**: Persistent vector and keyword store indexing project documentation, historical outputs, codebases, and domain files.
- **Audit Trails**: Complete chronological message replay and execution history stored in local SQLite.

### 4.5. Visual Builders
- **Visual Agent Builder**: Interactive interface to construct, configure, test, and tune individual agents (prompts, model parameters, MCP servers, tools, permission policies).
- **Visual Team Builder**: Canvas-based composition of multi-agent topologies (Manager-Worker, Peer Review, Assembly Line, Swarm) with visual wiring of inputs, outputs, and review gates.

### 4.6. Extensibility & Ecosystem
- **Model Context Protocol (MCP)**: Native support for all MCP servers (STDIO, SSE, Streamable HTTP).
- **Skill Bundles & YAML Recipes**: Declarative packaging of instructions, scripts, and MCP references for one-click sharing and versioning.
- **Dual Form Factor**:
  - **Desktop Application**: Modern Electron + React + Tailwind workspace for visual workflows, canvas orchestration, and interactive review.
  - **Headless & Interactive CLI**: High-throughput terminal interface for server automation, scripts, and developer pipelines.

---

## 5. Non-Goals & Boundaries

To preserve architectural integrity and guarantee execution feasibility, the platform explicitly defines the following boundaries:

- **Not a Cloud SaaS Platform**: We do not build multi-tenant cloud hosting, user billing gateways, or central web portals. The platform is locally hosted and user-controlled.
- **Not a Black-Box Fully Autonomous Loop**: The platform never removes the human-in-the-loop for destructive or irreversible actions. Safety and permission boundaries are hard constraints.
- **Not a Monolithic Code Assistant**: While coding workflows are world-class, core concepts are deliberately decoupled from software-only concepts (e.g., git commits or syntax trees are extensions, not core platform primitives).
- **Not a Goose Rewrite**: We preserve Goose's core agent engine, ACP implementation, and MCP client as upstream dependencies, writing adapters and higher-order management crates around them.
