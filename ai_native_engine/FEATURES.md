# Features

This document lists what is currently implemented in the AI-Native Engine and what is on the roadmap.

## Implemented

| Feature | Status | Notes |
|---------|--------|-------|
| Rust toolchain setup | ✅ Done | Rust 1.98.1 via rustup |
| Windows C++ build tools | ✅ Done | Visual Studio Build Tools 2022 with C++ workload |
| Bevy 0.19.1 integration | ✅ Done | Default plugins, wgpu renderer, winit window |
| 3D windowed application | ✅ Done | Opens a window with Vulkan backend |
| Basic 3D scene | ✅ Done | Camera, point light, and rotating red cube |
| ECS architecture | ✅ Done | Native Bevy ECS for entities, components, systems |
| Hot compilation | ✅ Done | Incremental cargo builds after dependencies are cached |
| Project structure | ✅ Done | Standard Cargo project with `.gitignore` |

## In Progress

| Feature | Status | Notes |
|---------|--------|-------|
| AI agent tool interface | 🔄 Planned | MCP server with 10 scene-manipulation tools |

## Roadmap

| Feature | Status | Notes |
|---------|--------|-------|
| ARPG combat system | ⏳ Planned | Hit detection, damage, health |
| Enemy AI behavior | ⏳ Planned | Patrol/chase/attack state machines |
| Player controller | ⏳ Planned | Movement and attack input |
| Quest/dialogue system | ⏳ Planned | Data-driven quest graphs |
| Inventory/loot system | ⏳ Planned | Item definitions and drop tables |
| Skill/ability system | ⏳ Planned | Cooldowns, effects, animations |
| glTF asset pipeline | ⏳ Planned | Blender export/import workflow |
| PBR material system | ⏳ Planned | Standard material with lighting |
| Scene viewer / inspector | ⏳ Planned | Minimal visual editor fallback |
| Level streaming | ⏳ Planned | Open world loading/unloading |
| Save/load system | ⏳ Planned | Serialize game state |
| AI-generated content | ⏳ Planned | LLM-driven scene, quest, and dialogue creation |
| AI playtesting loop | ⏳ Planned | Automated testing and bug reporting |

## Legend

- ✅ Done — implemented and verified
- 🔄 In Progress — currently being worked on
- ⏳ Planned — scheduled for future work
