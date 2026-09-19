# AI-Native Game Engine: Project Plan

## Vision

Build the first **AI-native game engine** where AI is the primary interface, not a plugin. Developers describe their game in natural language and the AI builds it. Purpose-built for Action RPGs (Diablo/Hades/PoE style).

---

## Core Differentiation

**AI-native from ground up.** Developers describe what they want in natural language; AI builds it. Traditional editor is secondary/fallback. AI is deeply integrated at every layer: development, runtime, and as the primary interface option.

**Why developers choose this over Godot/Unity/Unreal:**
- 10x productivity for solo devs and small teams
- Purpose-built ARPG tooling (combat, quests, dialogue, inventory, loot)
- "Describe your game, AI builds it" — no engine learning curve
- Rust's memory safety + data-oriented ECS architecture

---

## Target Market

1. **Solo devs / small teams** — indie developers who want AI to multiply productivity 10x
2. **ARPG studios specifically** — studios building Diablo/Hades-like games

---

## Timeline & Funding

- **Status**: Seeking funding/sponsorship
- **Goal**: Compelling pitch + working demo within 6-12 months
- **Approach**: Show traction and technical feasibility to investors; demonstrate a differentiated product (10x, not 10%)

---

## Why Bevy (Not Stride)

### Bevy advantages for an AI-native engine

| Factor | Why Bevy wins |
|--------|---------------|
| **AI-native architecture** | ECS is data-oriented, explicit, inspectable — AI reads/writes world state as plain data, no UI to work around |
| **Pure-code approach** | No editor to bypass; AI has maximum control |
| **Rust safety** | Fewer crashes = better AI feedback loop |
| **Differentiation** | "AI-native engine in Rust" is disruptive; "Stride with AI features" is incremental |
| **Funding story** | 10x better vs 10% better |
| **Demo timeline** | Compelling AI-native demo possible in 6 months |
| **Market fit** | Solo devs want max AI leverage; ARPG studios want specialized tools — both served by Bevy's ECS |

### Why not Stride

- Not differentiated enough for funding / community excitement
- Editor-centric architecture — AI must work around UI, not native to it
- C# lacks Rust's safety and tight AI-compiler feedback loop
- Incremental, not disruptive

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│         Natural Language Interface (Primary)            │
│  "Create a dungeon with 5 rooms, each with enemies"     │
└──────────────────┬──────────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────────┐
│              AI Agent Layer                              │
│  - LLM reasoning (Claude/GPT-4/local models)            │
│  - Tool definitions (MCP/OpenAPI)                       │
│  - Context management (project state, assets)           │
│  - Multi-agent orchestration                            │
└──────────────────┬──────────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────────┐
│              Bevy Engine (Rust)                          │
│  - ECS world (data-oriented, AI-inspectable)            │
│  - wgpu rendering (modern, portable)                    │
│  - Avian physics (ECS-native)                           │
│  - Asset pipeline (glTF, hot reload)                    │
│  - Minimal visual feedback (scene viewer, inspector)    │
└──────────────────┬──────────────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────────────┐
│              ARPG Systems (Built on Bevy)                │
│  - Combat system                                        │
│  - Quest/dialogue system                                │
│  - Inventory/loot system                                │
│  - Enemy AI (behavior trees)                            │
│  - Skill system                                         │
│  - Level streaming                                      │
└─────────────────────────────────────────────────────────┘
```

---

## 6-Month Demo Plan (For Funding)

### Month 1-2: Core Engine Setup
- Bevy rendering + physics working
- AI tool interface (MCP server) with 20+ tools
- AI can create scenes from natural language

### Month 3-4: ARPG Systems
- Combat system (attacks, damage, hit detection)
- Basic quest/dialogue system
- Enemy AI (simple behavior trees)
- AI can generate quests and enemy placements

### Month 5-6: Polish + Demo
- Asset pipeline (Blender → engine via glTF)
- AI imports assets and sets up materials
- Minimal visual inspector (scene viewer, property editor)
- AI playtests and reports bugs
- Record demo video showing end-to-end workflow

### Funding Demo Script
1. Developer says: "Create an ARPG dungeon with 5 rooms"
2. AI generates level layout, places enemies, sets up lighting
3. Developer says: "Add a boss in room 5 with a health bar"
4. AI creates boss entity, adds health bar UI, sets up attack patterns
5. Developer says: "Add a quest to defeat the boss and get a legendary sword"
6. AI creates quest logic, dialogue, loot table
7. Developer says: "Playtest and report any bugs"
8. AI runs game, detects issues, suggests fixes
9. Developer inspects scene in minimal visual UI, makes tweaks via natural language

---

## Risks & Mitigation

| Risk | Mitigation |
|------|------------|
| 24-36 months to complete full engine | Focus on ARPG vertical first, not general-purpose |
| Need some visual feedback | Build minimal scene viewer + inspector (2-3 months) |
| Asset pipeline complexity | Use glTF standard, build import automation |
| Smaller Bevy community | Open source from day one, build in public |
| Competing with established engines | Don't compete on features — compete on AI-native approach |

---

## Next Steps

### 1. Validate Technical Feasibility (2 weeks)
- Install Bevy, build simple scene
- Create MCP server with 10 tools
- Test AI can create scene via tool calls

### 2. Build Funding Pitch (2 weeks)
- Technical architecture document
- Market analysis (solo devs + ARPG studios)
- Competitive analysis (why AI-native wins)
- 6-month demo plan

### 3. Seek Funding (ongoing)
- Apply to game dev grants (Epic MegaGrants, etc.)
- Approach VCs focused on AI + gaming
- Consider sponsorships (NVIDIA, AMD, etc.)

### 4. Build in Public (ongoing)
- Open source from day one
- Regular dev logs / videos
- Community building (Discord, Twitter)

---

## Final Recommendation

**Use Bevy. Build the engine. Seek funding. Ship demo in 6 months.**

The key insight: we are not building "a lighter Unreal." We are building **"the first AI-native game engine."** That is a fundamentally different product with a different architecture, market, and funding story.