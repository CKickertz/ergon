# Theatron Research Notes -- Implementation Readiness Audit

**Date:** 2026-03-01
**Purpose:** Identify what needs to be decided or built before Spec 45 can be turned into dispatch-ready prompts at M6.

---

## 1. Existing UI Inventory

The current `ui/` directory is a Svelte 5 app with substantial infrastructure already in place. This is NOT a greenfield build.

### What Already Exists

| Component | Files | Status | Theatron Relevance |
|-----------|-------|--------|-------------------|
| SSE event system | `lib/events.svelte.ts` | Production | **Core reuse.** Already handles reconnection, heartbeat, typed events (turn, tool, status, planning, task). Theatron's "SSE as the spine" is already built. |
| Agent store | `stores/agents.svelte.ts` | Production | Direct reuse. Identity cache, emoji, active agent tracking. |
| Task store | `stores/tasks.svelte.ts` | Production | Direct reuse. Full CRUD, optimistic updates, SSE subscription. |
| Chat view | `components/chat/*` (18 files) | Production | Becomes Chat view widget. Already has streaming, tool calls, thinking panels, planning cards, markdown. |
| Planning view | `components/planning/*` (15 files) | Production | Becomes Projects view widget. Has execution status, roadmap, retrospective, checkpoints, discussion, annotations. |
| Metrics view | `components/metrics/*` | Production | Becomes Health/Cost widget. Has uptime, tokens, cache rates, cron status, service health. |
| Cost dashboard | `CostDashboard.svelte` | Production | Becomes Cost widget. Has per-agent cost, token breakdown. |
| Graph view | `components/graph/*` (7 files) | Production | Becomes Memory Graph Explorer widget. Has 2D/3D force graphs, node cards, timeline slider, drift panel, health bar. |
| File editor | `components/files/*` (4 files) | Production | Becomes code/file widget. Has tree explorer, CodeMirror editor tabs. |
| Settings view | `components/settings/*` | Production | Partial reuse for agent controls. |
| Auth/onboarding | `components/auth/*`, `components/onboarding/*` | Production | Reuse as-is. Setup wizard, login flow. |
| Layout | `components/layout/*` | Production | **Replace.** Current layout is chat-centric with tab switching. Theatron needs grid-based view composition. |
| Shared components | `components/shared/*` | Production | Reuse: Badge, ErrorBanner, Spinner, Toast. |
| Type system | `lib/types.ts` | Production | Extensive types for agents, sessions, messages, tools, plans, metrics, graphs, costs, threads. Foundation for widget data contracts. |
| Utility libs | `lib/api.ts`, `lib/format.ts`, `lib/auth.ts`, `lib/stream.ts` | Production | Direct reuse. |

### Key Patterns Established

- **Svelte 5 runes** (`$state`, `$effect`) for all reactive state -- NOT legacy stores.
- **SvelteMap/SvelteSet** from `svelte/reactivity` for collections.
- **SSE dispatch model:** `onGlobalEvent(cb)` subscribes, components filter by event type.
- **Optimistic mutations** with rollback on error (see task store).
- **Auth:** Token-based with httpOnly cookie session refresh.
- **No component library.** All custom CSS, CSS variables for theming (`--text-muted`, `--border`, `--accent`, etc.).
- **No chart library.** `UsageChart.svelte` likely uses SVG or Canvas directly. `force-graph` and `3d-force-graph` for graph viz.
- **CodeMirror 6** for code editing.
- **Vite 7** + **vitest 4** for build/test.

### What Doesn't Exist Yet

| Need | Status | Notes |
|------|--------|-------|
| Widget registry / dynamic component loading | Not built | Core Theatron primitive. Need a way to map widget type strings to Svelte components. |
| Grid layout engine | Not built | Current layout is flexbox with manual panes. Need CSS Grid or library. |
| View definition parser | Not built | YAML/JSON → widget tree → rendered grid. |
| View CRUD API (Pylon) | Not built | `/api/theatron/views` endpoints. |
| View store (Svelte) | Not built | Client-side view management, persistence, hot-reload. |
| Nous-scoped data filtering | Partially exists | Agent store has `activeAgentId` but widgets don't auto-filter. |
| Drag-and-drop | Not built | Widget rearrangement, resize handles. |
| Session replay timeline | Not built | Chat view shows live streaming but no historical timeline scrubbing. |
| Context window visualizer | Not built | Token counts exist in `TurnOutcome` but no visualization. |
| Cost time series | Not built | Only aggregate totals. No sparklines, no daily/weekly trends. |
| Prosoche signal display | Not built | No prosoche integration in UI. |
| Daemon cron control | Not built | Metrics view shows cron status read-only. No toggle/trigger. |

---

## 2. Technology Decisions Needed

These decisions should be made NOW so prompts can be written with specificity.

### 2a. Grid Layout Approach

**Options:**

1. **CSS Grid native.** Use CSS `display: grid` with `grid-template-columns: repeat(12, 1fr)`. Widgets placed via `grid-column` and `grid-row` from view definitions. Drag-and-drop via pointer events + coordinate math.
   - Pro: Zero dependencies, fast, matches existing pattern of no external UI libs.
   - Con: Drag-and-drop requires significant custom code (grid snapping, placeholder rendering, collision detection).

2. **gridstack.js** (~10K gzipped). Battle-tested draggable/resizable grid library. Used by Grafana, Homarr.
   - Pro: Drag, resize, responsive breakpoints, serialization all built in.
   - Con: jQuery-era API (though modern version is vanilla). May fight Svelte reactivity.

3. **svelte-grid** (Svelte-native gridstack alternative).
   - Pro: Svelte-native, lighter.
   - Con: Less mature, fewer features, smaller community.

**Recommendation:** CSS Grid native for Phase 0-1 (layout rendering from definitions). Add drag-and-drop (possibly gridstack.js or custom) in Phase 7. Don't couple the layout engine to a drag library from the start -- most users will customize view definitions directly before they drag widgets.

### 2b. Chart Library

**Options:**

1. **Custom SVG.** Hand-rolled sparklines, bar charts. Matches no-dependency pattern.
   - Pro: Tiny, fast, no bundle bloat, full control.
   - Con: Significant effort for complex charts (heatmaps, stacked areas, interactive timelines).

2. **Chart.js** (~60K gzipped). Ubiquitous.
   - Pro: Every chart type, well-documented.
   - Con: Canvas-based (harder to style with CSS), bundle size.

3. **uPlot** (~8K gzipped). Fast, lightweight time-series charts.
   - Pro: Tiny, fast, designed for time-series (which is most of what Theatron needs).
   - Con: Narrow focus (time-series only), Canvas-based.

4. **Layercake** -- Svelte-native chart framework. Generates SVG/Canvas/HTML layers.
   - Pro: Svelte-native, composable, SVG (CSS-stylable), lightweight framework.
   - Con: You still write chart logic, it's a scaffold not a library.

**Recommendation:** Custom SVG for simple widgets (sparklines, status indicators, progress bars). Evaluate uPlot or Layercake for time-series views (cost trends, activity heatmaps, session timelines). Decide at Phase 5 (Cost Cockpit) when we know the actual chart requirements.

### 2c. Widget Registry Pattern

**Options:**

1. **Static import map.** A TypeScript object mapping widget type strings to Svelte component imports.

```typescript
const WIDGET_REGISTRY: Record<string, () => Promise<typeof import('*.svelte')>> = {
  'task-list': () => import('./widgets/TaskList.svelte'),
  'health-tile': () => import('./widgets/HealthTile.svelte'),
  'agent-status': () => import('./widgets/AgentStatus.svelte'),
  // ...
};
```

   - Pro: Type-safe, tree-shakeable (unused widgets don't ship), simple.
   - Con: New widget types require code changes + rebuild.

2. **Dynamic plugin loading.** Widgets loaded at runtime from a URL or WASM module.
   - Pro: Extensible without rebuild.
   - Con: Massive complexity, security concerns, not needed until prostheke (M5).

**Recommendation:** Static import map for v1. Strongly typed, lazy-loaded via dynamic import. When prostheke ships, add a `PluginWidget` wrapper that loads WASM-backed widgets through the plugin API.

### 2d. View Definition Storage

**Options:**

1. **Taxis YAML files.** View definitions live in oikos config files. Hot-reload via arc-swap.
   - Pro: Consistent with all other config. Version-controllable.
   - Con: File I/O for every save. No query capability.

2. **SQLite table.** `theatron_views` table with JSON column for layout.
   - Pro: CRUD is natural. Query by scope, author, etc.
   - Con: Another storage backend to manage.

3. **Both.** Built-in defaults from YAML (shipped with binary). User/agent-created views in SQLite.
   - Pro: Best of both. Defaults are code-reviewed. Custom views are dynamic.
   - Con: Two sources to reconcile.

**Recommendation:** Option 3. Built-in defaults compiled into the binary from YAML templates. Runtime views (user-created, agent-authored) stored in SQLite alongside sessions. Pylon serves merged results: built-ins + custom, with custom overriding built-in if same name.

---

## 3. Pylon Route Gaps

Current Pylon routes (Rust):

```
GET  /api/health
POST /api/sessions
GET  /api/sessions/{id}
DEL  /api/sessions/{id}
POST /api/sessions/{id}/messages
GET  /api/sessions/{id}/history
GET  /api/nous
GET  /api/nous/{id}
GET  /api/nous/{id}/tools
```

Current TypeScript runtime also serves (not yet ported to Rust):

```
GET  /api/events               (SSE stream -- global event bus)
GET  /api/metrics              (system metrics)
GET  /api/credentials          (credential info)
GET  /api/agents/{id}/identity (agent name, emoji)
GET  /api/graph/*              (knowledge graph queries)
GET  /api/tasks*               (task CRUD)
POST /api/tasks                (create)
PATCH /api/tasks/{id}          (update)
DEL  /api/tasks/{id}           (delete)
POST /api/tasks/{id}/complete  (mark done)
GET  /api/setup/status         (onboarding)
```

**Routes Theatron needs that don't exist anywhere:**

```
# View management
GET    /api/theatron/views                  (list all views for scope)
GET    /api/theatron/views/{id}             (get view definition)
POST   /api/theatron/views                  (create view)
PATCH  /api/theatron/views/{id}             (update layout/widgets)
DELETE /api/theatron/views/{id}             (delete view)

# Agent groups
GET    /api/theatron/groups                 (list groups)

# Agent controls (write)
PATCH  /api/nous/{id}/config               (toggle model, autonomy, wake)

# Prosoche
GET    /api/prosoche/signals                (current scored signals)
GET    /api/prosoche/status                 (cycle info, dedup state)
POST   /api/prosoche/cycle                  (force cycle)

# Daemon crons
GET    /api/daemon/crons                    (list crons + status)
POST   /api/daemon/crons/{id}/run           (force run)

# Cost time series
GET    /api/costs/daily?window=7d           (daily cost aggregates)
GET    /api/costs/by-agent?window=7d        (per-agent cost over time)

# Session replay
GET    /api/sessions/{id}/turns             (turn-by-turn data with token counts)

# Decision audit log
GET    /api/theatron/audit                  (operator mutation log)
```

---

## 4. SSE Event Gaps

Current SSE events (from `events.svelte.ts`):

```
init                          (bootstrap: agent list, active turns)
turn:before / turn:after      (agent turn lifecycle)
tool:called / tool:failed     (tool execution)
status:update                 (agent status text)
session:created / archived    (session lifecycle)
distill:before/stage/after    (distillation)
planning:*                    (7 planning events)
task:*                        (5 task events)
ping                          (heartbeat)
```

**Events Theatron needs that don't exist:**

```
health:update          (subsystem health change)
cost:tick              (periodic cost summary push)
prosoche:cycle         (prosoche scored signals)
prosoche:wake          (wake attempt)
config:changed         (taxis config hot-reload notification)
view:updated           (view definition changed by another client/agent)
agent:model-changed    (model toggle took effect)
cron:executed          (daemon cron completed)
```

---

## 5. Implementation Readiness Summary

| Phase | Backend Ready? | Frontend Ready? | Blocking Decisions |
|-------|---------------|----------------|-------------------|
| Phase 0: Frame + Widget Engine | Partial (pylon serves static, has SSE) | Need grid engine, widget registry, view store | Grid approach (CSS native vs lib) |
| Phase 1: Home + Tasks | Tasks API exists in TS, not Rust | Task store exists, need widget wrappers | None -- can proceed |
| Phase 2: Agent Detail + Controls | Need `/nous/{id}/config` write endpoint | Agent store exists, need control widgets | Config hot-reload granularity |
| Phase 3: Health Board | Partial (health check exists) | MetricsView exists, need decomposition into widgets | Prosoche/daemon API |
| Phase 4: Projects | Planning exists in TS runtime | PlanningView exists (15 components!) | None -- mostly widget wrapping |
| Phase 5: Cost Cockpit | Need time-series cost API | CostDashboard exists, need charts | Chart library decision |
| Phase 6: Session Replay | Need turns endpoint with full data | Nothing exists | Session replay data model |
| Phase 7: Canvas + Drag-and-drop | Canvas API (Spec 43b) | Nothing exists | Drag-and-drop approach |
| Phase 8: Command Palette | Need NL parsing or command router | Nothing exists | NL parsing approach |

---

## 6. What To Research Next

### High-value, stable decisions (research NOW):

1. **Grid layout engine.** Build a proof-of-concept with CSS Grid native: render 3 widgets in a 12-column grid from a JSON definition. Test responsive collapse. If it works cleanly, we're done. If not, evaluate gridstack.js Svelte integration.

2. **Widget registry pattern.** Prototype the static import map approach. Verify that dynamic `import()` + Svelte's `<svelte:component>` plays well with the registry pattern. Test that unused widgets are tree-shaken from the build.

3. **View definition schema.** Write the actual TypeScript types and YAML schema for view definitions. This is the contract everything else builds on. Include: scope, layout grid, widget placements, data bindings, config overrides.

4. **Existing component decomposition plan.** The current MetricsView, CostDashboard, PlanningView etc. need to be broken into individual widgets. Map exactly which existing component becomes which widget, what data it needs, and what config it accepts.

### Medium-value, may evolve (research LATER at M4-M5):

5. **Pylon route porting.** Many routes exist in the TS runtime but not in Rust pylon yet. These will be ported through M3-M4 naturally. Don't duplicate the design work now.

6. **Session replay data model.** Depends on how session/turn storage evolves through M3. Research when the storage layer is stable.

7. **Chart library evaluation.** Build sparklines with custom SVG first. Only evaluate uPlot/Layercake when we hit a chart that's too complex for hand-rolled SVG.

### Low-value, will change (defer to M6):

8. **Drag-and-drop implementation.** Phase 7. The grid engine needs to work with static placement first.

9. **Command palette NL parsing.** Phase 8. Entirely separate concern.

10. **Agent-authored view UX.** How views appear in nav, approval flow, forking UX. Design when the basic view system works.
