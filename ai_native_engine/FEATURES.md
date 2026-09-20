# Features

This document lists what is currently implemented in the AI-Native Engine and what is on the roadmap.

## Implemented

| Feature | Status | Notes |
|---------|--------|-------|
| Rust toolchain setup | Done | Rust 1.98.1 via rustup |
| Windows C++ build tools | Done | Visual Studio Build Tools 2022 with C++ workload |
| Bevy 0.19.1 integration | Done | Default plugins, wgpu renderer, winit window |
| 3D windowed application | Done | Opens a window with Vulkan backend |
| Basic 3D scene | Done | Camera, point light, and rotating red cube, all registered in EntityRegistry |
| ECS architecture | Done | Native Bevy ECS for entities, components, systems |
| Hot compilation | Done | Incremental cargo builds after dependencies are cached |
| Project structure | Done | Modular: main.rs, scene.rs, entity_registry.rs, mcp_server.rs |
| Named entity registry | Done | HashMap<String, Entity> for AI-addressable entities |
| MCP server (stdio) | Done | JSON-RPC 2.0 over stdio, runs in separate thread |
| MCP tool: create_entity | Done | Creates named empty entity, rejects duplicates |
| MCP tool: delete_entity | Done | Removes entity by name, handles missing entities |
| MCP tool: set_transform | Done | Moves entity to (x, y, z) |
| MCP tool: rotate_entity | Done | Rotates entity with Euler angles (degrees), composes with existing rotation |
| MCP tool: scale_entity | Done | Scales entity per-axis, composes with existing scale |
| MCP tool: create_cube | Done | Spawns colored cube (red/green/blue/yellow/white/gray) |
| MCP tool: create_light | Done | Spawns point light with configurable intensity |
| MCP tool: create_camera | Done | Spawns 3D camera auto-aimed at origin |
| MCP tool: list_entities | Done | Returns all registered entity names |
| MCP initialize handshake | Done | Returns protocol version and server capabilities |
| MCP tools/list | Done | Returns all 10 tools with JSON schemas |
| Error handling | Done | Graceful messages for missing entities, duplicates, unknown commands |
| Tokio async runtime | Done | Full async runtime for MCP server and future HTTP calls |
| reqwest HTTP client | Done | For future Jev API integration |
| serde + serde_json | Done | JSON serialization for MCP protocol |

## In Progress

| Feature | Status | Notes |
|---------|--------|-------|
| MCP tool: screenshot | Planned | Placeholder exists, needs Bevy screenshot API |
| Jev AI integration | Blocked | Requires Vercel AI Gateway credit card verification |

## Known Issues

| Issue | Severity | Notes |
|-------|----------|-------|
| MCP tool: screenshot | Medium | Placeholder exists, needs Bevy screenshot API implementation |
| blocking_lock() in Bevy system | Low | Works but could cause frame hitches under heavy MCP traffic |

## Roadmap

| Feature | Status | Notes |
|---------|--------|-------|
| Jev AI tool router | Planned | Choice: which tool to call based on NL input |
| Jev AI guardrails | Planned | Noul: validate tool calls before execution |
| Jev AI content scoring | Planned | Score: rate generated content quality |
| ARPG combat system | Planned | Hit detection, damage, health |
| Enemy AI behavior | Planned | Patrol/chase/attack state machines |
| Player controller | Planned | Movement and attack input |
| Quest/dialogue system | Planned | Data-driven quest graphs |
| Inventory/loot system | Planned | Item definitions and drop tables |
| Skill/ability system | Planned | Cooldowns, effects, animations |
| glTF asset pipeline | Planned | Blender export/import workflow |
| PBR material system | Planned | Standard material with lighting |
| Scene viewer / inspector | Planned | Minimal visual editor fallback |
| Level streaming | Planned | Open world loading/unloading |
| Save/load system | Planned | Serialize game state |
| AI-generated content | Planned | LLM-driven scene, quest, and dialogue creation |
| AI playtesting loop | Planned | Automated testing and bug reporting |

## Legend

- Done — implemented and verified
- Planned — scheduled for future work
- Blocked — waiting on external dependency
