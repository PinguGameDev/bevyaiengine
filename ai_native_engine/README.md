# AI-Native Engine

An Action RPG game engine built in Rust with Bevy, driven by AI agents through an MCP (Model Context Protocol) tool interface over stdio.

## What is this?

This is the foundational project for an AI-native game engine. The long-term vision is a system where a developer can describe a game in natural language and an AI agent builds it by calling engine tools. The engine is purpose-built for Action RPGs (Diablo / Path of Exile / Hades style).

## Prerequisites

### Windows

1. **Rust toolchain** — install via [rustup](https://rustup.rs/):
   ```powershell
   Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
   .\rustup-init.exe -y
   ```

2. **Visual Studio Build Tools 2022** with the C++ workload (required for the MSVC linker):
   ```powershell
   winget install --id Microsoft.VisualStudio.2022.BuildTools --silent --accept-package-agreements --accept-source-agreements --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
   ```

3. A GPU with Vulkan, DirectX 12, or OpenGL support. The default Bevy renderer uses wgpu, which selects the best available backend.

## Build and Run

From the project root:

```powershell
cd ai_native_engine
cargo run
```

The first build downloads and compiles all dependencies and may take 10-20 minutes. Subsequent builds are much faster (~1 min).

You should see a window titled **ai_native_engine** with a rotating red cube. The MCP server starts automatically on stdio.

## Project Structure

```
ai_native_engine/
├── Cargo.toml              # Dependencies: bevy, tokio, reqwest, serde, serde_json
├── FEATURES.md             # Feature status tracker
├── NEXT_STEPS.md           # Upcoming work and roadmap
├── README.md               # This file
└── src/
    ├── main.rs             # Entry point, Bevy app setup, MCP command processor
    ├── scene.rs            # Starter scene (camera, light, rotating cube)
    ├── entity_registry.rs  # Named entity registry (HashMap<String, Entity>)
    └── mcp_server.rs       # JSON-RPC MCP server over stdio transport
```

## MCP Server

The engine exposes a JSON-RPC 2.0 MCP server over stdio. AI agents (Claude Desktop, Codex, OpenCode) can connect and call tools to manipulate the scene in real-time.

### Connecting an AI Agent

Configure your MCP client to launch the engine as a stdio server:

```json
{
  "mcpServers": {
    "ai-native-engine": {
      "command": "cargo",
      "args": ["run", "--manifest-path", "C:/dev/bevyaiengine/ai_native_engine/Cargo.toml"]
    }
  }
}
```

### Available Tools

| Tool | Parameters | Description |
|------|-----------|-------------|
| `create_entity` | `name` | Spawn a named empty entity |
| `delete_entity` | `name` | Remove entity by name |
| `set_transform` | `name, x, y, z` | Move an entity to a position |
| `rotate_entity` | `name, x, y, z` | Rotate an entity (degrees) |
| `scale_entity` | `name, x, y, z` | Scale an entity |
| `create_cube` | `name, x, y, z, color` | Spawn a colored cube (red/green/blue/yellow/white/gray) |
| `create_light` | `name, x, y, z, intensity` | Spawn a point light |
| `create_camera` | `name, x, y, z` | Spawn a 3D camera looking at origin |
| `list_entities` | — | List all registered entities |
| `screenshot` | — | Capture rendered frame (not yet implemented) |

### Example MCP Session

```json
{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}
{"jsonrpc":"2.0","method":"tools/call","params":{"name":"create_cube","arguments":{"name":"red_box","x":2.0,"y":0.0,"z":0.0,"color":"red"}},"id":2}
{"jsonrpc":"2.0","method":"tools/call","params":{"name":"create_light","arguments":{"name":"main_light","x":0.0,"y":5.0,"z":0.0,"intensity":2000000.0}},"id":3}
{"jsonrpc":"2.0","method":"tools/call","params":{"name":"list_entities","arguments":{}},"id":4}
```

## Tested and Working

| Feature | Status | Notes |
|---------|--------|-------|
| Engine startup | Working | Vulkan backend, NVIDIA/AMD/Intel GPUs supported |
| 3D window | Working | Opens with starter scene (camera, light, rotating cube) |
| MCP server (stdio) | Working | JSON-RPC 2.0, runs in separate thread |
| MCP initialize | Working | Returns protocol version and server info |
| MCP tools/list | Working | Returns all 10 tools with JSON schemas |
| create_entity | Working | Creates named entity, rejects duplicate names |
| delete_entity | Working | Removes entity, handles missing entities gracefully |
| set_transform | Working | Moves entity, handles missing entities |
| rotate_entity | Working | Rotates entity with Euler angles (degrees) |
| scale_entity | Working | Scales entity uniformly or per-axis |
| create_cube | Working | Supports 6 named colors |
| create_light | Working | Configurable intensity |
| create_camera | Working | Auto-aims at origin |
| list_entities | Working | Returns comma-separated entity names |
| screenshot | Not implemented | Returns placeholder message |
| Duplicate name detection | Working | Returns error if entity name already exists |
| Error handling | Working | Graceful messages for missing entities and unknown commands |

## Known Issues

- **rotate_entity / scale_entity** replace the full Transform, which can lose position or rotation data. These should compose with the existing transform instead.
- **screenshot** is a placeholder — Bevy's screenshot API needs to be wired up.
- **Starter scene entities** (the default camera, light, and cube) are not registered in the EntityRegistry, so they cannot be manipulated via MCP tools. Only entities created through MCP tools are tracked.
- **blocking_lock()** is used to read MCP commands inside a Bevy system. This works but could cause frame hitches under heavy MCP traffic.

## Usage

### Running the engine

```powershell
cargo run
```

### Building for release

```powershell
cargo build --release
```

### Running tests

```powershell
cargo test
```

## Troubleshooting

### `cargo` not found

Open a new PowerShell window or reload PATH:

```powershell
$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
```

### `link.exe` not found

Visual Studio Build Tools are not installed or not in PATH. Re-run the winget command above or open **Developer Command Prompt for VS 2022**.

### Slow builds

Bevy is large. After the first build, incremental compilation is fast. You can also enable dynamic linking for development:

```toml
[dependencies]
bevy = { version = "0.19.1", features = ["dynamic_linking"] }
```

Note: `dynamic_linking` should not be used for release builds.

## Roadmap

See [NEXT_STEPS.md](./NEXT_STEPS.md) and [FEATURES.md](./FEATURES.md) for the detailed roadmap. Key upcoming work:

- Implement screenshot tool
- Fix rotate/scale to compose with existing transforms
- Register starter scene entities in EntityRegistry
- Add Jev AI decision layer (tool routing, guardrails, content scoring)
- Build ARPG systems (combat, enemy AI, quests, inventory)
- glTF asset pipeline
- Scene viewer / property inspector

## License

TBD — to be determined once the project direction and funding approach are finalized.
