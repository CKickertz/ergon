# Spec 45: Coworking Workspace -- Shared Operations Surface for Human + Agents

**Status:** Draft
**Author:** Syn
**Date:** 2026-03-01
**Spec:** 45

---

## Problem

The human-agent interaction model is serial messaging through a slot in a wall. Cody sends a message, waits, reads a response. No shared visible state. No mutual awareness. No ability to see what agents are thinking about, what's healthy, what's burning money, or what needs attention -- without explicitly asking.

This creates three concrete failures:

1. **Context reconstruction tax.** Every session starts with "what's the current state of X?" because neither party can see the other's workspace. The human reconstructs agent state from memory. The agent reconstructs human intent from messages.

2. **Invisible operations.** Prosoche is scoring signals. Daemon crons are running. Phases are executing. Cost is accumulating. Sessions are active. None of this is visible unless you ask or grep logs. Failures surface as symptoms ("why did that take so long?"), not as live indicators.

3. **No shared work surface.** When Cody adds a task, he types it in Signal. When an agent finishes work, it reports in chat. There is no single surface where both parties see pending work, check items off, and share awareness of what is done and what remains.

Signal is a communication *method* -- a pipe. What is missing is a communication *manner* -- a shared room where human and agents cowork with mutual visibility.

---

## Vision

A web-based operations workspace served by Pylon that replaces the current chat-only WebUI as the primary coworking surface. Signal remains the mobile/quick channel. The workspace is the deep-work channel.

The workspace is **bidirectional**. The human sees agent state. Agents see human decisions. Toggles take effect immediately. Tasks sync in both directions. Health is always visible. Cost is always visible. The room is always lit.

This is one human's personal operations center and coworking space -- not multi-tenant, not multi-user. One deployment, one operator.

### What This Is Not

- Not a kanban board (Spec 30's task board is a component, not the product)
- Not a monitoring-only dashboard (read-write, not read-only)
- Not a replacement for Signal (complement -- different interaction mode)
- Not a replacement for A2UI canvas (canvas is agent-writable surfaces within the workspace)

### Relationship to Existing Specs

| Spec | Relationship |
|------|-------------|
| 29 (UI Layout) | Workspace supersedes current layout. Agent bar, theme, responsive design carry forward. |
| 30 (Homepage Dashboard) | Absorbed. Task board and activity feed become workspace views. |
| 43b (A2UI Canvas) | Integrated. Canvas surfaces render within the workspace as agent-writable panels. |

---

## Design

### Principles

1. **Unconcealment over reporting.** State is always visible, not fetched on demand. If prosoche is scoring, you see it. If a phase is blocked, you see it. The workspace practices aletheia.

2. **Bidirectional by default.** Every display is also a control. Agent status is visible AND the model powering it is switchable. Tasks are visible AND checkable by either party. Health is visible AND services are restartable.

3. **Immediacy.** Toggles take effect now. No confirmation dialogs, no "are you sure?" The human is the operator. The workspace is a cockpit, not a wizard.

4. **Complementary to Signal.** Signal is the phone in your pocket. The workspace is the desk you sit at. Different postures, same system. A message sent via Signal appears in the workspace. A task created in the workspace is visible to agents in their next turn.

5. **Incremental assembly.** The workspace is a collection of views and panels, not a monolith. Each view can ship independently. The frame (navigation, SSE connection, auth) ships first, views fill in.

### Architecture

```
Browser                          Pylon (Axum)
┌─────────────────┐              ┌─────────────────────────┐
│                  │   SSE        │                         │
│  Workspace UI    │◄────────────│  /ws/events (SSE)       │
│  (Svelte 5)      │              │                         │
│                  │   REST       │  /api/workspace/*       │
│                  │────────────►│    tasks, toggles,      │
│                  │              │    health, config       │
│                  │              │                         │
│  Canvas panels   │◄────────────│  /api/canvas/* (Spec 43b)│
│                  │              │                         │
└─────────────────┘              └──────────┬──────────────┘
                                            │
                              ┌─────────────┼──────────────┐
                              │             │              │
                         NousActor     Prosoche       Daemon
                         sessions      signals        crons
                         turns         scores         evolution
                         tools         attention      distillation
```

**SSE as the spine.** A single SSE connection carries all live state: agent status changes, task updates, health heartbeats, cost ticks, phase transitions, canvas surface updates. The workspace subscribes once and routes events to the appropriate view.

**REST for mutations.** Toggle a model, create a task, restart a service, change a config value. POST/PATCH endpoints that take effect immediately and emit SSE events confirming the change.

**Pylon serves everything.** Static assets (Svelte build) + API routes + SSE stream. No separate dashboard server. The workspace IS the WebUI -- same origin, same auth (symbolon), same port.

---

## Views

### 1. Home -- The Room

The default view. What you see when you sit down at the desk.

```
┌──────────────────────────────────────────────────────────────┐
│  Aletheia  ●Syn Idle  ◉Demi Working  ●Syl Idle  ●Akron Idle │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Evening, Cody.                            Mar 1, 8:30 PM   │
│                                                              │
│  ┌─ Tasks ──────────────────────────────────────────────┐   │
│  │  ☐ Review pylon auth tests              → Syn        │   │
│  │  ☐ Respond to James re Q3               → Cody       │   │
│  │  ☐ Research dashboard projects           → Syn    ✓   │   │
│  │  ☑ Merge PRs #387-389                   → Syn        │   │
│  │  [+ Add task]                                        │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─ Active Now ────────────────────────────────────────┐    │
│  │  Demi: Executing Phase 2 "Infrastructure" (3/7)     │    │
│  │  Prosoche: 3 signals scored, next cycle in 4m       │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌─ System ─────────────┬─ Cost (7d) ─────────────────┐    │
│  │  Gateway: ● healthy  │  Sonnet: $8.42    ▁▃▂▅▃▁▂   │    │
│  │  Signal:  ● healthy  │  Opus:   $4.10    ▁▁▃▁▁▁▅   │    │
│  │  Prosoche: ● active  │  Embed:  $0.00               │    │
│  │  CozoDB:  ● healthy  │  Total:  $12.52              │    │
│  └──────────────────────┴──────────────────────────────┘    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Components:**
- **Task board** (from Spec 30, refined). Shared between human and agents. Either party adds, either checks off. Backed by SQLite via pylon API. Agents see their tasks injected into context.
- **Active now**. What is happening right this moment. Drawn from SSE events: active turns, running phases, prosoche cycles, daemon crons.
- **System health**. Heartbeat tiles for every subsystem. Green/yellow/red. Click to drill.
- **Cost summary**. 7-day rolling cost by model. Sparkline trends. Updated per-turn.

### 2. Agent Detail

Click an agent pill to see their world.

```
┌──────────────────────────────────────────────────────────────┐
│  ← Home    Syn                                     [Chat ↗]  │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Status: Idle          Model: [Sonnet 4 ▾]                   │
│  Last active: 2m ago   Sessions today: 7                     │
│  Tokens (24h): 48.2K   Cost (24h): $1.84                    │
│                                                              │
│  ┌─ Controls ──────────────────────────────────────────┐    │
│  │  Model:       [Sonnet 4 ▾]  (dropdown, immediate)  │    │
│  │  Autonomous:  [●━━━━○]  OFF                         │    │
│  │  Wake on signal: [━━━━●○]  ON                       │    │
│  │  Prosoche:    [━━━━●○]  ON   (cycle: 5m)            │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌─ Recent Sessions ──────────────────────────────────┐     │
│  │  #s_01J8... "PR merge and docs update"   42 turns   │     │
│  │  #s_01J7... "Prosoche dedup fix"         18 turns   │     │
│  │  #s_01J6... "Profile README"              8 turns   │     │
│  │  [View session replay →]                            │     │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌─ Attention (Prosoche) ─────────────────────────────┐     │
│  │  Signal: "3 overdue work tasks" — score: 0.72       │     │
│  │  Signal: "PR #385 open 2d" — score: 0.45            │     │
│  │  Signal: "Memory approaching 80%" — score: 0.31     │     │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

**Controls that write to taxis config (hot-reloaded via arc-swap):**
- **Model selector.** Dropdown: Sonnet / Opus / Haiku. Writes to agent's model config. Takes effect on next turn.
- **Autonomous mode toggle.** When ON, agent picks up open tasks and prosoche signals without being prompted. Writes to autonomy gradient config (Spec 39).
- **Wake on signal.** Whether prosoche wakes this agent. Toggle writes to daemon config.
- **Prosoche cycle.** Interval slider or preset. Writes to daemon cron config.

**Session replay** (inspired by AgentOps): Click a session to see a timeline of every turn, tool call, and decision. Scrub through it like a video. Data already exists in session store -- this is a read-only visualization.

**Prosoche attention panel.** Live view of what this agent's prosoche is scoring. Directly surfacing the signal pipeline that is currently invisible.

### 3. Projects + Phases

Dianoia execution state visualized.

```
┌──────────────────────────────────────────────────────────────┐
│  ← Home    Projects                                          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  Aletheia Issue Hackathon                                    │
│  State: executing                                            │
│                                                              │
│  ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐    │
│  │Quick Win│──►│Infra &  │──►│CLI      │──►│A2UI     │    │
│  │  ✅     │   │Reliab.  │   │Found.   │   │Canvas   │    │
│  │ done    │   │ ◉ exec  │   │ ○ queue │   │ ○ queue │    │
│  └─────────┘   └────┬────┘   └─────────┘   └─────────┘    │
│                      │                                       │
│               ┌──────▼──────┐                                │
│               │ 3/7 plans   │                                │
│               │ ██████░░░░  │                                │
│               │             │                                │
│               │ ✅ lint fix │                                │
│               │ ✅ CI yaml  │                                │
│               │ ◉ test cov  │ ← Demi working                │
│               │ ○ error fmt │                                │
│               │ ○ dead code │                                │
│               │ ○ dep audit │                                │
│               │ ○ docs sync │                                │
│               └─────────────┘                                │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

Phase dependency graph. Click a phase to expand its plans. Click a plan to see the agent executing it. Verification status shown per phase. Checkpoint approvals can be given directly from this view.

### 4. Health + Services

Full system health board. Expansion of the home view's summary tiles.

```
┌──────────────────────────────────────────────────────────────┐
│  ← Home    System Health                                     │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─ Pylon Gateway ───────────────────────── ● Healthy ──┐   │
│  │  Port: 18789  │  Uptime: 14d 3h  │  Req/min: 12     │   │
│  │  Active SSE connections: 2                            │   │
│  │  [View logs]                                          │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─ Signal Channel ──────────────────────── ● Healthy ──┐   │
│  │  signal-cli PID: 48291  │  Last msg: 3m ago          │   │
│  │  Pending sends: 0  │  Failed (24h): 0                │   │
│  │  [View logs]  [Restart]                               │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─ Prosoche ────────────────────────────── ● Active ───┐   │
│  │  Cycle: 5m  │  Last run: 2m ago  │  Signals: 7       │   │
│  │  Wake budget: 1/2 used (1h window)                    │   │
│  │  Dedup window: 8h  │  Fingerprints: 3                 │   │
│  │  [View signals]  [Force cycle]  [Clear fingerprints]  │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─ CozoDB ─────────────────────────────── ● Healthy ───┐   │
│  │  Relations: 42  │  Rows: 18.4K  │  Size: 28MB        │   │
│  │  Last compact: 6h ago  │  HNSW indices: 3             │   │
│  │  [Compact now]  [View stats]                          │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─ Daemon Crons ────────────────────────── ● Active ───┐   │
│  │  Syn:  distill 6h ago ✓  │  evolve 23h ago ✓         │   │
│  │  Demi: distill 2h ago ✓  │  evolve 12h ago ✓         │   │
│  │  Syl:  distill 1d ago ✓  │  evolve 3d ago ⚠          │   │
│  │  [Run now ▾]                                          │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

Every tile is a subsystem. Each tile shows key metrics, last-known state, and action buttons. Actions hit pylon REST endpoints that talk to the relevant actor.

### 5. Cost Cockpit

Dedicated view for token economics.

- **Burn rate** per agent, per model, per phase, per day. Line charts.
- **Model comparison.** Same task class, Sonnet vs Opus cost and quality. Helps inform the model toggle decision with data.
- **Projections.** "At current rate, this month will cost $X."
- **Budget alerts.** Optional. Set a daily/weekly ceiling. Visual warning when approaching.
- **Per-session cost.** How much did that conversation cost? Drill into any session.

### 6. Session Replay

The AgentOps-inspired feature. A timeline view of a completed (or active) session.

```
Session #s_01J8K... "PR merge and docs update"
Agent: Syn  │  42 turns  │  Duration: 1h 14m  │  Cost: $3.22

Timeline:
├─ 0:00  System prompt assembled (12.4K tokens)
├─ 0:02  User: "Let's merge those PRs"
├─ 0:04  Tool: exec("gh pr list") → 3 open PRs
├─ 0:06  Tool: exec("gh pr view 387") → reviews docs
├─ 0:08  Thinking: "Need to check merge conflicts..."
├─ 0:12  Tool: exec("gh api .../merge") → merged #387
├─ ...
├─ 1:12  Response: "All three merged, branches cleaned up"
└─ 1:14  Session end (stop_reason: end_turn)
```

Scrub the timeline. Expand any turn to see full request/response. See thinking blocks. See tool inputs and outputs. See token counts per turn. This is the "video replay" of agent work.

### 7. Chat

The existing chat view, preserved. Click "Chat" from an agent detail page or from the agent bar. Full conversation interface. This is what Signal does in web form -- direct messaging with an agent.

### 8. Canvas (Spec 43b)

Agent-writable dynamic surfaces that appear as panels within any view. A Dianoia phase can push a progress surface. An agent can push a metrics surface. Canvas surfaces render inline or in a collapsible side panel.

---

## Wishlist -- Things We Want Even If Hard

These are ideas that may not make v1 but belong in the design space. They represent the ceiling of what this system could be. Organized roughly by how deeply they change the interaction model.

### Interaction Model

#### Shared Cursor / Presence

When Cody is looking at the Projects view, agents can see "operator is viewing Phase 2." When an agent is actively executing, the workspace shows which file or tool the agent is touching. Mutual awareness of attention. Inspiration: multiplayer cursors in Figma, but asymmetric -- one human, many agents.

#### Voice Notes

Press-and-hold to record a voice note from the workspace. Transcribed and delivered to the agent as a message. The agent can respond with synthesized voice. Low-friction input when typing is too slow. We already have voice_reply in the agent toolkit -- this is the UI counterpart.

#### Command Palette

Cmd+K / Ctrl+K universal command palette. "Switch Syn to Opus." "Show health." "Create task: review PR #390." "Pause Demi's phase." Type a natural language command, the workspace interprets and executes. Blurs the line between UI navigation and agent instruction.

#### Quick Capture

Floating button or keyboard shortcut to capture a thought without context-switching views. "Note: check if CozoDB compaction is scheduled" -- drops into an inbox that prosoche can score and route. Inspired by Things/Todoist quick-add.

### Observability

#### Context Window Visualizer

Show the token budget breakdown for any agent's current or recent turn. System prompt: 12K tokens. History: 8K. Tools: 2K. Available for response: 78K. Stacked bar chart per turn. Helps diagnose "why did the agent forget X?" -- because it was pushed out of the window. Inspired by Langfuse's trace detail view.

#### Memory Graph Explorer

Interactive visualization of an agent's CozoDB knowledge graph. Nodes = entities, edges = relationships. Click a node to see its facts, provenance, and temporal history. Filter by entity type, time range, or confidence. Uses the graph surface type from Spec 43b canvas.

#### Prompt Playground

Select an agent, see its current system prompt fully assembled (SOUL + TELOS + MNEME + tools + prosoche + tasks). Edit it live. Run a test turn against it. See the response. Iterate. Based on Langfuse's playground concept but tightly integrated with our assembly pipeline.

#### Log Stream

Tail logs from any subsystem in real-time within the workspace. Filter by crate, log level, agent. Inspired by the Clawd Control activity feed, but with structured tracing spans (from our Spec 41 observability work). Click a span to expand its children.

#### Infinite Loop Detection

Visual indicator when an agent is stuck in a loop -- editing the same file repeatedly, retrying the same tool call, or generating similar outputs across turns. Ralph TUI's core value proposition. The workspace should surface this pattern automatically and offer a "break loop" button. We already track tool calls per session.

### Control

#### Tiered Model Strategy

Not just per-agent model selection, but per-task-type model routing. "Use Haiku for lint fixes, Sonnet for feature work, Opus for architecture decisions." Configure tiers in the workspace, enforced by hermeneus model routing. Inspired by Ralph TUI's tiered cost strategy.

#### Autonomous Run Limits

When autonomous mode is ON, set iteration limits: "Work on up to 5 tasks, then stop and report." Prevents runaway autonomous sessions. Visible in agent detail as a progress bar (3/5 tasks completed this cycle).

#### Scheduled Actions

"At 6am tomorrow, have Demi start the test coverage expansion." "Every Monday, run a full health check and report." Cron-like scheduling from the UI that writes to daemon config. Visual calendar or timeline of upcoming scheduled actions.

#### Bulk Operations

Select multiple agents, apply the same model change. Select multiple tasks, reassign to a different agent. Multi-select with shift-click, batch actions toolbar. Important as the team grows beyond 4 agents.

### Visualization

#### Ambient Mode

A view designed to stay open on a secondary monitor. Minimal chrome, large type, auto-rotating between: active agent status, task board, cost ticker, phase progress. The workspace as a passive awareness surface -- glanceable, not interactive. Think airport departure board aesthetic.

#### Activity Heatmap

GitHub-contribution-style heatmap showing agent activity over time. Color intensity = number of turns/tasks completed. Hover for details. Spans days/weeks/months. Shows patterns: "Demi is most active on Mondays," "Syn had a burst Wednesday evening."

#### Dependency Graph View

Not just Dianoia phases, but a broader view of how everything connects: specs reference issues, issues reference code, code references tests, agents own domains. A navigable graph of the entire project topology. Click any node to drill into it.

### Data

#### Replay Diffing

Compare two session replays side by side. Same task, different models. Same agent, before and after a prompt change. See where behavior diverged. Useful for evaluating model switches and prompt engineering.

#### Agent Journaling

A read-only view of an agent's memory files, session notes, and MNEME entries over time. See how the agent's understanding of the world has evolved. Timeline scrubbing. The unconcealment of the agent's inner life.

#### Evaluation Snapshots

Periodically snapshot agent performance on standard tasks. Track quality over time as prompts, models, and memory evolve. Inspired by Langfuse's evaluation datasets. "Is Syn getting better at PR reviews?"

#### Export / Reporting

Generate a PDF or markdown summary of work completed this week/month. Tasks done, cost incurred, phases advanced, sessions run. Useful for Cody's own accountability and for sharing with collaborators who don't have workspace access.

### Platform

#### Mobile Companion

Not a full mobile app -- a responsive web view optimized for phone. Check tasks, see health, toggle a model. Quick actions from the couch. The full workspace stays on the desktop.

#### Notification Bridge

Workspace events (task completed, phase failed, health degraded) can optionally push to Signal or system notifications. The workspace doesn't require you to be watching it -- it reaches out when something needs attention.

#### Plugin Marketplace View

If prostheke (WASM plugins) matures, a view for browsing, installing, and configuring plugins. Each plugin shows its granted capabilities, resource usage, and health.

#### Drag-and-Drop Workspace Customization

Homarr-style widget placement. Instead of fixed view layouts, let the operator rearrange panels, resize tiles, pin favorites. Save layouts per user preference. Heavy lift but dramatically increases personalization.

---

## Implementation Strategy

### Phase 0: Frame

The skeleton that everything hangs on.

- Pylon serves static Svelte build at `/` (already does this)
- Single SSE endpoint `/ws/events` with typed event discriminator
- Symbolon auth for all workspace routes
- Navigation shell: Home, Agents, Projects, Health, Cost, Chat
- Agent bar (from Spec 29) as persistent top element

### Phase 1: Home + Tasks

The "sit down at the desk" experience.

- Task CRUD API (`/api/tasks`)
- Task store (SQLite, system-level -- not per-session)
- Home view: greeting, task board, active-now panel, system summary, cost summary
- Agent tools: `task_list`, `task_create`, `task_complete`
- SSE events: `task:created`, `task:updated`, `task:completed`

### Phase 2: Agent Detail + Controls

See and control each agent.

- Agent detail view with status, metrics, recent sessions
- Config toggles that write to taxis (model, autonomy, wake, prosoche)
- Hot-reload via arc-swap so toggles take effect without restart
- Prosoche attention panel (read-only view of scored signals)

### Phase 3: Health Board

Full system visibility.

- Health check endpoints for each subsystem
- Heartbeat protocol (each actor reports periodically)
- Health view with tiles, status indicators, action buttons
- Service actions: restart, force-cycle, compact, clear cache

### Phase 4: Projects + Phases

Dianoia visualization.

- Phase dependency graph renderer (SVG or Canvas)
- Plan-level detail with agent assignment and status
- Checkpoint approval from the UI
- Verification gap display

### Phase 5: Cost Cockpit

Token economics.

- Per-turn cost recording (already in session store)
- Aggregation queries by agent, model, time window
- Cost view with charts, projections, model comparison
- Budget alert configuration

### Phase 6: Session Replay

The crown jewel.

- Session timeline data structure (turns, tools, thinking, tokens)
- Timeline renderer with scrubbing
- Turn detail expansion (full request/response)
- Active session live-follow mode

### Phase 7: Canvas Integration

Spec 43b brought into the workspace.

- Canvas panel (collapsible sidebar or inline)
- Surface type renderers (progress, table, metrics, markdown, graph)
- Agent tool: `canvas_update`

---

## Open Questions

1. **Task backend: SQLite or CozoDB?** Tasks are simple CRUD. SQLite is proven (sessions already use it). CozoDB could unify storage but adds complexity for a flat table. Leaning SQLite.

2. **SSE event schema.** One mega-stream with typed discriminators, or multiple SSE endpoints per view? Single stream is simpler for the client. Multiple streams allow partial subscription. Need to decide.

3. **Config hot-reload granularity.** Can we reload a single agent's model without touching other config? arc-swap replaces the whole config atomically. May need per-agent config segments.

4. **Session replay storage.** Full turn data (including thinking blocks and tool I/O) is large. Store everything? Trim after N days? Separate replay store from session store?

5. **How does autonomous mode work?** When toggled ON, does the agent immediately scan for work? Or wait for next prosoche cycle? What triggers the first autonomous turn?

6. **Canvas panel placement.** Right sidebar (like VS Code panels), bottom panel (like a terminal), or floating/dockable? Probably right sidebar with collapse toggle.

7. **What is the gnomon name for this?** The workspace where truth is unconcealed. Where human and agents share a room. The seeing-place. Contenders: theatron (seeing-place), synoptikon (seeing-together), synergeion (working-together-place). Or something we have not yet discovered.

---

## Non-Goals

- Multi-user / multi-tenant (this is personal infrastructure)
- Mobile app (web-responsive is sufficient for v1)
- Replacing Signal (complementary, not competitive)
- Real-time collaborative editing (not Google Docs for agents)
- Custom dashboard builder / widget framework (views are designed, not assembled)

---

## References

### Internal

- [Spec 29 -- UI Layout & Theming](29_ui-layout-and-theming.md)
- [Spec 30 -- Homepage Dashboard](30_homepage-dashboard.md)
- [Spec 41 -- Observability](41_observability.md)
- [Spec 43b -- A2UI Live Canvas](43_a2ui-canvas.md)
- [Spec 39 -- Autonomy Gradient](archive/DECISIONS.md) (absorbed)

### External -- Agent Dashboards + Ops

- [AgentOps](https://github.com/AgentOps-AI/agentops) (5.3K stars, MIT) -- Session replay timelines, cost tracking, agent execution graphs. Open source app in `/app` directory. Best reference for session replay UI.
- [Clawd Control](https://github.com/Temaki-AI/clawd-control) (111 stars, MIT) -- SSE live updates, fleet health tiles, agent detail drilldowns, auto-discovery. Vanilla HTML/JS + Node. Zero build step. Best reference for simple, effective health monitoring.
- [mudrii/openclaw-dashboard](https://github.com/mudrii/openclaw-dashboard) (136 stars, MIT) -- Python + pure HTML/SVG. 11 panels: cost trends, session tracking, sub-agent hierarchy trees, cron status. Zero external dependencies.
- [Lattice Workbench](https://latticeruntime.com/) (Apache 2.0) -- Agent IDE + operations console. Multi-model chat, real-time monitoring, cost tracking. Desktop/web/CLI/VS Code. Enforcement primitives (identity, auth, audit, constraints).
- [Ralph TUI](https://www.verdent.ai/guides/ralph-tui-ai-agent-dashboard) -- Terminal-based agent mission control. Key ideas: iteration limits for autonomous runs, infinite loop detection, session state persistence to disk, tiered model strategy (expensive for architecture, cheap for lint). The "if you can't see what your AI is doing, you shouldn't let it touch your codebase" philosophy.

### External -- LLM Engineering

- [Langfuse](https://github.com/langfuse/langfuse) (22.5K stars, open source) -- LLM observability, trace visualization, prompt management + playground, evaluation datasets, cost metrics. Best reference for trace detail views and prompt iteration workflow.
- [Homarr](https://homarr.dev/) -- Self-hosted server dashboard with drag-and-drop widget placement. Reference for customizable layout patterns.
