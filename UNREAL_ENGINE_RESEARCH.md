# Unreal Engine 4.22 Architecture Research for AI-Native Engine

> Source repository: https://github.com/folgerwang/UnrealEngine (UE 4.22 mirror)
> Research date: 2026-09-23
> Purpose: Extract useful patterns, systems, and architectural decisions from Unreal Engine 4.22 for the Bevy-based AI-Native Engine project.

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Core Engine Architecture](#core-engine-architecture)
3. [Game Framework](#game-framework)
4. [AI Systems (Core Focus)](#ai-systems-core-focus)
5. [Gameplay Systems (Core Focus)](#gameplay-systems-core-focus)
6. [Navigation & Pathfinding](#navigation--pathfinding)
7. [Animation](#animation)
8. [Rendering & Camera](#rendering--camera)
9. [Physics & Collision](#physics--collision)
10. [Audio](#audio)
11. [Input](#input)
12. [UI Systems](#ui-systems)
13. [Networking](#networking)
14. [Materials & Shaders](#materials--shaders)
15. [Particle Systems & VFX](#particle-systems--vfx)
16. [Logging, Profiling & Debugging](#logging-profiling--debugging)
17. [Data Tables & Curves](#data-tables--curves)
18. [Localization](#localization)
19. [Landscape & Foliage](#landscape--foliage)
20. [Multiplayer & Replication Details](#multiplayer--replication-details)
21. [Editor & Tools](#editor--tools)
22. [Build System & Modules](#build-system--modules)
23. [Serialization & Save/Load](#serialization--saveload)
24. [Asset Pipeline](#asset-pipeline)
25. [AI-Native Engine Adaptation Roadmap](#ai-native-engine-adaptation-roadmap)
26. [File & Module Index](#file--module-index)

---

## Executive Summary

Unreal Engine 4.22 is a monolithic game engine built on C++ with a component-based object model (UObject/AActor/UActorComponent), extensive reflection, and a modular build system. For the AI-Native Engine project, the most valuable areas are:

- **AI Module**: Behavior trees, perception, navigation, environment queries
- **GameplayTags**: Hierarchical tag system for damage types, status effects, abilities
- **GameFramework**: Actor/Component patterns, controllers, damage system, character movement
- **NavigationSystem**: Pathfinding, navmesh, crowd avoidance
- **Engine Subsystems**: Serialization, events, timers, input mapping
- **Editor Systems**: Viewport controls, property editors, content browser patterns

This document maps UE4 patterns to Bevy/Rust equivalents and provides implementation guidance for the AI-Native Engine.

---

## Core Engine Architecture

### Coordinate System

UE4 uses a **left-handed, Z-up** coordinate system:

| Axis | Direction | Color |
|------|-----------|-------|
| X | Forward | Red |
| Y | Right | Green |
| Z | Up | Blue |

This differs from Bevy's **right-handed, Y-up** system. When adapting UE patterns:
- Keep Bevy's coordinate system internally
- Map control schemes (like viewport navigation) rather than coordinate values
- Forward in Bevy is typically `-Z`, right is `+X`, up is `+Y`

### UObject System

**Location:** `Engine/Source/Runtime/CoreUObject/` and `Engine/Source/Runtime/Core/`

UObject is the base class for most engine objects. Key features:
- **Reflection**: UPROPERTY/UCLASS/UFUNCTION macros expose properties to editor and Blueprint
- **Garbage Collection**: Mark-and-sweep GC for UObject-derived types
- **Serialization**: UObject can be serialized to/from disk
- **Lifecycle**: Spawn/Destroy pattern with BeginPlay/EndPlay/Tick

**Bevy Equivalent:**
- Bevy ECS entities/components replace UObject
- `Component` derives replace UPROPERTY
- Bevy's `App` plugin system replaces module-level lifecycle
- No built-in reflection; use `bevy_reflect` crate for similar functionality

### Module System

**Location:** `Engine/Source/Runtime/*/Build.cs`

UE4 is organized into modules with `.Build.cs` files declaring:
- Module name and type (Runtime/Editor)
- Public/private dependencies
- Include paths
- Precompiled headers

Example structure:
```csharp
public class AIModule : ModuleRules
{
    public AIModule(ReadOnlyTargetRules Target) : base(Target)
    {
        PublicDependencyModuleNames.AddRange(new string[] { "Core", "CoreUObject", "Engine", "GameplayTasks" });
    }
}
```

**Bevy Equivalent:**
- Cargo workspace members or separate crates
- `Cargo.toml` dependencies
- Bevy plugins for runtime modules

### Reflection & Serialization

**Location:** `Engine/Source/Runtime/CoreUObject/Public/UObject/`

Key files:
- `Class.h` - UClass metadata
- `Property.h` - UProperty reflection
- `Object.h` - UObject base
- `ScriptMacros.h` - UPROPERTY/UCLASS macros

**Adaptation Notes:**
- Use `bevy_reflect` for runtime reflection
- Use `serde` for serialization (already in project)
- Consider `TypeRegistry` for component discovery

---

## Game Framework

**Location:** `Engine/Source/Runtime/Engine/Classes/GameFramework/`

### Actor / Component Pattern

**Key Classes:**
- `AActor` - Base class for all placeable objects
- `UActorComponent` - Base class for components
- `USceneComponent` - Component with transform
- `UPrimitiveComponent` - Component that can be rendered

**AActor Responsibilities:**
- Owns components
- Has RootComponent with world transform
- Lifecycle: BeginPlay, Tick, EndPlay
- Replication for multiplayer

**UActorComponent Responsibilities:**
- Modular functionality
- Can tick independently
- Register/unregister with owner

**Bevy Equivalent:**
```rust
// UE4 Actor with components becomes Bevy entity with components
commands.spawn((
    Name::new("Enemy"),
    Transform::default(),
    Health { current: 100.0, max: 100.0 },
    Weapon { damage: 10.0 },
));
```

### Pawn / Controller Pattern

**Key Classes:**
- `APawn` - Actor that can be possessed
- `AController` - AI or player controller
- `APlayerController` - Player input, camera management
- `AAIController` - AI logic

**Pattern:**
- Controller "possesses" a Pawn
- Controller handles input/AI decisions
- Pawn handles movement/abilities
- Separation allows AI and player to control same character

**Bevy Equivalent:**
```rust
#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct AIControlled;

#[derive(Component)]
pub struct Controller {
    pub controlled_entity: Entity,
}
```

### Character & Movement

**Key Classes:**
- `ACharacter` - Pawn with skeletal mesh and movement
- `UCharacterMovementComponent` - Advanced movement physics
- `UFloatingPawnMovement` - Simple floating movement
- `UProjectileMovementComponent` - Projectile physics
- `USpringArmComponent` - Camera boom

**CharacterMovementComponent Features:**
- Walking/running/falling physics
- Jumping
- Crouching
- Swimming
- Flying
- Network prediction
- Root motion integration

**Adaptation Notes:**
- Use Bevy's physics integration (Rapier/Avian) for movement
- Or implement custom CharacterController component
- SpringArm component pattern useful for third-person camera

### Damage System

**Key Files:**
- `Engine/Source/Runtime/Engine/Classes/GameFramework/DamageType.h`
- `Engine/Source/Runtime/Engine/Classes/GameFramework/Actor.h`

**UDamageType Properties:**
```cpp
UCLASS(MinimalAPI, const, Blueprintable, BlueprintType)
class UDamageType : public UObject
{
    uint32 bCausedByWorld:1;
    uint32 bScaleMomentumByMass:1;
    uint32 bRadialDamageVelChange:1;
    float DamageImpulse;
    float DestructibleImpulse;
    float DestructibleDamageSpreadScale;
    float DamageFalloff;
};
```

**Pattern:**
- DamageType is a class, not just an enum
- Damage events carry type, amount, instigator, hit location
- Actors implement `TakeDamage` virtual function
- Supports radial damage, point damage, generic damage

**Bevy Equivalent:**
```rust
#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    pub damage_type: DamageType,
    pub instigator: Option<Entity>,
    pub hit_location: Option<Vec3>,
}

#[derive(Clone)]
pub struct DamageType {
    pub name: String,
    pub impulse: f32,
    pub falloff: f32,
}
```

### GameMode / GameState / PlayerState

**Key Classes:**
- `AGameModeBase` - Rules and flow of the game
- `AGameStateBase` - Replicated game state
- `APlayerState` - Per-player replicated state

**Pattern:**
- GameMode is server-authoritative
- GameState replicates to all clients
- PlayerState tracks score, kills, team, etc.

**Bevy Equivalent:**
```rust
#[derive(Resource)]
pub struct GameMode {
    pub current_phase: GamePhase,
    pub rules: GameRules,
}

#[derive(Resource)]
pub struct GameState {
    pub players: HashMap<Entity, PlayerState>,
}
```

---

## AI Systems (Core Focus)

**Location:** `Engine/Source/Runtime/AIModule/`

### Architecture Overview

The AI Module provides:
1. **Behavior Trees** - Hierarchical decision making
2. **Blackboard** - Shared AI memory
3. **Perception System** - Senses (sight, hearing, damage, etc.)
4. **Navigation** - Pathfinding integration
5. **AI Controller** - AI-specific pawn controller
6. **Environment Query System (EQS)** - Spatial queries
7. **Gameplay Tasks** - Async AI actions

### Behavior Trees

**Location:** `Engine/Source/Runtime/AIModule/Classes/BehaviorTree/`

**Key Classes:**
- `UBehaviorTree` - Asset containing the tree
- `UBTCompositeNode` - Branch nodes (Selector, Sequence)
- `UBTDecorator` - Conditions that control execution
- `UBTService` - Nodes that tick while branch is active
- `UBTTaskNode` - Leaf actions

**Node Types:**
- **Selector** - Tries children until one succeeds
- **Sequence** - Executes children until one fails
- **Parallel** - Runs children simultaneously
- **Simple Parallel** - Runs main task and background branch

**Standard Decorators:**
- BlackboardBasedCondition
- Cooldown
- ForceSuccess
- Loop
- TimeLimit

**Standard Tasks:**
- MoveTo
- RotateToFaceBBEntry
- Wait
- RunBehaviorTree (subtrees)

**Blackboard Integration:**
```cpp
class UBlackboardData : public UDataAsset
{
    TArray<FBTTextKeyword> TextKeywords;
    TArray<FBlackboardEntry> Keys;
    UBlackboardData* Parent;
};
```

**Bevy Adaptation:**
```rust
// Simplified behavior tree
pub enum BTNode {
    Sequence(Vec<BTNode>),
    Selector(Vec<BTNode>),
    Task(Box<dyn Fn(&mut Commands, Entity, &Blackboard) -> BTStatus>),
    Decorator(Box<dyn Fn(&Blackboard) -> bool>, Box<BTNode>),
}

pub enum BTStatus {
    Success,
    Failure,
    Running,
}

#[derive(Component)]
pub struct BehaviorTree {
    pub root: BTNode,
    pub blackboard: Blackboard,
}
```

### Perception System

**Location:** `Engine/Source/Runtime/AIModule/Classes/Perception/`

**Key Classes:**
- `UAIPerceptionComponent` - Collects sensory data
- `UAIPerceptionStimuliSourceComponent` - Emits stimuli
- `UAIPerceptionSystem` - Global perception manager
- `UAISense` - Base class for senses
- `UAISenseConfig` - Configuration for senses

**Sense Types:**
| Sense | Purpose | Key Config |
|-------|---------|------------|
| Sight | Vision cone detection | Range, angle, peripheral vision |
| Hearing | Sound detection | Range, noise tags |
| Damage | React to damage events | None |
| Touch | Physical contact | None |
| Team | Team awareness | Team ID |
| Prediction | Predict target location | None |

**UAISenseConfig_Sight Properties:**
- SightRadius
- LoseSightRadius
- PeripheralVisionAngleDegrees
- DetectionByAffiliation (hostile/neutral/friendly)
- AutoSuccessRangeFromLastSeenLocation

**Perception Events:**
- OnPerceptionUpdated
- OnTargetPerceptionUpdated
- Stimulus has age, strength, location

**Bevy Adaptation:**
```rust
#[derive(Component)]
pub struct PerceptionComponent {
    pub senses: Vec<SenseConfig>,
    pub known_entities: Vec<KnownEntity>,
}

pub enum SenseConfig {
    Sight { range: f32, angle: f32 },
    Hearing { range: f32 },
    Damage,
}

pub struct KnownEntity {
    pub entity: Entity,
    pub last_known_position: Vec3,
    pub last_seen_time: f32,
    pub confidence: f32,
}
```

### AI Controller

**Location:** `Engine/Source/Runtime/AIModule/Classes/AIController.h`

**Key Features:**
- Possesses pawns
- Runs behavior trees
- Uses perception
- Integrates with navigation
- Can control path following

**Key Functions:**
- `RunBehaviorTree(BehaviorTree)`
- `SetFocalPoint()`
- `ClearFocus()`
- `MoveToLocation()`
- `MoveToActor()`

### Environment Query System (EQS)

**Location:** `Engine/Source/Runtime/AIModule/Classes/EnvironmentQuery/`

**Purpose:** Find best positions in space based on tests.

**Key Concepts:**
- **Generator** - Produces candidate points (circle, grid, etc.)
- **Test** - Scores/filters points (distance, visibility, etc.)
- **Context** - Reference points (querier, target, etc.)

**Use Cases:**
- Find cover
- Find flanking position
- Find nearest health pickup
- Find attack position

**Bevy Adaptation:**
```rust
pub struct EnvironmentQuery {
    pub generator: Box<dyn Generator>,
    pub tests: Vec<Box<dyn Test>>,
    pub context: QueryContext,
}

pub trait Generator {
    fn generate(&self, context: &QueryContext) -> Vec<Vec3>;
}

pub trait Test {
    fn score(&self, point: Vec3, context: &QueryContext) -> f32;
}
```

### AI Tasks

**Location:** `Engine/Source/Runtime/AIModule/Classes/Tasks/`

Tasks are leaf nodes in behavior trees that perform actions:
- `AITask_MoveTo`
- `AITask_RunEQS`
- `AITask_Wait`

They support latent/async execution and can be aborted.

---

## Gameplay Systems (Core Focus)

### GameplayTags

**Location:** `Engine/Source/Runtime/GameplayTags/`

**Key Classes:**
- `FGameplayTag` - Single hierarchical tag
- `FGameplayTagContainer` - Collection of tags
- `UGameplayTagsManager` - Global tag registry
- `IGameplayTagAssetInterface` - Interface for tagged assets

**Tag Format:**
```
Damage.Type.Fire
Damage.Type.Ice
Status.Effect.Stunned
Status.Effect.Burning
Ability.Melee.Sword
Ability.Ranged.Bow
```

**Container Operations:**
- `HasTag(Tag)`
- `HasAny(Container)`
- `HasAll(Container)`
- `AddTag(Tag)`
- `RemoveTag(Tag)`
- `AppendTags(Container)`
- `Reset()`

**Key Use Cases:**
- Damage type classification
- Status effects
- Ability categorization
- Team/ faction identification
- Weapon types
- Animation states
- Buff/debuff tracking

**Bevy Adaptation:**
```rust
#[derive(Component, Default, Clone)]
pub struct GameplayTags {
    tags: HashSet<String>,
}

impl GameplayTags {
    pub fn add(&mut self, tag: impl Into<String>) { self.tags.insert(tag.into()); }
    pub fn remove(&mut self, tag: &str) { self.tags.remove(tag); }
    pub fn has(&self, tag: &str) -> bool { self.tags.contains(tag) }
    pub fn has_any(&self, tags: &[&str]) -> bool { tags.iter().any(|t| self.has(t)) }
    pub fn has_all(&self, tags: &[&str]) -> bool { tags.iter().all(|t| self.has(t)) }
    pub fn matches(&self, pattern: &str) -> bool {
        // Supports wildcard matching like "Damage.Type.*"
        self.tags.iter().any(|tag| tag_matches(tag, pattern))
    }
}
```

### GameplayAbilities (Plugin)

**Note:** GameplayAbilities is a separate plugin in UE4, not in the base runtime. However, the patterns are worth documenting.

**Key Concepts:**
- **Ability** - A thing an actor can do (spell, attack, etc.)
- **GameplayEffect** - Modifies attributes over time
- **AttributeSet** - Health, mana, stamina, etc.
- **GameplayCue** - Visual/audio feedback
- **Cost** - Mana/stamina requirements
- **Cooldown** - Time between uses

**Attribute System Pattern:**
```rust
#[derive(Component)]
pub struct HealthAttribute {
    pub base_value: f32,
    pub current_value: f32,
    pub modifiers: Vec<AttributeModifier>,
}

pub struct AttributeModifier {
    pub source: Option<Entity>,
    pub value: f32,
    pub operation: ModifierOp,  // Add, Multiply, Override
    pub duration: Option<f32>,
}
```

### Damage System with Tags

Combining DamageType and GameplayTags:
```rust
#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    pub damage_type: String,  // "Damage.Type.Fire"
    pub instigator: Option<Entity>,
    pub hit_location: Option<Vec3>,
    pub tags: GameplayTags,   // Additional context tags
}
```

### Status Effect System

**Pattern:**
- Status effects are gameplay tags with duration
- Systems query for tags and apply effects
- Effects can stack, refresh, or expire

```rust
#[derive(Component)]
pub struct StatusEffect {
    pub tag: String,
    pub duration: f32,
    pub remaining: f32,
    pub stacks: u32,
    pub source: Option<Entity>,
}

// Systems:
// - status_effect_tick_system: decreases remaining time
// - status_effect_expiration_system: removes expired effects
// - burning_damage_system: applies damage to burning entities
// - stun_system: prevents action when stunned
```

---

## Navigation & Pathfinding

**Location:** `Engine/Source/Runtime/NavigationSystem/` and `Engine/Source/Runtime/Navmesh/`

### Key Components

- **UNavigationSystemV1** - Main navigation system
- **UNavMeshBoundsVolume** - Defines navmesh generation bounds
- **UNavigationPath** - Result of path query
- **UNavArea** - Cost modifiers for navmesh areas
- **UCrowdFollowingComponent** - Crowd avoidance

### Path Following

**Location:** `Engine/Source/Runtime/AIModule/Classes/Navigation/`

- `UPathFollowingComponent` - Follows a navigation path
- `UNavLinkProxy` - Special movement links (jumps, climbs)

### Recast/Detour Integration

UE4 integrates Recast for navmesh generation and Detour for pathfinding. Bevy ecosystem alternatives:
- `bevy_pathmesh` or `bevy_rapier_navmesh`
- `oxidized_navigation`
- Custom A* on grid/mesh

### Bevy Adaptation

```rust
#[derive(Component)]
pub struct NavMeshAgent {
    pub radius: f32,
    pub height: f32,
    pub max_speed: f32,
    pub current_path: Vec<Vec3>,
    pub current_path_index: usize,
}

#[derive(Resource)]
pub struct NavigationSystem {
    pub navmesh: NavMesh,
}

pub fn pathfind_system(
    nav: Res<NavigationSystem>,
    mut agents: Query<(Entity, &mut NavMeshAgent, &Transform)>,
    targets: Query<&Transform, With<PathTarget>>,
) {
    // Find paths for agents with targets
}
```

---

## Animation

**Location:** `Engine/Source/Runtime/Engine/Classes/Animation/` and `Engine/Source/Runtime/AnimGraphRuntime/`

### Key Systems

- **USkeletalMeshComponent** - Renders animated skeletal mesh
- **UAnimInstance** - Animation blueprint runtime
- **UAnimSequence** - Single animation clip
- **UAnimMontage** - Composite animation with sections and events
- **UBlendSpace** - 2D/1D animation blending by parameters
- **UAnimStateMachine** - State-based animation
- **UAnimNotify** - Events fired during animation

### Animation Blueprint Pattern

```cpp
class UMyAnimInstance : public UAnimInstance
{
    UPROPERTY(BlueprintReadOnly)
    float Speed;
    
    UPROPERTY(BlueprintReadOnly)
    bool bIsInAir;
    
    virtual void NativeUpdateAnimation(float DeltaSeconds) override;
};
```

### Bevy Adaptation

Bevy 0.19 animation support is limited. Options:
- Use `bevy_animation` crate (experimental)
- Use `bevy_gltf` with skinned meshes
- Integrate `machine_learning` or custom animation state machine

```rust
#[derive(Component)]
pub struct AnimationStateMachine {
    pub states: HashMap<String, AnimationState>,
    pub current_state: String,
    pub transitions: Vec<AnimationTransition>,
}

pub struct AnimationState {
    pub clip: Handle<AnimationClip>,
    pub speed: f32,
    pub loop_type: LoopType,
}
```

### Root Motion

**Location:** `Engine/Source/Runtime/Engine/Classes/GameFramework/RootMotionSource.h`

Root motion drives movement from animation. Useful for:
- Melee attack lunges
- Dodges
- Scripted movement

---

## Rendering & Camera

**Location:** `Engine/Source/Runtime/Engine/Classes/Camera/` and `Engine/Source/Runtime/Renderer/`

### Camera System

**Key Classes:**
- `UCameraComponent` - Scene component camera
- `ACameraActor` - Standalone camera actor
- `APlayerCameraManager` - Manages active camera, transitions, effects
- `USpringArmComponent` - Camera boom with collision
- `UCameraModifier` - Camera shake/modifiers

**UCameraComponent Features:**
- Field of view
- Orthographic/perspective projection
- Aspect ratio
- Post-process settings
- Additive offsets
- LOD considerations

**SpringArmComponent Pattern:**
- Maintains distance from target
- Collides with world geometry
- Smoothly interpolates position

### Post-Processing

**Location:** `Engine/Source/Runtime/Engine/Classes/Engine/Scene.h`

`FPostProcessSettings` includes:
- Bloom
- Motion blur
- Depth of field
- Auto exposure
- Color grading
- Ambient occlusion
- Screen space reflections

### Lighting

**Location:** `Engine/Source/Runtime/Engine/Classes/Components/`

- `ULightComponent`
- `UDirectionalLightComponent`
- `UPointLightComponent`
- `USpotLightComponent`
- `USkyLightComponent`

Bevy equivalents exist in `bevy_pbr`.

### Bevy Adaptation

Already implemented: orbit camera controller.
Future additions:
```rust
#[derive(Component)]
pub struct CameraSpringArm {
    pub target: Entity,
    pub desired_distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub collision_radius: f32,
    pub lag_speed: f32,
}
```

---

## Physics & Collision

**Location:** `Engine/Source/Runtime/Engine/Classes/PhysicsEngine/` and `Engine/Source/Runtime/PhysXCooking/`

### Key Components

- **UPrimitiveComponent** - Base for physics/collision
- **UStaticMeshComponent** - Static mesh rendering/collision
- **USkeletalMeshComponent** - Skeletal mesh with physics assets
- **UCapsuleComponent** - Common character collision
- **USphereComponent** - Sphere collision
- **UBoxComponent** - Box collision

### Collision Settings

UE4 uses collision channels:
- Visibility
- Camera
- WorldStatic
- WorldDynamic
- Pawn
- Vehicle
- PhysicsBody
- Destructible

Collision responses: Block, Overlap, Ignore

### Physics Integration

UE4 uses PhysX (in 4.22). Bevy alternatives:
- `bevy_rapier3d` - Rust physics engine
- `bevy_xpbd` / `avian3d` - XPBD-based physics
- `heron` - Simpler physics wrapper

### Bevy Adaptation

```rust
#[derive(Component)]
pub struct CollisionProfile {
    pub channel: CollisionChannel,
    pub responses: HashMap<CollisionChannel, CollisionResponse>,
}

pub enum CollisionChannel {
    WorldStatic,
    WorldDynamic,
    Pawn,
    Weapon,
    Projectile,
}

pub enum CollisionResponse {
    Block,
    Overlap,
    Ignore,
}
```

---

## Audio

**Location:** `Engine/Source/Runtime/Engine/Classes/Sound/` and `Engine/Source/Runtime/AudioMixer/`

### Key Systems

- **USoundBase** - Base sound asset
- **USoundCue** - Composed sound with modifiers
- **USoundWave** - Raw audio
- **UAudioComponent** - Plays sounds in world
- **USoundAttenuation** - Spatial audio settings
- **USoundConcurrency** - Limits simultaneous sounds

### Sound Cue Nodes

- WavePlayer
- Mixer
- Random
- Concatenator
- Modulator
- Delay
- Branch

### Bevy Adaptation

Bevy has `bevy_audio` with `AudioSource`, `AudioSink`, spatial audio.

```rust
#[derive(Component)]
pub struct AudioEmitter {
    pub sound: Handle<AudioSource>,
    pub volume: f32,
    pub pitch: f32,
    pub looped: bool,
    pub spatial: bool,
    pub attenuation: f32,
}
```

---

## Input

**Location:** `Engine/Source/Runtime/Engine/Classes/GameFramework/PlayerInput.h` and `Engine/Source/Runtime/InputCore/`

### Input Mapping

UE4 uses:
- **Action Mappings** - Digital actions (jump, fire)
- **Axis Mappings** - Analog input (move forward, look up)
- **InputComponent** - Binds actions/axes to functions

### Enhanced Input (Newer UE)

Though not fully in 4.22, the concept exists:
- Input Actions as assets
- Mapping Contexts
- Modifiers (invert, scale, deadzone)
- Triggers (pressed, released, hold, tap)

### Bevy Adaptation

Already using Bevy's `ButtonInput<KeyCode>` and `MouseMotion`. For more complex mapping:

```rust
#[derive(Resource)]
pub struct InputMapping {
    pub actions: HashMap<String, Vec<InputBinding>>,
    pub axes: HashMap<String, Vec<AxisBinding>>,
}

pub enum InputBinding {
    Key(KeyCode),
    MouseButton(MouseButton),
    GamepadButton(GamepadButton),
}
```

---

## UI Systems

**Location:** `Engine/Source/Runtime/Engine/Classes/Slate/` and `Engine/Source/Runtime/UMG/`

### Slate

Slate is UE4's low-level UI framework (C++).

### UMG (Unreal Motion Graphics)

UMG is the visual UI editor built on Slate.

**Key Classes:**
- `UUserWidget` - Base widget class
- `UWidgetComponent` - 3D world-space UI
- `UCanvasPanel` - Layout container
- `UHorizontalBox`, `UVerticalBox` - Linear layouts
- `UTextBlock` - Text display
- `UButton` - Button widget
- `UProgressBar` - Health/mana bars
- `UImage` - Image display

### Bevy Adaptation

Already using `bevy_egui` for editor/dashboard UI. For in-game UI:
- Continue using egui
- Or use `bevy_ui` for game UI
- Or integrate `sickle_ui`/`bevy_lunex`

---

## Networking

**Location:** `Engine/Source/Runtime/Engine/Classes/Net/` and `Engine/Source/Runtime/Networking/`

### Key Concepts

- **Replication** - Automatic property synchronization
- **RPCs** - Remote Procedure Calls
  - Server RPC
  - Client RPC
  - Multicast RPC
- **Network Prediction** - Client-side prediction for movement
- **Relevancy** - Only replicate actors near players
- **Dormancy** - Pause replication for inactive actors

### Actor Replication

```cpp
UCLASS()
class AMyActor : public AActor
{
    GENERATED_BODY()
    
    UPROPERTY(Replicated)
    float Health;
    
    UFUNCTION(Server, Reliable, WithValidation)
    void ServerFireWeapon(FVector Target);
};
```

### Bevy Adaptation

For future multiplayer:
- Use `bevy_replicon` or `bevy_net`
- Implement server-authoritative patterns
- Use Bevy events for RPC-like communication

---

## Editor & Tools

**Location:** `Engine/Source/Editor/`

### Editor Viewport

**Location:** `Engine/Source/Editor/UnrealEd/Private/EditorViewportClient.cpp`

Key features:
- Multiple viewport layouts
- Real-time rendering
- Gizmo manipulation
- Camera navigation modes
- Selection/highlighting

### Property Editor

**Location:** `Engine/Source/Editor/PropertyEditor/`

- Inspects UObject properties via reflection
- Custom property editors
- Details panel

### Content Browser

**Location:** `Engine/Source/Editor/ContentBrowser/`

- Asset discovery and management
- Thumbnail generation
- Folder hierarchy
- Filter/search

### Blueprint Editor

**Location:** `Engine/Source/Editor/Kismet/`

- Visual scripting
- Node graph editor
- Compilation to bytecode

### Bevy Adaptation

Current dashboard can be extended:
- Scene hierarchy (outliner)
- Property editor with component fields
- Asset browser
- Console/command window
- Visual scripting nodes (long-term)

---

## Build System & Modules

### Build.cs Files

Every module has a `.Build.cs` file:
```csharp
public class AIModule : ModuleRules
{
    public AIModule(ReadOnlyTargetRules Target) : base(Target)
    {
        PublicDependencyModuleNames.AddRange(
            new string[] { "Core", "CoreUObject", "Engine", "GameplayTasks" }
        );
        PrivateDependencyModuleNames.AddRange(
            new string[] { "Projects" }
        );
    }
}
```

### Plugin System

Plugins extend engine functionality:
- Content plugins
- Code plugins
- Editor plugins
- Mixed plugins

Plugin descriptor: `.uplugin` file

### Bevy Equivalent

- Cargo workspace crates
- Bevy plugins
- Feature flags in Cargo.toml

---

## Serialization & Save/Load

**Location:** `Engine/Source/Runtime/Core/Public/Serialization/`

### UObject Serialization

- `FArchive` - Base serialization stream
- `Serialize(FArchive& Ar)` virtual function
- Text/binary formats
- Delta serialization for networking

### SaveGame System

**Location:** `Engine/Source/Runtime/Engine/Classes/GameFramework/SaveGame.h`

```cpp
class USaveGame : public UObject
{
    UPROPERTY()
    FString PlayerName;
    
    UPROPERTY()
    int32 UserIndex;
};

// Save
UGameplayStatics::SaveGameToSlot(SaveGameInstance, "SaveSlot", 0);

// Load
USaveGame* LoadedGame = UGameplayStatics::LoadGameFromSlot("SaveSlot", 0);
```

### Bevy Adaptation

Use serde with custom save format:
```rust
#[derive(Serialize, Deserialize)]
pub struct GameSaveData {
    pub version: u32,
    pub player: PlayerSaveData,
    pub entities: Vec<EntitySaveData>,
    pub inventory: InventorySaveData,
}
```

---

## Materials & Shaders

**Location:** `Engine/Source/Runtime/Engine/Classes/Materials/` and `Engine/Source/Runtime/Engine/Classes/Engine/BlendableInterface.h`

### Material System

**Key Classes:**
- `UMaterial` - Root material asset
- `UMaterialInstance` - Parameterized material instance
- `UMaterialInstanceConstant` - Non-runtime material instance
- `UMaterialInstanceDynamic` - Runtime parameter updates
- `UMaterialExpression` - Nodes in material graph
- `UMaterialFunction` - Reusable material function

### Common Material Expressions

- TextureSample
- Constant, Constant3Vector, Constant4Vector
- Multiply, Add, Subtract, Divide
- Lerp (Linear Interpolate)
- Fresnel
- Normal
- WorldPosition
- CameraPosition
- Time
- VertexColor
- PixelDepth

### Shader System

UE4 uses HLSL shaders compiled for each platform:
- Vertex shaders
- Pixel/fragment shaders
- Compute shaders
- Geometry shaders

Shaders are generated from:
- Material graphs
- Custom HLSL nodes
- Engine shader files (`.usf`)

### Bevy Adaptation

- Bevy materials: `StandardMaterial`, custom `Material` trait
- Bevy shaders: WGSL in `.wgsl` files
- Custom materials can expose uniforms
- Post-processing via render graph

```rust
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom.wgsl".into()
    }
}
```

---

## Particle Systems & VFX

**Location:** `Engine/Source/Runtime/Engine/Classes/Particles/`

### Cascade Particle System

**Key Classes:**
- `UParticleSystem` - Particle system asset
- `UParticleEmitter` - Single emitter
- `UParticleModule` - Behavior module
- `UParticleSystemComponent` - Runtime component

### Module Types

- Spawn modules (rate, burst)
- Lifetime modules
- Initial velocity/size/rotation
- Color over life
- Size over life
- Velocity over life
- Acceleration
- Collision
- Sub-image animation
- Light
- Event generators

### Niagara (Preview in UE4, mature in UE5)

Niagara is the next-gen VFX system with:
- GPU simulation
- Modular emitters
- Data interfaces
- Simulation stages

### Bevy Adaptation

Bevy's particle support is limited. Options:
- `bevy_hanabi` - GPU particle system for Bevy
- Custom CPU particle systems
- Simple spawn/despawn particle entities

```rust
#[derive(Component)]
pub struct ParticleEmitter {
    pub spawn_rate: f32,
    pub lifetime: f32,
    pub velocity: Vec3,
    pub velocity_variance: f32,
    pub start_color: Color,
    pub end_color: Color,
    pub start_size: f32,
    pub end_size: f32,
}
```

---

## Logging, Profiling & Debugging

**Location:** `Engine/Source/Runtime/Core/Public/Logging/` and `Engine/Source/Runtime/Engine/Classes/VisualLogger/`

### Logging System

UE4 logging categories:
```cpp
DECLARE_LOG_CATEGORY_EXTERN(LogMySystem, Log, All);

UE_LOG(LogMySystem, Log, TEXT("Message: %s"), *MyString);
```

Log levels: Fatal, Error, Warning, Display, Log, Verbose, VeryVerbose

### Visual Logger

**Location:** `Engine/Source/Runtime/Engine/Classes/VisualLogger/VisualLoggerExtension.h`

The Visual Logger captures:
- Actor state over time
- Debug shapes
- Text logs
- AI decisions
- Perception events

Useful for debugging AI and gameplay.

### Profiling

- Unreal Insights - CPU/GPU profiler
- Stat commands (`stat fps`, `stat unit`)
- RenderDoc integration
- Memory profiling

### Bevy Adaptation

- Use `tracing` crate for logging (Bevy uses this)
- Use Bevy's built-in diagnostics: `DiagnosticsStore`
- Custom dashboard logging (already implemented)
- Tracy profiler integration with Bevy

```rust
info!("Entity {} took damage", entity);
warn!("Low health: {}", health.current);
error!("Failed to load asset: {}", path);
```

---

## Data Tables & Curves

**Location:** `Engine/Source/Runtime/Engine/Classes/Engine/DataTable.h` and `Engine/Source/Runtime/Engine/Classes/Curves/`

### Data Tables

Data-driven rows of structs:
- Weapon stats
- Enemy stats
- Item definitions
- Loot tables
- Quest data

```cpp
USTRUCT(BlueprintType)
struct FWeaponData : public FTableRowBase
{
    UPROPERTY(EditAnywhere, BlueprintReadOnly)
    float Damage;
    
    UPROPERTY(EditAnywhere, BlueprintReadOnly)
    float FireRate;
    
    UPROPERTY(EditAnywhere, BlueprintReadOnly)
    TSubclassOf<UDamageType> DamageType;
};
```

### Curve Tables

Float curves indexed by value:
- Damage falloff
- Experience curves
- Difficulty scaling

### Bevy Adaptation

```rust
#[derive(Resource)]
pub struct WeaponTable {
    pub entries: HashMap<String, WeaponData>,
}

#[derive(Clone, Deserialize)]
pub struct WeaponData {
    pub damage: f32,
    pub fire_rate: f32,
    pub damage_type: String,
}

// Load from RON/JSON/YAML
```

---

## Localization

**Location:** `Engine/Source/Runtime/Engine/Classes/Internationalization/`

### Localization System

UE4 uses:
- Text asset files (`.po`)
- Culture-specific formatting
- Namespace/key organization
- `FText` for localized strings

### Key Concepts

- `FText` - Localized text
- `FName` - Immutable string identifier
- `FString` - Mutable string
- Culture switching at runtime

### Bevy Adaptation

- Use `fluent` crate for localization
- Or simple JSON key-value lookup
- Integrate with UI text rendering

```rust
#[derive(Resource)]
pub struct Localization {
    pub current_locale: String,
    pub strings: HashMap<String, HashMap<String, String>>,
}
```

---

## Landscape & Foliage

**Location:** `Engine/Source/Runtime/Landscape/` and `Engine/Source/Runtime/Foliage/`

### Landscape

**Key Classes:**
- `ALandscape` - Landscape actor
- `ULandscapeComponent` - Render component
- `ULandscapeHeightfieldCollisionComponent` - Collision
- `ULandscapeInfo` - Landscape metadata

Features:
- Heightmap-based terrain
- Multiple material layers
- Grass types
- HLOD support

### Foliage

**Key Classes:**
- `UFoliageType` - Foliage asset definition
- `AInstancedFoliageActor` - Manages foliage instances
- `UHierarchicalInstancedStaticMeshComponent` - Renders many instances

### Bevy Adaptation

- Generate terrain meshes from heightmaps
- Use `bevy_lunex` or custom terrain systems
- GPU instancing for grass/trees via Bevy's `RenderCommand`
- Or use `bevy_mesh_terrain` crates

---

## Multiplayer & Replication Details

**Location:** `Engine/Source/Runtime/Engine/Classes/Net/`

### Replication Details

**Actor Replication:**
- `bReplicates` flag
- `GetLifetimeReplicatedProps` declares replicated properties
- Replication conditions: None, InitialOnly, OwnerOnly, SkipOwner, SimulatedOnly, AutonomousOnly

**Network Roles:**
- Authority
- AutonomousProxy (owned by local client)
- SimulatedProxy (other clients)

**Replication Graph:**
Advanced replication culling and aggregation.

### RPCs

```cpp
UFUNCTION(Server, Reliable, WithValidation)
void ServerFire(FVector AimLocation);

UFUNCTION(Client, Reliable)
void ClientShowHitEffect(FVector Location);

UFUNCTION(NetMulticast, Reliable)
void MulticastPlaySound(USoundBase* Sound);
```

### Network Prediction for Movement

Client predicts movement, server corrects.

### Bevy Adaptation

- `bevy_replicon` provides snapshot/replication
- Implement prediction/rollback for competitive gameplay
- Or use simpler lockstep for slower games

---

## Asset Pipeline

### Asset Types

- **UStaticMesh** - 3D static geometry
- **USkeletalMesh** - Skinned geometry
- **UTexture** - 2D/3D textures
- **UMaterial** - Shading graphs
- **USoundWave** - Audio
- **UAnimSequence** - Animation

### Import Pipeline

**Location:** `Engine/Source/Editor/UnrealEd/Private/Factories/`

- FBX import
- Texture import
- Audio import
- CSV/JSON data tables

### Derived Data Cache

UE4 cooks/compresses assets into platform-specific formats.

### Bevy Adaptation

- glTF assets via `bevy_gltf`
- Custom asset loaders with `AssetLoader` trait
- Asset preprocessing pipeline

---

## AI-Native Engine Adaptation Roadmap

### Immediate Priorities (Next 2-4 Weeks)

1. **GameplayTags System**
   - Implement component-based tag system
   - Add hierarchical matching
   - Integrate with damage/events

2. **Damage System**
   - `DamageEvent` with type, amount, instigator
   - `Health` component
   - `DamageType` data

3. **AI Perception Prototype**
   - Sight sense using raycasts
   - Hearing sense using event-driven detection
   - Known entity memory

4. **Simple Behavior Tree / State Machine**
   - Patrol/Chase/Attack states
   - Transitions based on perception

### Short-Term (1-3 Months)

5. **Navigation System**
   - Integrate navmesh/pathfinding crate
   - Path following component

6. **Character Movement**
   - Movement component abstraction
   - Jump/dash/crouch mechanics

7. **Animation State Machine**
   - Basic locomotion blend
   - Attack animations

8. **Status Effect System**
   - Duration-based tags
   - Stack management
   - Effect application/removal

### Medium-Term (3-6 Months)

9. **Gameplay Ability System**
   - Cost/cooldown framework
   - Attribute modifiers
   - Gameplay cues

10. **Quest/Dialogue Data Model**
    - Graph-based quest structure
    - Condition system
    - Dialogue trees

11. **Inventory & Loot**
    - Item definitions
    - Drop tables
    - Equipment slots

12. **Save/Load System**
    - Entity serialization
    - Scene state capture
    - Player progression

### Long-Term (6+ Months)

13. **AI-Generated Content**
    - LLM-driven quest/scene generation
    - AI playtesting loop
    - Content scoring

14. **Advanced AI**
    - Full behavior trees
    - Environment queries
    - Squad tactics
    - Memory/investigation

15. **Editor Features**
    - Scene hierarchy panel
    - Asset browser
    - Visual scripting (nodes)

16. **Networking**
    - Server-authoritative multiplayer
    - Replication system

---

## File & Module Index

### Most Relevant Modules

| Module | Location | Purpose |
|--------|----------|---------|
| AIModule | `Runtime/AIModule/` | AI behavior, perception, navigation, EQS |
| GameplayTags | `Runtime/GameplayTags/` | Hierarchical tag system |
| GameplayTasks | `Runtime/GameplayTasks/` | Async gameplay tasks |
| NavigationSystem | `Runtime/NavigationSystem/` | Pathfinding, navmesh |
| Navmesh | `Runtime/Navmesh/` | Navmesh data structures |
| Engine | `Runtime/Engine/` | Core game framework, rendering, physics |
| Core | `Runtime/Core/` | Fundamentals (math, containers, logging) |
| CoreUObject | `Runtime/CoreUObject/` | UObject reflection/serialization |
| InputCore | `Runtime/InputCore/` | Input abstractions |
| AudioMixer | `Runtime/AudioMixer/` | Audio mixing |
| AnimationCore | `Runtime/AnimationCore/` | Animation fundamentals |
| AnimGraphRuntime | `Runtime/AnimGraphRuntime/` | Animation graph runtime |
| Slate | `Runtime/Slate/` | Low-level UI framework |
| Networking | `Runtime/Networking/` | Network abstractions |
| Projects | `Runtime/Projects/` | Plugin/project management |
| AssetRegistry | `Runtime/AssetRegistry/` | Asset discovery |
| MovieScene | `Runtime/MovieScene/` | Sequencer/cinematics |
| LevelSequence | `Runtime/LevelSequence/` | Cinematic sequences |
| Renderer | `Runtime/Renderer/` | Rendering pipeline |
| RenderCore | `Runtime/RenderCore/` | Rendering core |
| RHI | `Runtime/RHI/` | Render hardware interface |

### Key Header Files

| File | Location | Purpose |
|------|----------|---------|
| Actor.h | `Runtime/Engine/Classes/GameFramework/` | Base actor class |
| Pawn.h | `Runtime/Engine/Classes/GameFramework/` | Base pawn class |
| Character.h | `Runtime/Engine/Classes/GameFramework/` | Character class |
| Controller.h | `Runtime/Engine/Classes/GameFramework/` | Controller base |
| AIController.h | `Runtime/AIModule/Classes/` | AI controller |
| BrainComponent.h | `Runtime/AIModule/Classes/` | AI brain component |
| BehaviorTree.h | `Runtime/AIModule/Classes/BehaviorTree/` | Behavior tree asset |
| AIPerceptionComponent.h | `Runtime/AIModule/Classes/Perception/` | Perception component |
| AISense_Sight.h | `Runtime/AIModule/Classes/Perception/` | Sight sense |
| GameplayTagContainer.h | `Runtime/GameplayTags/Classes/` | Tag container |
| DamageType.h | `Runtime/Engine/Classes/GameFramework/` | Damage type |
| CharacterMovementComponent.h | `Runtime/Engine/Classes/GameFramework/` | Movement |
| CameraComponent.h | `Runtime/Engine/Classes/Camera/` | Camera |
| SpringArmComponent.h | `Runtime/Engine/Classes/GameFramework/` | Camera boom |
| SaveGame.h | `Runtime/Engine/Classes/GameFramework/` | Save game |
| GameModeBase.h | `Runtime/Engine/Classes/GameFramework/` | Game mode |

---

## Conclusion

Unreal Engine 4.22 provides a proven blueprint for game engine architecture. The most valuable patterns for the AI-Native Engine are:

1. **Component-based entity design** (already aligned with Bevy ECS)
2. **Behavior tree / state machine AI** with perception and navigation
3. **GameplayTags** for flexible, data-driven classification
4. **Event-driven damage/status systems**
5. **Modular plugin/module architecture**
6. **Editor tooling** for observation and manipulation
7. **Serialization pipeline** for save/load and asset management

The next recommended action is to implement the **GameplayTags** system, as it serves as the foundation for damage types, status effects, abilities, and AI classification in the ARPG systems.

---

*End of research document.*
