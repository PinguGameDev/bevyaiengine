# Phase 1: ARPG Core Systems Implementation Plan

> Created: 2026-09-25
> Scope: Minimum viable low-level systems for first playable combat loop
> Estimated: 2-3 weeks
> Dependencies: None (starts from current project state)

## Table of Contents

1. [System Dependencies](#system-dependencies)
2. [GameplayTags System](#gameplaytags-system)
3. [Health/Damage System](#healthdamage-system)
4. [AI State Machine](#ai-state-machine)
5. [Navigation](#navigation)
6. [Player Controller](#player-controller)
7. [Scene Setup](#scene-setup)
8. [MCP Tools](#mcp-tools)
9. [Implementation Order](#implementation-order)
10. [Testing Strategy](#testing-strategy)
11. [File Structure](#file-structure)
12. [Success Criteria](#success-criteria)
13. [Design Decisions & Defaults](#design-decisions--defaults)

---

## System Dependencies

```
GameplayTags (foundation)
    ↓
Health/Damage (uses tags)
    ↓
AI State Machine (uses damage, tags)
    ↓
Navigation (enables AI movement)
    ↓
Player Controller (enables gameplay)
    ↓
Scene + MCP Tools (integration)
```

---

## GameplayTags System

### Architecture

```rust
// Tag registry - single source of truth
#[derive(Resource, Default)]
pub struct TagRegistry {
    next_id: u32,
    name_to_id: HashMap<String, u32>,
    id_to_name: HashMap<u32, String>,
    parent_map: HashMap<u32, u32>,  // child_id -> parent_id
}

// Entity tags - stores IDs only
#[derive(Component, Default)]
pub struct GameplayTags {
    tag_ids: HashSet<u32>,
}
```

### Key Methods

| Method | Purpose |
|--------|---------|
| `add(tag_id: u32)` | Add tag by ID |
| `remove(tag_id: u32)` | Remove tag by ID |
| `has_tag(tag_id: u32) -> bool` | Check exact tag |
| `has_parent_tag(registry, parent_id) -> bool` | Check tag OR any descendant |
| `has_any_tag(tag_ids: &[u32]) -> bool` | Check if any match |
| `has_all_tags(tag_ids: &[u32]) -> bool` | Check if all match |
| `tag_names(registry) -> Vec<String>` | Get all tag names (debug) |

### Tag Hierarchy

```
Damage                          ← parent
└── Damage.Type                 ← parent
    ├── Damage.Type.Physical    ← leaf
    ├── Damage.Type.Fire        ← leaf
    ├── Damage.Type.Ice         ← leaf
    ├── Damage.Type.Lightning   ← leaf
    └── Damage.Type.Poison      ← leaf

Status                          ← parent
├── Status.Stunned              ← leaf
├── Status.Burning              ← leaf
├── Status.Frozen               ← leaf
└── Status.Poisoned             ← leaf

Faction                         ← parent
├── Faction.Player              ← leaf
├── Faction.Enemy               ← leaf
└── Faction.Neutral             ← leaf
```

### Automatic Inheritance

If entity has `Damage.Type.Fire`, it automatically matches:
- `has_tag(Damage.Type.Fire)` → true
- `has_parent_tag(Damage.Type)` → true
- `has_parent_tag(Damage)` → true

### Design Decisions

- **Format:** Dot-notation hierarchical strings (`Damage.Type.Fire`)
- **Storage:** String interning with u32 IDs (memory efficient, O(1) lookups)
- **Hierarchy:** Parent-child relationships with automatic inheritance
- **Pattern matching:** O(1) via hierarchy traversal
- **Performance:** O(1) exact match, O(depth) for parent queries (typically < 3)

---

## Health/Damage System

### Components

```rust
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
```

### Events

```rust
#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    pub damage_type: String,
    pub instigator: Option<Entity>,
    pub hit_location: Option<Vec3>,
}

#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
    pub killer: Option<Entity>,
}
```

### Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `apply_damage_system` | Update | Handles DamageEvent, reduces health, spawns DeathEvent |
| `death_cleanup_system` | Update | Despawns entities on DeathEvent |
| `health_regen_system` | Update | Optional: regenerates health over time |

### Design Decisions

- **Damage timing:** Instant only (no DoT yet)
- **Modifiers:** None yet (flat damage)
- **Death:** Instant despawn (no dying state)
- **Health regen:** None by default (can add later)

---

## AI State Machine

### Component

```rust
#[derive(Component)]
pub struct AIState {
    pub current: AIStateType,
    pub target_entity: Option<Entity>,
    pub state_timer: f32,
    pub last_known_position: Option<Vec3>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum AIStateType {
    Idle,
    Patrol { waypoints: Vec<Vec3>, current_index: usize },
    Chase,
    Attack { attack_cooldown: f32 },
    Dead,
}
```

### State Transitions

```
Idle ────────→ Patrol (after timer expires)
Patrol ──────→ Chase (if player detected)
Chase ───────→ Attack (if in attack range)
Chase ───────→ Patrol (if player lost for 3+ seconds)
Attack ──────→ Chase (if out of attack range)
Attack ──────→ Dead (if health <= 0)
Any ─────────→ Dead (if health <= 0)
Dead ─────────→ (terminal)
```

### Perception

```rust
#[derive(Component)]
pub struct AIPerception {
    pub detection_range: f32,
    pub attack_range: f32,
    pub field_of_view: f32,  // degrees, 360 = full circle
    pub detected_entities: Vec<Entity>,
    pub last_seen_position: Option<Vec3>,
    pub memory_duration: f32,  // how long to remember position after losing sight
}
```

### Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `ai_perception_system` | Update | Detects player via range/FOV check |
| `ai_state_machine_system` | Update | Transitions between states |
| `ai_behavior_system` | Update | Executes current state behavior |
| `ai_attack_system` | Update | Deals damage when attacking |

### Design Decisions

- **Memory:** Remembers last known position for 5 seconds
- **Communication:** None (AI act independently)
- **Personalities:** One type (aggressive: always chases when detected)
- **Transitions:** Timer-based with range checks
- **FOV:** 120 degrees (360 for full awareness, configurable)

---

## Navigation

### Component

```rust
#[derive(Component)]
pub struct NavAgent {
    pub target_position: Option<Vec3>,
    pub speed: f32,
    pub radius: f32,
    pub path: Vec<Vec3>,
    pub path_index: usize,
}
```

### Resource

```rust
#[derive(Resource)]
pub struct NavigationGrid {
    pub cell_size: f32,
    pub grid_width: i32,
    pub grid_height: i32,
    pub walkable: HashSet<(i32, i32)>,
}
```

### Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `nav_pathfind_system` | Update | Calculates A* path to target |
| `nav_move_system` | Update | Moves entity along path |

### A* Implementation

- Grid-based A* with 8-directional movement
- Cell size: 1.0 units
- Walkable cells determined at scene load
- Path smoothing: skip redundant intermediate points

### Design Decisions

- **Navigation type:** Grid-based A* (simpler, good for prototyping)
- **Dynamic obstacles:** None (static grid only)
- **Path smoothing:** Yes, skip collinear points
- **Cell size:** 1.0 world units
- **Max path length:** 100 cells (prevent excessive computation)

---

## Player Controller

### Component

```rust
#[derive(Component)]
pub struct PlayerController {
    pub move_speed: f32,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
    pub attack_timer: f32,
    pub damage_type: String,
}

#[derive(Component)]
pub struct PlayerInput {
    pub move_direction: Vec2,
    pub attack_pressed: bool,
}
```

### Events

```rust
#[derive(Event)]
pub struct PlayerAttackEvent {
    pub attacker: Entity,
    pub damage: f32,
    pub damage_type: String,
    pub direction: Vec3,
    pub range: f32,
}
```

### Systems

| System | Schedule | Purpose |
|--------|----------|---------|
| `player_input_system` | Update | Reads keyboard/mouse input |
| `player_movement_system` | Update | Moves player based on input |
| `player_attack_system` | Update | Handles attack input, spawns PlayerAttackEvent |
| `player_attack_resolve_system` | Update | Resolves attack, spawns DamageEvent for hit entities |

### Input Mapping

| Input | Action |
|-------|--------|
| W/A/S/D | Move forward/left/back/right |
| Mouse right + drag | Orbit camera (existing) |
| Left click | Attack |
| Space | Dash (future) |
| Shift | Sprint (future) |

### Design Decisions

- **Attack type:** Melee only (sphere cast in front of player)
- **Dodge/roll:** None yet (focus on attack first)
- **Camera:** Existing orbit camera controller
- **Movement:** Relative to camera facing direction
- **Attack cooldown:** 0.5 seconds

---

## Scene Setup

### Entities to Spawn

| Entity | Components | Position |
|--------|------------|----------|
| Player | PlayerController, Health, GameplayTags, Transform | (0, 0, 0) |
| Enemy 1 | AIState, AIPerception, NavAgent, Health, GameplayTags | (5, 0, 5) |
| Enemy 2 | AIState, AIPerception, NavAgent, Health, GameplayTags | (-5, 0, 5) |
| Enemy 3 | AIState, AIPerception, NavAgent, Health, GameplayTags | (0, 0, -8) |
| Ground | Mesh3d, MeshMaterial3d, Transform | (0, 0, 0) |
| Walls | Mesh3d, MeshMaterial3d, Transform, Obstacle | Various |

### Navigation Grid

- Grid bounds: 30x30 cells
- Walkable: All cells except wall positions
- Regenerated at scene load

### Design Decisions

- **Scene type:** Fixed test scene (hardcoded positions)
- **Enemy count:** 3 enemies (enough to test behavior)
- **Respawning:** None (one-shot encounter for now)
- **Procedural:** None (fixed layout)

---

## MCP Tools

### New Tools

| Tool | Args | Purpose |
|------|------|---------|
| `attack_entity` | attacker_name, target_name | Make one entity attack another |
| `set_health` | entity_name, health | Set entity health directly |
| `set_ai_state` | entity_name, state | Force AI state change |
| `spawn_enemy` | name, x, y, z | Spawn enemy at position |
| `spawn_player` | name, x, y, z | Spawn player at position |
| `get_health` | entity_name | Get entity current/max health |
| `get_ai_state` | entity_name | Get entity's current AI state |

### Design Decisions

- **Tool scope:** Key gameplay actions + debug tools
- **Debug tools:** Yes (get_health, get_ai_state, list_entities)
- **Error handling:** Return descriptive error messages

---

## Implementation Order

| Day | Task | Deliverable |
|-----|------|-------------|
| 1 | GameplayTags system | `src/gameplay_tags.rs` with unit tests |
| 2 | GameplayTags integration | Tags on existing entities, dashboard display |
| 3 | Health/Damage system | `src/combat.rs` with Health, DamageEvent |
| 4 | Damage integration | MCP tools for damage, testing |
| 5 | AI state machine | `src/ai.rs` with state transitions |
| 6 | AI perception | Sight detection, FOV checks |
| 7 | AI behavior + testing | Patrol/Chase/Attack working |
| 8 | Navigation grid | `src/navigation.rs` with A* |
| 9 | Navigation integration | Enemies move along paths |
| 10 | Player controller | `src/player.rs` with WASD movement |
| 11 | Player attack | Melee attack with DamageEvent |
| 12 | Scene setup | Full test scene with player + enemies |
| 13 | Integration testing | Full combat loop, bug fixes |
| 14 | MCP tools | New tools for gameplay control |
| 15 | Polish + documentation | Clean code, update docs |

---

## Testing Strategy

### Unit Tests

| System | Tests |
|--------|-------|
| GameplayTags | add, remove, has, has_any, has_all, pattern matching |
| Health/Damage | apply damage, health < 0 → death, death event |
| AI State Machine | all state transitions, timer behavior |
| Navigation | A* path correctness, obstacle avoidance |
| Player | input reading, movement direction, attack cooldown |

### Integration Tests

| Scenario | Expected Result |
|----------|-----------------|
| Player attacks enemy | Enemy health decreases, DeathEvent if health <= 0 |
| Enemy detects player | State changes to Chase |
| Enemy reaches attack range | State changes to Attack, damage dealt |
| Enemy loses sight of player | State returns to Patrol after memory expires |
| Player moves with WASD | Player moves in correct direction |
| Enemy navigates around walls | Path avoids non-walkable cells |

### Manual Testing Checklist

- [ ] Player can move with WASD
- [ ] Player can attack enemies with left click
- [ ] Enemies patrol between waypoints
- [ ] Enemies chase player when in detection range
- [ ] Enemies attack player when in attack range
- [ ] Damage reduces health correctly
- [ ] Death despawns entities
- [ ] Navigation works with obstacles
- [ ] MCP tools work for all new tools
- [ ] Game runs at 60 FPS with 5+ entities

---

## File Structure

```
ai_native_engine/src/
├── main.rs                    # Entry point, plugin registration
├── scene.rs                   # Scene setup (modify existing)
├── entity_registry.rs         # Named entities (existing)
├── mcp_server.rs              # MCP tools (modify existing)
├── dashboard.rs               # UI panels (existing)
├── camera_controller.rs       # Camera controls (existing)
├── gameplay_tags.rs           # NEW: GameplayTags system
├── combat.rs                  # NEW: Health, damage, death
├── ai.rs                      # NEW: AI state machine, perception
├── navigation.rs              # NEW: Navigation grid, A*
├── player.rs                  # NEW: Player controller, input
└── tests/                     # NEW: Integration tests
    ├── gameplay_tags_test.rs
    ├── combat_test.rs
    ├── ai_test.rs
    ├── navigation_test.rs
    └── player_test.rs
```

### Cargo.toml Additions

```toml
[dependencies]
# No new dependencies — all systems use existing Bevy + stdlib
```

---

## Success Criteria

### Functional
- [x] Player can move with WASD
- [x] Player can attack enemies
- [x] Enemies can patrol between waypoints
- [x] Enemies can chase player when detected
- [x] Enemies can attack player when in range
- [x] Damage reduces health
- [x] Death despawns entities
- [x] Navigation works with obstacles
- [x] MCP tools work for AI control

### Performance
- [x] Game runs at 60 FPS with 5+ entities
- [x] No frame hitches from MCP commands
- [x] Pathfinding completes in < 5ms

### Code Quality
- [x] All new systems have unit tests
- [x] Integration tests pass
- [x] Documentation updated (FEATURES.md, NEXT_STEPS.md)
- [x] Clean build with no warnings

---

## Design Decisions & Defaults

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Tag format | Dot-notation strings | Simple, debuggable, extensible |
| Tag storage | String interning + u32 IDs | Memory efficient, O(1) lookups |
| Tag inheritance | Automatic via hierarchy | Parent queries work automatically |
| Damage timing | Instant | No DoT complexity yet |
| Damage modifiers | None | Flat damage first |
| Death state | Instant despawn | No dying animation yet |
| AI memory | 5 seconds | Balance between smart and simple |
| AI communication | None | Independent AI |
| AI personalities | One (aggressive) | Add variety later |
| Navigation type | Grid-based A* | Simpler, good for prototype |
| Dynamic obstacles | None | Static grid only |
| Path smoothing | Yes | Skip collinear points |
| Attack type | Melee only | Ranged later |
| Dodge/roll | None | Focus on attack first |
| Camera | Orbit (existing) | Already working |
| Scene type | Fixed | Procedural later |
| Enemy count | 3 | Enough to test |
| Respawning | None | One-shot for now |
| HUD | None | Focus on gameplay |
| Sound | None | Add after gameplay |
| Particles | None | Add after gameplay |
| MCP tool scope | Key + debug | Enough for AI control |

### Adjustable Parameters

| Parameter | Default | Range | Purpose |
|-----------|---------|-------|---------|
| Player move speed | 5.0 | 1.0-15.0 | Movement speed |
| Player attack damage | 25.0 | 1.0-100.0 | Damage per hit |
| Player attack range | 2.0 | 0.5-5.0 | Melee range |
| Player attack cooldown | 0.5 | 0.1-2.0 | Time between attacks |
| Enemy detection range | 10.0 | 1.0-30.0 | Sight range |
| Enemy attack range | 1.5 | 0.5-3.0 | Melee range |
| Enemy attack damage | 10.0 | 1.0-50.0 | Damage per hit |
| Enemy attack cooldown | 1.0 | 0.2-3.0 | Time between attacks |
| Enemy patrol speed | 3.0 | 1.0-10.0 | Patrol movement speed |
| Enemy chase speed | 5.0 | 1.0-10.0 | Chase movement speed |
| Nav grid cell size | 1.0 | 0.5-2.0 | Navigation resolution |
| AI memory duration | 5.0 | 0.0-30.0 | How long to remember position |

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| A* performance on large grids | Medium | Low | Limit grid to 30x30, add early exit |
| AI state machine complexity | Medium | Medium | Start simple, add states incrementally |
| Bevy 0.19 API incompatibility | Low | High | Check API docs before each system |
| MCP blocking lock hitches | Low | Medium | Fix async channel when needed |
| Scope creep | High | High | Stick to minimum viable systems |

---

## Next Steps After Phase 1

1. **Phase 2: Expand Systems (1-2 months)**
   - Status effects (using GameplayTags)
   - Abilities system with costs/cooldowns
   - Inventory and loot
   - Better AI behavior trees

2. **Phase 3: Content & Polish (1-2 months)**
   - Asset pipeline (glTF models)
   - Animation system
   - Sound effects
   - Visual effects

3. **Phase 4: Jev AI Integration (2-3 months)**
   - AI-generated content
   - AI playtesting
   - Natural language scene creation

---

*End of Phase 1 implementation plan.*
