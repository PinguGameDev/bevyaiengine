# Jev AI Integration Plan

## What is Jev AI?

Jev is TypeSafe AI's **System One decision model**. Unlike generative LLMs (Claude, GPT) that produce free-form text, Jev returns **typed, structured decisions with calibrated probabilities**:

| Primitive | What it does | Returns |
|-----------|-------------|---------|
| **Choice** | Pick one option from a defined set | Selected option + probability distribution over all options + confidence |
| **Score** | Rate something against an ordered rubric | Score level + probability per level + confidence |
| **Noul** | Evaluate a yes/no question | Probability that the answer is "yes" |

**Key properties:**
- No text generation — bounded, typed outputs only
- Calibrated probabilities (not vibes)
- Multiple questions in a single request (parallel evaluation)
- Fast and cheap: ~$0.042 per million input tokens via Vercel AI Gateway
- Software-native: designed for code to consume directly, not for humans to read

---

## Why Jev Matters for This Project

Your engine's vision is "AI-native from ground up" — AI is the primary interface, not a plugin. That means the AI layer must be **reliable, inspectable, and deterministic**. Jev solves the hardest part of that:

### The Problem Jev Solves

When an LLM (Claude/GPT) controls your engine via tool calls, you have no way to:
- Know *which* tool it chose and *why* (it's a black box)
- Guard against destructive actions (delete entity, reset scene)
- Validate that AI-generated content matches the user's intent
- Score the quality of AI-generated quests, encounters, or dialogue
- Decide when the AI is "done" with a task

Without Jev, every decision is a generative guess. With Jev, every decision is a **typed judgment with a probability you can threshold in code**.

### Where Jev Fits in the Architecture

```
User: "Create a dungeon with 5 rooms"
        |
        v
[Generative LLM (Claude/GPT)]  <-- writes code, generates content
        |
        v
[Jev Decision Layer]           <-- validates, routes, guards, scores
        |
        v
[Bevy Engine (ECS)]            <-- executes tool calls, renders scene
```

Jev sits **between** the generative AI and the engine. It doesn't replace Claude/GPT — it makes them safe and reliable.

---

## Your API Access: Vercel AI Gateway

You have a Vercel AI Gateway API key. This gives you access to Jev through a proxied endpoint:

| Field | Value |
|-------|-------|
| **Base URL** | `https://ai-gateway.vercel.sh/typesafe` |
| **System One endpoint** | `POST https://ai-gateway.vercel.sh/typesafe/v1/systemone` |
| **Models endpoint** | `GET https://ai-gateway.vercel.sh/typesafe/v1/models` |
| **Model ID** | `typesafe-ai/jev` |
| **Auth header** | `Authorization: Bearer $AI_GATEWAY_API_KEY` |
| **Pricing** | $0.042 per 1M input tokens (output is free) |

**You do NOT need a separate TypeSafe API key.** Vercel AI Gateway handles authentication and billing.

### Verify your key works

```powershell
curl https://ai-gateway.vercel.sh/typesafe/v1/systemone `
  -H "Authorization: Bearer $env:AI_GATEWAY_API_KEY" `
  -H "Content-Type: application/json" `
  -d '{
    "model": "typesafe-ai/jev",
    "state": "Player is at 10% health, enemy is at 80% health, player has a healing potion",
    "questions": {
      "should_heal": {
        "type": "noul",
        "instructions": "Should the player use a healing potion?"
      }
    }
  }'
```

Expected response shape:
```json
{
  "answers": {
    "should_heal": {
      "type": "noul",
      "noul": 0.92,
      "confidence": 0.87
    }
  },
  "usage": { "inputTokens": 45 }
}
```

---

## Concrete Use Cases for This Engine

### 1. AI Tool Router (Phase 1 — Highest Priority)

When the generative LLM wants to call one of your 10+ engine tools, Jev decides **which** tool to call. This scales better than stuffing all tool schemas into one prompt.

```
State: user request + current scene state
Question: Choice — "Which tool should execute next?"
Options: CREATE_ENTITY, DELETE_ENTITY, SET_TRANSFORM, CREATE_CUBE,
         CREATE_LIGHT, CREATE_CAMERA, LIST_ENTITIES, SCREENSHOT, NONE
```

**Why this matters:** As your tool count grows (20, 50, 100+), LLMs degrade at tool selection. Jev handles it in one fast call with calibrated probabilities.

### 2. Tool-Call Guardrails (Phase 1)

Before executing destructive operations, Jev judges whether the action is safe.

```
State: proposed tool call + current scene + user's last request
Question: Noul — "Is this tool call consistent with the user's request?"
Threshold: if probability < 0.7, ask user for confirmation
```

**Why this matters:** Prevents the AI from deleting entities the user didn't ask to delete, or placing objects in invalid locations.

### 3. NPC / Enemy AI Decisions (Phase 2 — ARPG)

Replace hand-written behavior trees with Jev-driven decisions.

```
State: enemy health, player distance, enemy type, terrain, cooldowns
Question: Choice — "What should the enemy do?"
Options: PATROL, CHASE, ATTACK_MELEE, ATTACK_RANGED, FLEE, IDLE
```

**Why this matters:** AI-driven enemies that adapt without hand-tuning. The probabilities give you inspectable decision-making ("the enemy fled because it was at 15% health and the player was at full health — 0.89 probability").

### 4. Content Quality Scoring (Phase 2)

When the AI generates quests, dialogue, or encounters, Jev scores them.

```
State: generated quest description + target difficulty + player level
Question: Score — "Rate this quest's difficulty"
Levels: TRIVIAL, EASY, NORMAL, HARD, NIGHTMARE
```

**Why this matters:** Automated quality gates. "If the generated boss encounter scores below 0.6 on 'interesting', regenerate it."

### 5. Task Completion Verification (Phase 3 — Demo)

When the AI claims it's done building a scene, Jev verifies.

```
State: user's original request + current scene entity list + entity properties
Question: Noul — "Does the scene match the user's request?"
```

**Why this matters:** Critical for the funding demo. "AI says it created 5 rooms — Jev verifies with 0.94 probability." This is the "AI playtests and reports bugs" loop from your plan.

### 6. Model Routing (Phase 3)

Jev decides which generative model to use for each request.

```
State: user request complexity + scene state + available models
Question: Choice — "Which model should handle this?"
Options: CLAUDE_SONNET, GPT_4O_MINI, LOCAL_MODEL
```

**Why this matters:** Cost optimization. Simple requests ("move the cube left") don't need Claude. Jev routes cheap tasks to cheap models.

---

## Integration Architecture

### Option A: Rust SDK (In-Process, Recommended)

Use the `typesafe-ai` Rust crate directly inside Bevy systems. Jev decisions happen inside the engine process.

```
[Bevy App]
  |
  +-- [JevSystem] -- calls Jev API via typesafe-ai crate
  |                     points at Vercel AI Gateway base URL
  |
  +-- [ToolRouterSystem] -- uses Jev Choice to pick tools
  +-- [GuardrailSystem] -- uses Jev Noul to validate actions
  +-- [EnemyAISystem] -- uses Jev Choice for NPC decisions
  +-- [QualityGateSystem] -- uses Jev Score for content validation
```

**Pros:**
- Zero latency overhead (same process)
- Full access to ECS world state for Jev context
- Type-safe Rust enums for all Jev responses
- Works with async (reqwest) or blocking (ureq) HTTP clients

**Cons:**
- Requires `TYPESAFE_API_KEY` or custom base URL config
- Need to handle API key securely in Rust binary

### Option B: MCP Server (Agent-Facing)

Expose Jev as an MCP tool alongside your engine tools. The generative LLM agent discovers Jev tools and calls them.

```
[AI Agent (Claude/GPT)]
  |
  +-- MCP Tool: create_entity(...)
  +-- MCP Tool: delete_entity(...)
  +-- MCP Tool: jev_decide(state, questions)  <-- Jev as a tool
  +-- MCP Tool: jev_guard(state, proposed_action)
```

**Pros:**
- Agent can self-regulate (ask Jev before acting)
- No engine changes needed
- Works with any MCP-compatible agent

**Cons:**
- Adds network round-trip per Jev call
- Agent must know when to call Jev (requires good prompting)
- Less deterministic than in-process

### Option C: Hybrid (Best for Funding Demo)

- **Rust SDK** for in-engine decisions (NPC AI, guardrails, content validation)
- **MCP tool** for the developer-facing AI agent (Jev validates the agent's choices)

This gives you the best of both worlds and the strongest demo story.

---

## Implementation Plan

### Step 1: Verify API Key (5 minutes)

Run the curl command above to confirm your Vercel AI Gateway key works with Jev.

### Step 2: Add Dependencies (2 minutes)

```toml
# Cargo.toml
[dependencies]
bevy = "0.19.1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**Note:** We use raw `reqwest` + `serde` instead of the `typesafe-ai` crate because:
- The crate targets `api.typesafe.ai` directly; we need Vercel's gateway URL
- Raw HTTP gives us full control over the request shape
- Fewer dependencies to manage during early development
- We can migrate to the crate later if it supports custom base URLs

### Step 3: Create Jev Client Module (1-2 hours)

Create `src/jev/mod.rs` with:
- `JevClient` struct holding the API key and base URL
- `JevRequest` / `JevResponse` types matching the TypeSafe API schema
- `evaluate()` method that sends requests and parses typed responses
- Error handling for rate limits, auth failures, malformed responses

### Step 4: Create Jev Bevy Resource (30 minutes)

Register `JevClient` as a Bevy `Resource` so all systems can access it:

```rust
#[derive(Resource)]
struct JevClient {
    api_key: String,
    base_url: String,
    http: reqwest::Client,
}
```

### Step 5: Build Tool Router (1-2 hours)

Create a system that uses Jev Choice to decide which engine tool to call:

```rust
async fn route_tool(
    client: &JevClient,
    user_input: &str,
    scene_state: &str,
) -> ToolChoice {
    // Send Choice question to Jev
    // Return typed tool selection with probability
}
```

### Step 6: Build Guardrail System (1 hour)

Before executing any destructive tool call, send a Noul question to Jev:

```rust
async fn guard_tool_call(
    client: &JevClient,
    proposed_action: &str,
    user_request: &str,
) -> bool {
    // Return true if Jev says the action is consistent with user request
    // Threshold: probability > 0.7
}
```

### Step 7: Wire Into MCP Server (Phase 1, later)

Once the MCP server is built (from NEXT_STEPS.md), expose Jev as an additional tool:
- `jev_decide(state, questions)` — generic decision tool
- `jev_guard(action, context)` — safety check before destructive ops

### Step 8: Enemy AI Integration (Phase 2)

Replace simple state machines with Jev-driven decisions:

```rust
fn enemy_ai_system(
    jev: Res<JevClient>,
    enemies: Query<(&Enemy, &Transform, &Health)>,
    player: Query<&Transform, With<Player>>,
) {
    // For each enemy, send state to Jev
    // Jev returns Choice: PATROL, CHASE, ATTACK, FLEE
    // Apply decision to enemy behavior
}
```

---

## Cost Estimates

| Scenario | Tokens per call | Calls per minute | Monthly cost |
|----------|----------------|------------------|-------------|
| Tool routing (dev session) | ~500 | 10 | ~$0.63 |
| Guardrails (dev session) | ~300 | 5 | ~$0.19 |
| Enemy AI (gameplay) | ~200 | 60 (1/sec per enemy) | ~$7.56 |
| Content scoring (generation) | ~1000 | 2 | ~$2.52 |
| **Total (active development)** | | | **~$11/month** |

This is extremely cheap. A typical Claude/GPT session costs 10-100x more.

---

## Risks and Mitigations

| Risk | Mitigation |
|------|-----------|
| API latency adds to frame time | Run Jev calls async; don't block the render loop. Cache decisions for static state. |
| API key exposed in binary | Read from environment variable at runtime, never hardcode. Use `.env` file locally. |
| Jev makes wrong decisions | Always threshold probabilities in code. Log all Jev decisions for debugging. |
| Rate limits under heavy use | Vercel AI Gateway has generous limits. Batch questions per request. |
| Vercel AI Gateway goes down | Fall back to local heuristics. Jev is an optimization, not a hard dependency. |

---

## Questions Before Proceeding

Before I start implementing, I need to clarify:

1. **API key format:** Is your Vercel AI Gateway key stored as `AI_GATEWAY_API_KEY` in your environment? Or do you have it saved somewhere else?

2. **Priority:** Do you want to:
   - (A) Build the MCP tool interface first (10 scene tools from NEXT_STEPS.md), then add Jev on top?
   - (B) Add Jev integration now, before the MCP server?
   - (C) Do both in parallel (Jev client module now, wire it into MCP later)?

3. **Scope for now:** Should I start with just the Jev client module (Step 2-4) so we have a working API connection, or go all the way to tool routing + guardrails (Step 5-6)?

4. **Async runtime:** Bevy 0.19 doesn't include a built-in async runtime. Should I add `tokio` as the async runtime for HTTP calls, or use blocking `ureq` to keep things simple?

5. **Environment setup:** Do you already have `AI_GATEWAY_API_KEY` set as an environment variable on this machine, or do we need to set that up?

---

## Summary

Jev is the **decision backbone** that makes your AI-native engine reliable and inspectable. It turns vague LLM tool calls into typed, probabilistic decisions your code can act on. With your Vercel AI Gateway key, integration is straightforward — one HTTP endpoint, typed JSON in/out, ~$11/month at active development pace.

The immediate next step is creating the Jev client module in Rust and verifying the API connection works. From there, we layer in tool routing, guardrails, and eventually NPC AI and content scoring.
