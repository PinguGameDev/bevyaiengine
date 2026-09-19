# Setup Runbook: AI-Native Engine

This document captures the exact setup steps, commands, and fixes from this conversation so the project can be reproduced on another Windows machine.

---

## Original Environment

- **OS**: Windows 11 Education
- **CPU**: Intel(R) Core(TM) i3-8145U @ 2.10GHz, 2 cores
- **RAM**: 7.9 GiB
- **GPU**: Intel(R) UHD Graphics 620
- **Renderer backend used by Bevy**: Vulkan

---

## Step 1: Install Rust Toolchain

Installed **Rust 1.98.1** via rustup.

### Commands used

```powershell
Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "C:\Users\admin\AppData\Local\Temp\opencode\rustup-init.exe"
& "C:\Users\admin\AppData\Local\Temp\opencode\rustup-init.exe" -y
```

### Verify installation

```powershell
cargo --version
rustc --version
```

Expected output:
```
cargo 1.98.1 (797e8a9bc 2026-08-05)
rustc 1.98.1 (48a229cea 2026-09-01)
```

### Default host triple

```
x86_64-pc-windows-msvc
```

---

## Step 2: Install Visual Studio Build Tools

Rust on Windows requires the MSVC C++ linker (`link.exe`). The first build failed with:

```
error: linker `link.exe` not found
```

### Fix

Installed **Visual Studio Build Tools 2022 v17.14.41** with the C++ workload via winget:

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools --silent --accept-package-agreements --accept-source-agreements --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

### Where link.exe ended up

```
C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\link.exe
```

After installation, a fresh PowerShell session should find `link.exe` automatically. If not, either:
- Add the directory above to `PATH`, or
- Use the **Developer Command Prompt for VS 2022**.

---

## Step 3: Reload PATH in Current Session

If `cargo` is not found in the current PowerShell window after installing Rust, reload PATH:

```powershell
$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
```

---

## Step 4: Scaffold the Bevy Project

Working directory: `C:\Users\admin\devel\ai_ue`

### Create project

```powershell
cargo new ai_native_engine --bin
cargo add bevy --manifest-path ai_native_engine\Cargo.toml
```

### Resulting dependency

`ai_native_engine/Cargo.toml`:
```toml
[dependencies]
bevy = "0.19.1"
```

Bevy version resolved: **0.19.1**.

---

## Step 5: Build and Run

```powershell
cd ai_native_engine
cargo run
```

### First build

- Downloads ~550 crates.
- Compiles Bevy and dependencies.
- Took roughly 10-20 minutes total on the original machine (Intel i3, 2 cores).

### Subsequent builds

Took ~1.5 minutes for the project after dependencies were cached.

### Verified runtime output

```
SystemInfo { os: "Windows 11 Education", kernel: "26200", cpu: "Intel(R) Core(TM) i3-8145U CPU @ 2.10GHz", core_count: "2", memory: "7.9 GiB" }
AdapterInfo { name: "Intel(R) UHD Graphics 620", vendor: 32902, device: 16032, device_type: IntegratedGpu, ... backend: Vulkan, ... }
GPU clustering is supported on this device.
GPU preprocessing is fully supported on this device.
Creating new window ai_native_engine (65v0)
AI-Native Engine starter scene loaded.
```

A window titled **ai_native_engine** opened with a rotating red cube.

---

## Project Files

After setup, the project contains:

```
C:\Users\admin\devel\ai_ue\
├── AI_Native_Engine_Plan.md          # High-level product plan
└── ai_native_engine\
    ├── Cargo.toml
    ├── Cargo.lock
    ├── .gitignore
    ├── README.md                        # Usage guide
    ├── FEATURES.md                      # Implemented + roadmap features
    ├── NEXT_STEPS.md                    # Prioritized task list
    └── src\
        └── main.rs                      # Starter 3D scene
```

---

## Common Issues

### `cargo` not found

Open a new PowerShell window, or reload PATH with:

```powershell
$env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("PATH", "User")
```

### `link.exe` not found during build

Install Visual Studio Build Tools 2022 with the C++ workload (see Step 2). Then open a fresh terminal.

### Builds are slow

Expected for the first build. For development, you can enable Bevy dynamic linking in `Cargo.toml`:

```toml
[dependencies]
bevy = { version = "0.19.1", features = ["dynamic_linking"] }
```

Do **not** use `dynamic_linking` for release builds.

---

## Next Action

The immediate next step from `NEXT_STEPS.md` is to add an MCP server with 10 scene-manipulation tools so an AI agent can control the engine.
