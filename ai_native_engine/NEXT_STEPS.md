# Next Steps

This document tracks the immediate and upcoming work for the AI-Native Engine project.

## Immediate: AI Agent Tool Interface (Weeks 1-2)

Goal: enable an AI agent to manipulate the Bevy scene through a structured tool interface.

- [ ] Add an MCP (Model Context Protocol) server to the project
- [ ] Expose 10 initial engine tools:
  - `create_entity(name)` — spawn a named empty entity
  - `delete_entity(name)` — remove an entity by name
  - `set_transform(name, x, y, z)` — move an entity
  - `rotate_entity(name, x, y, z)` — rotate an entity
  - `scale_entity(name, x, y, z)` — scale an entity
  - `create_cube(name, x, y, z, color)` — spawn a colored cube
  - `create_light(name, x, y, z, intensity)` — spawn a point light
  - `create_camera(name, x, y, z)` — spawn a camera
  - `list_entities()` — return all entities in the scene
  - `screenshot()` — capture the current rendered frame
- [ ] Document the tool schema (JSON-RPC / MCP)
- [ ] Validate that an LLM can call these tools and modify the running scene

## Short-Term: ARPG Core Systems (Weeks 3-8)

Goal: prove the engine can power an Action RPG prototype.

- [ ] Combat system (hit detection, damage, health)
- [ ] Basic enemy AI (patrol/chase/attack states)
- [ ] Player controller (movement, attack input)
- [ ] Quest and dialogue data model
- [ ] Inventory and loot table system
- [ ] Skill/ability system
- [ ] Save/load game state

## Medium-Term: Developer Experience (Weeks 9-16)

Goal: make the engine usable by a solo developer with AI assistance.

- [ ] glTF asset import pipeline (Blender → engine)
- [ ] Material/shader system with PBR support
- [ ] Minimal scene viewer and property inspector
- [ ] Hot reload for scripts, scenes, and assets
- [ ] Open world / level streaming prototype
- [ ] Build configuration for release and dynamic linking

## Long-Term: Funding Demo & Engine Hardening (Weeks 17-24)

Goal: produce a fundable demo and a stable foundation.

- [ ] End-to-end demo: natural language → AI generates ARPG scene
- [ ] AI-generated quests, dialogue, and enemy placements
- [ ] AI playtesting and bug reporting loop
- [ ] Performance profiling and optimization
- [ ] Documentation and example projects
- [ ] Funding pitch materials and investor outreach

## How to update this file

When a task is completed, move it under `FEATURES.md` and mark the checkbox here as done.
