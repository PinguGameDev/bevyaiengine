# MCP Server Implementation Plan

## Overview

Build a Model Context Protocol (MCP) server with stdio transport that exposes 10 scene-manipulation tools. This allows AI agents (Claude Desktop, Codex, OpenCode) to control the Bevy engine via structured tool calls.

**Transport:** stdio (standard for local AI agents)
**Crate:** `rmcp` v3.4.0 with `transport-io` feature
**Estimated time:** 2-3 hours

---

## Architecture

```
AI Agent (Claude/Codex/OpenCode)
        |
        | stdio (JSON-RPC)
        v
   MCP Server (rmcp)
        |
        | channels / shared state
        v
   Entity Registry (HashMap<String, Entity>)
        |
        | Bevy ECS commands
        v
   Bevy Engine (renders scene)
```

The MCP server runs in a separate thread with its own tokio runtime. It communicates with the Bevy ECS through a shared `EntityRegistry` resource and Bevy channels.

---

## File Structure

```
src/
├── main.rs              # App entry, Bevy setup, plugin registration
├── scene.rs             # Scene setup, entity spawning helpers
├── entity_registry.rs   # Named entity registry (HashMap<String, Entity>)
└── mcp_server.rs        # MCP tool definitions and stdio transport
```

---

## Step 1: Add `transport-io` Feature to rmcp

Update `Cargo.toml`:

```toml
rmcp = { version = "3.4.0", features = ["transport-io"] }
```

---

## Step 2: Named Entity Registry

A Bevy `Resource` that maps string names to ECS entities. Every tool that creates, deletes, or transforms entities goes through this registry.

```rust
#[derive(Resource, Default)]
pub struct EntityRegistry {
    entities: HashMap<String, Entity>,
}

impl EntityRegistry {
    pub fn register(&mut self, name: String, entity: Entity) { ... }
    pub fn remove(&mut self, name: &str) -> Option<Entity> { ... }
    pub fn get(&self, name: &str) -> Option<Entity> { ... }
    pub fn list(&self) -> Vec<(String, Entity)> { ... }
}
```

---

## Step 3: MCP Server with 10 Tools

### Tool Definitions

| # | Tool | Parameters | Description |
|---|------|-----------|-------------|
| 1 | `create_entity` | `name: String` | Spawn a named empty entity |
| 2 | `delete_entity` | `name: String` | Remove entity by name |
| 3 | `set_transform` | `name: String, x: f32, y: f32, z: f32` | Move an entity |
| 4 | `rotate_entity` | `name: String, x: f32, y: f32, z: f32` | Rotate an entity (degrees) |
| 5 | `scale_entity` | `name: String, x: f32, y: f32, z: f32` | Scale an entity |
| 6 | `create_cube` | `name: String, x: f32, y: f32, z: f32, color: String` | Spawn a colored cube |
| 7 | `create_light` | `name: String, x: f32, y: f32, z: f32, intensity: f32` | Spawn a point light |
| 8 | `create_camera` | `name: String, x: f32, y: f32, z: f32` | Spawn a camera |
| 9 | `list_entities` | none | Return all entities in the scene |
| 10 | `screenshot` | none | Capture the current rendered frame |

### Tool Implementation Pattern

```rust
#[tool_handler]
impl MyMcpServer {
    #[tool(description = "Create a named empty entity at the origin")]
    async fn create_entity(&self, name: String) -> Result<String, Error> {
        // Send command to Bevy via channel
        // Register entity in EntityRegistry
        Ok(format!("Entity '{}' created", name))
    }

    #[tool(description = "Delete an entity by name")]
    async fn delete_entity(&self, name: String) -> Result<String, Error> {
        // Look up entity in registry
        // Send despawn command to Bevy
        // Remove from registry
        Ok(format!("Entity '{}' deleted", name))
    }

    // ... 8 more tools
}
```

---

## Step 4: Integration with Bevy

### Communication Pattern

```
MCP Thread                          Bevy Thread
    |                                    |
    |--- McpCommand::Create(name) ------>|
    |                                    |--- commands.spawn(...)
    |                                    |--- registry.register(name, entity)
    |<-- Ok("Entity created") -----------|
    |                                    |
```

### Bevy Systems

- `process_mcp_commands` — reads commands from channel, applies to ECS
- `sync_entity_registry` — keeps registry in sync with world state
- `screenshot_system` — captures frame when requested

---

## Step 5: Verification

1. Build and run the engine
2. Configure Claude Desktop / OpenCode to connect to the MCP server via stdio
3. Test each tool:
   - "Create a red cube at position 0, 1, 0"
   - "Add a point light above the cube"
   - "List all entities in the scene"
   - "Move the cube to position 2, 0, 0"
   - "Delete the cube"
4. Verify scene updates are visible in the Bevy window

---

## Step 6: Later — Add Jev Decision Layer

Once the Vercel AI Gateway credit card is verified:

| Jev Use Case | Primitive | Example |
|-------------|-----------|---------|
| Tool routing | Choice | "Which tool should handle this request?" |
| Guardrails | Noul | "Is this action safe to execute?" |
| Content scoring | Score | "Rate this generated scene layout 1-5" |
| Task completion | Noul | "Does the scene match the user's request?" |
| Model routing | Choice | "Which generative model handles this?" |

---

## Dependencies

| Crate | Version | Feature | Purpose |
|-------|---------|---------|---------|
| `rmcp` | 3.4.0 | `transport-io` | MCP server with stdio transport |
| `tokio` | 1.53.1 | `full` | Async runtime (already installed) |
| `serde` | 1.0.229 | `derive` | Serialization (already installed) |
| `serde_json` | 1.0.151 | — | JSON handling (already installed) |

---

## Risks and Mitigations

| Risk | Mitigation |
|------|-----------|
| MCP server blocks Bevy render loop | Run MCP in separate thread with own tokio runtime |
| Entity name collisions | Return error if name already exists; require unique names |
| Screenshot requires GPU readback | Use Bevy's built-in screenshot API; save to file |
| rmcp API may differ from docs | Check crate docs at build time; adapt as needed |

---

## Success Criteria

- [ ] Engine opens a window with starter scene
- [ ] MCP server starts on stdio and accepts connections
- [ ] AI agent can call all 10 tools
- [ ] Scene updates are visible in the Bevy window in real-time
- [ ] `list_entities()` returns correct entity list
- [ ] `screenshot()` saves a PNG file
- [ ] No crashes or panics during normal operation
