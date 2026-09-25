# Camera Controller Plan

## Overview

A professional viewport camera controller combining the best features from UE5, Unity, Blender, and custom game engines. Provides both orbit (arcball) and fly (6DOF) modes for maximum flexibility.

## Navigation Modes

### Orbit Mode (Default)
| Input | Action | Notes |
|-------|--------|-------|
| Right mouse drag | Orbit around focus point | Primary navigation |
| Middle mouse drag | Pan (screen-space) | Move focus point |
| Mouse wheel | Zoom (dolly) | Distance-based speed |
| Alt + Left drag | Orbit (alternative) | Matches UE5/Unity |
| Ctrl + Right drag | Zoom (alternative) | Matches UE5 |
| F key | Focus on selection | Smooth transition |

### Fly Mode (WASD)
| Input | Action | Notes |
|-------|--------|-------|
| Right mouse + WASD | Fly camera | Hold RMB to activate |
| W | Move forward | In camera look direction |
| S | Move backward | Opposite of camera look direction |
| A | Move left | Relative to camera orientation |
| D | Move right | Relative to camera orientation |
| E / Space | Move up | World-space vertical movement |
| Q | Move down | World-space vertical movement |
| Ctrl | Slow mode | Precision control (0.1x speed) |
| Shift | Fast mode | Speed boost (3x speed) |
| Mouse | Free look | Yaw/pitch rotation |

### Quick Views (Numpad)
| Input | Action | Notes |
|-------|--------|-------|
| Numpad 1 | Front view | Orthographic |
| Numpad 3 | Right view | Orthographic |
| Numpad 7 | Top view | Orthographic |
| Numpad 0 | Game camera | Toggle to main camera |
| Numpad 5 | Toggle ortho/perspective | Switch projection |

## Implementation Details

### Data Structure

```rust
#[derive(Component)]
pub struct OrbitCamera {
    // Orbit state
    pub target: Vec3,              // Focus point
    pub distance: f32,             // Distance from target
    pub yaw: f32,                  // Horizontal rotation (radians)
    pub pitch: f32,                // Vertical rotation (radians, clamped)
    
    // Smooth interpolation targets
    target_distance: f32,
    target_yaw: f32,
    target_pitch: f32,
    target_position: Vec3,
    
    // Configuration
    pub sensitivity: f32,          // Mouse sensitivity (0.005)
    pub zoom_speed: f32,           // Zoom multiplier (0.1)
    pub pan_speed: f32,            // Pan multiplier (0.01)
    pub fly_speed: f32,            // Fly mode speed (5.0)
    pub smoothing: f32,            // Interpolation factor (0.15)
    pub min_distance: f32,         // Minimum zoom distance (0.1)
    pub max_distance: f32,         // Maximum zoom distance (1000.0)
    
    // State flags
    pub mode: CameraMode,
    pub is_flying: bool,
    pub is_ortho: bool,
}

#[derive(PartialEq, Clone, Copy)]
pub enum CameraMode {
    Orbit,
    Fly,
    Pan,
}
```

### Systems

1. **`camera_input_system`**
   - Reads mouse and keyboard input
   - Updates OrbitCamera state based on input
   - Handles mode switching (orbit vs fly)

2. **`camera_transform_system`**
   - Converts OrbitCamera state to Transform
   - Applies smoothing/interpolation
   - Calculates position from spherical coordinates

3. **`focus_on_selection_system`**
   - Listens for F key press
   - Finds selected entity position from DashboardState
   - Smoothly moves target to selection

### Key Features

#### Smooth Interpolation
```rust
// Each frame, interpolate toward target values
camera.distance = camera.distance.lerp(camera.target_distance, camera.smoothing);
camera.yaw = camera.yaw.lerp(camera.target_yaw, camera.smoothing);
camera.pitch = camera.pitch.lerp(camera.target_pitch, camera.smoothing);
camera.target = camera.target.lerp(camera.target_position, camera.smoothing);
```

#### Distance-Based Speed
```rust
// Pan and zoom speed scales with distance
let pan_multiplier = camera.distance * camera.pan_speed;
let zoom_multiplier = camera.distance * camera.zoom_speed;
```

#### Pitch Clamping
```rust
// Prevent gimbal lock at poles
camera.pitch = camera.pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());
```

#### Fly Mode Physics
```rust
// In fly mode, WASD moves relative to camera orientation
let forward = Vec3::new(
    camera.yaw.sin() * camera.pitch.cos(),
    camera.pitch.sin(),
    -camera.yaw.cos() * camera.pitch.cos(),
);
let right = Vec3::new(
    camera.yaw.cos(),
    0.0,
    camera.yaw.sin(),
);
let up = Vec3::Y;

let mut velocity = Vec3::ZERO;
if keyboard.pressed(KeyCode::KeyW) { velocity += forward; }
if keyboard.pressed(KeyCode::KeyS) { velocity -= forward; }
if keyboard.pressed(KeyCode::KeyA) { velocity -= right; }
if keyboard.pressed(KeyCode::KeyD) { velocity += right; }
if keyboard.pressed(KeyCode::KeyE) || keyboard.pressed(KeyCode::Space) { velocity += up; }
if keyboard.pressed(KeyCode::KeyQ) { velocity -= up; }

// Apply speed modifiers
if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) { velocity *= 0.1; }
if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) { velocity *= 3.0; }

camera.target_position += velocity * camera.fly_speed * time.delta_secs();
```

## Files to Modify

1. **New:** `src/camera_controller.rs`
   - OrbitCamera component
   - CameraMode enum
   - camera_input_system
   - camera_transform_system
   - focus_on_selection_system
   - CameraControllerPlugin

2. **Modify:** `src/scene.rs`
   - Add OrbitCamera component to starter_camera
   - Configure initial camera state

3. **Modify:** `src/main.rs`
   - Add `mod camera_controller`
   - Add CameraControllerPlugin to App
   - Ensure systems run in Update schedule

4. **Optional:** `src/dashboard.rs`
   - Add camera settings panel (sensitivity, speed, etc.)
   - Add camera mode indicator

## Testing Checklist

- [ ] Right mouse drag orbits smoothly
- [ ] Middle mouse drag pans correctly
- [ ] Mouse wheel zooms with distance-based speed
- [ ] WASD fly mode works when holding right mouse
- [ ] W moves camera forward
- [ ] S moves camera backward
- [ ] A moves camera left
- [ ] D moves camera right
- [ ] E / Space moves camera up
- [ ] Q moves camera down
- [ ] F key focuses on selected entity
- [ ] Camera doesn't flip upside down (pitch clamped)
- [ ] Smooth interpolation feels natural
- [ ] Speed modifiers (Ctrl/Shift) work correctly
- [ ] Dashboard still works alongside camera
- [ ] No frame hitches or performance issues

## Performance Considerations

- Use `Single` query for camera (only one active camera)
- Avoid allocations in hot paths
- Use `Time::delta_secs()` for frame-rate independent movement
- Consider `FixedUpdate` for physics-based camera if needed

## Future Enhancements

- [ ] Camera presets (save/load positions)
- [ ] Camera animation/tweening
- [ ] Collision detection (don't clip through geometry)
- [ ] Camera shake effects
- [ ] Cinematic camera paths
- [ ] Multi-camera switching
- [ ] VR camera support
