# Next Steps

This document tracks the immediate and upcoming work for the AI-Native Engine project.

## Completed: AI Agent Tool Interface

Goal: enable an AI agent to manipulate the Bevy scene through a structured tool interface.

- [x] Add an MCP (Model Context Protocol) server to the project
- [x] Expose 10 initial engine tools:
  - [x] `create_entity(name)` — spawn a named empty entity
  - [x] `delete_entity(name)` — remove an entity by name
  - [x] `set_transform(name, x, y, z)` — move an entity
  - [x] `rotate_entity(name, x, y, z)` — rotate an entity
  - [x] `scale_entity(name, x, y, z)` — scale an entity
  - [x] `create_cube(name, x, y, z, color)` — spawn a colored cube
  - [x] `create_light(name, x, y, z, intensity)` — spawn a point light
  - [x] `create_camera(name, x, y, z)` — spawn a camera
  - [x] `list_entities()` — return all entities in the scene
  - [ ] `screenshot()` — capture the current rendered frame (placeholder)
- [x] Document the tool schema (JSON-RPC / MCP)
- [x] Validate that an LLM can call these tools and modify the running scene

## Completed: Developer Dashboard (Weeks 3-5)

Goal: build a real-time observation and control surface for watching the AI work and making manual adjustments.

**Approach:** egui-based dashboard inside the Bevy window (not a separate terminal TUI). This keeps everything in one process, shares ECS data directly, and avoids the complexity of IPC between separate windows.

### Dashboard Panels

- [x] **Entity List** — clickable list of all entities from EntityRegistry, with type icons and filter/search
- [x] **Property Inspector** — show Transform (position, rotation, scale) and components for selected entity
- [x] **System Log** — bottom strip showing Bevy logs and MCP events, filterable by level (INFO/WARN/ERROR)
- [x] **MCP Message Log** — bottom panel showing MCP tool call history
- [x] **Menu Bar** — top menu for toggling panel visibility
- [ ] **AI Conversation Log** — scrollable panel showing the full conversation between user and AI agent (future enhancement)
- [ ] **Viewport Camera Controls** — orbit camera (pan/zoom/rotate around scene) for better spatial understanding

### Architecture

- [x] Add `bevy_egui` dependency
- [x] Create dashboard plugin with egui panels
- [x] Add MCP message log ring buffer (capture all tool calls and responses)
- [x] Wire entity list to EntityRegistry
- [x] Wire property inspector to ECS Transform queries
- [x] Hook into Bevy's LogPlugin for system log panel
- [ ] Implement orbit camera controller

### Known Issues to Fix

- [x] Fix `rotate_entity` / `scale_entity` to compose with existing Transform instead of replacing it
- [x] Register starter scene entities (camera, light, cube) in EntityRegistry
- [ ] Replace `blocking_lock()` with async channel to avoid frame hitches
- [ ] Implement `screenshot()` tool using Bevy's screenshot API

## Short-Term: ARPG Core Systems (Weeks 6-11)

Goal: prove the engine can power an Action RPG prototype.

- [ ] Combat system (hit detection, damage, health)
- [ ] Basic enemy AI (patrol/chase/attack states)
- [ ] Player controller (movement, attack input)
- [ ] Quest and dialogue data model
- [ ] Inventory and loot table system
- [ ] Skill/ability system
- [ ] Save/load game state

## Medium-Term: Developer Experience (Weeks 12-19)

Goal: make the engine usable by a solo developer with AI assistance.

- [ ] glTF asset import pipeline (Blender → engine)
- [ ] Material/shader system with PBR support
- [ ] Hot reload for scripts, scenes, and assets
- [ ] Open world / level streaming prototype
- [ ] Build configuration for release and dynamic linking

## Long-Term: Jev AI Integration & Funding Demo (Weeks 20-26)

Goal: add the AI decision layer and produce a fundable demo.

**Prerequisite:** Vercel AI Gateway credit card verification to unlock Jev API access.

### Jev AI Decision Layer

- [ ] Jev client module (Rust SDK via Vercel AI Gateway)
- [ ] Tool router using Jev Choice (which tool to call based on natural language)
- [ ] Guardrail system using Jev Noul (validate tool calls before execution)
- [ ] Content scoring using Jev Score (rate generated content quality)
- [ ] Task completion verification using Jev Noul (does the scene match the request?)

### Funding Demo

- [ ] End-to-end demo: natural language → AI generates ARPG scene
- [ ] AI-generated quests, dialogue, and enemy placements
- [ ] AI playtesting and bug reporting loop
- [ ] Performance profiling and optimization
- [ ] Documentation and example projects
- [ ] Funding pitch materials and investor outreach

## How to update this file

When a task is completed, move it under `FEATURES.md` and mark the checkbox here as done.
