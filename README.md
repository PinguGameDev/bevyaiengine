# AI-Native Engine

An Action RPG game engine built in Rust with Bevy, designed to be driven by AI agents through a structured tool interface.

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

The first build downloads and compiles all Bevy dependencies and may take 10-20 minutes. Subsequent builds are much faster.

You should see a window titled **ai_native_engine** with a rotating red cube.

## Project Structure

```
ai_native_engine/
├── Cargo.toml          # Rust project configuration and Bevy dependency
├── .gitignore          # Excludes target/ and Cargo.lock
├── NEXT_STEPS.md       # Upcoming work and roadmap
├── FEATURES.md         # Current and planned features
├── README.md           # This file
└── src/
    └── main.rs         # Entry point and starter scene
```

## Current Capabilities

- Open a 3D window with Bevy's default plugins
- Render a PBR-lit scene with a camera, point light, and rotating cube
- Compile and run on Windows with Vulkan backend

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

## Next Steps

See [NEXT_STEPS.md](./NEXT_STEPS.md) for the detailed roadmap. The immediate priority is adding an MCP server so an AI agent can manipulate scenes through tools.

## License

TBD — to be determined once the project direction and funding approach are finalized.
