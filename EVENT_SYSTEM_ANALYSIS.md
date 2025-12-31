# Event System Analysis: JavaScript vs Rust

## Overview

Pokemon Showdown's event system is the core mechanism for coordinating game logic. Understanding the differences between JavaScript and Rust implementations is critical for achieving 1-to-1 parity.

---

## Core Event Functions

### JavaScript Battle.ts

```javascript
runEvent(eventid, target, source, sourceEffect, relayVar, onEffect, fastExit)
singleEvent(eventid, effect, state, target, source, sourceEffect, relayVar)
```

### Rust Battle

```rust
run_event(event_id, target, source, source_effect, relay_var) -> Option<i32>
single_event(event_id, effect_id, target, source, source_effect) -> EventResult
```

---

## Key Architectural Differences

### 1. **Parameter Differences**

| Parameter | JavaScript | Rust | Notes |
|-----------|-----------|------|-------|
| `onEffect` | ✅ Present | ❌ Missing | **CRITICAL**: Controls whether sourceEffect's handler is added to handlers list |
| `fastExit` | ✅ Present | ❌ Missing | Controls early exit behavior |
| `state` (singleEvent) | ✅ Present | ❌ Missing | Effect state tracking |

**Impact of Missing `onEffect` Parameter:**

In JavaScript:
```javascript
// When onEffect=true, sourceEffect's handler is ADDED to handlers
if (onEffect) {
    const callback = sourceEffect[`on${eventid}`];
    if (callback !== undefined) {
        handlers.unshift(this.resolvePriority({
            effect: sourceEffect, callback, ...
        }));
    }
}
```

In Rust (current implementation):
- **REMOVED in lines 337-344 of run_event.rs** after discovering onHit duplication bug
- sourceEffect is passed for context only (stored in current_event)
- sourceEffect's handler is NOT automatically called by run_event
- Use `single_event` explicitly when you need to call sourceEffect's handler

### 2. **Return Value Handling**

**JavaScript:**
- Returns: `any` (relayVar)
- Default: `true` if no relayVar provided
- Falsy values: `false`, `null`, `''` (NOT_FAIL), `0`, `undefined`

**Rust:**
- Returns: `EventResult` enum or `Option<i32>`
- Variants: `Boolean`, `Number`, `NotFail`, `Null`, `Continue`, `Stop`, `HitSubstitute`
- Must explicitly match on EventResult types

```rust
pub enum EventResult {
    Continue,         // JavaScript: undefined (keep going)
    Boolean(bool),    // JavaScript: true/false
    Number(i32),      // JavaScript: numeric relayVar
    NotFail,          // JavaScript: '' (empty string, falsy)
    Null,             // JavaScript: null
    Stop,             // JavaScript: break early
    HitSubstitute,    // JavaScript: HIT_SUBSTITUTE (0)
    // ... more variants
}
```

### 3. **Handler Discovery Process**

**JavaScript findEventHandlers:**
```javascript
findEventHandlers(target, eventName, source) {
    // 1. Check if target is array → recursively handle each
    // 2. If Pokemon target:
    //    - Find handlers on target: onEventName
    //    - Find handlers on allies: onAllyEventName, onAnyEventName
    //    - Find handlers on foes: onFoeEventName, onAnyEventName
    // 3. If source exists:
    //    - Find handlers on source: onSourceEventName
    // 4. Find side/field/battle handlers
    // 5. Return all handlers
}
```

**Rust find_event_handlers:**
```rust
pub fn find_event_handlers(
    event_id, target, source
) -> Vec<(String, ID, Option<(usize, usize)>)> {
    // Returns: (event_variant, effect_id, holder_target)
    // Simplified compared to JS:
    // - No array handling
    // - Pokemon targets only
    // - Calls find_pokemon_handlers for target
    // - Adds prefixed handlers (Ally, Foe, Any, Source)
}
```

---

## Event Flow Patterns

### Pattern 1: Move Execution with Both Events

**JavaScript (battle-actions.ts):**
```javascript
// For moves like King's Shield:
const hitResult =
    this.battle.singleEvent("PrepareHit", move, {}, targets[0], pokemon, move) &&
    this.battle.runEvent("PrepareHit", pokemon, targets[0], move);
```

**Analysis:**
1. `singleEvent("PrepareHit", move, ...)` - Calls **move.onPrepareHit** ONLY
2. `runEvent("PrepareHit", pokemon, targets[0], move)` - Finds handlers ON pokemon/targets (NOT on move)

**Rust (try_spread_move_hit.rs):**
```rust
// First: Call move's PrepareHit handler
let prepare_hit_result = battle.single_event(
    "PrepareHit",
    move_id,
    Some(targets[0]),
    Some(pokemon_pos),
    None,
);

// Then: Find handlers on pokemon/target
battle.run_event(
    "PrepareHit",
    Some(pokemon_pos),
    Some(targets[0]),
    Some(move_id),
    None,
);
```

**Key Insight:**
- `single_event` = Direct call to ONE effect's handler
- `run_event` = Find ALL handlers on target/source/field/etc.
- Both are needed for complete event processing

### Pattern 2: Hit Event Handling

**JavaScript:**
```javascript
// In spread_move_hit:
if (moveData.onHit) {
    this.battle.singleEvent('Hit', moveData, {}, target, source, move);
}
this.battle.runEvent('Hit', target, pokemon, move);
```

**What happens:**
1. `singleEvent('Hit', moveData, ...)` - Calls **move.onHit** if it exists
2. `runEvent('Hit', target, pokemon, move)` - Finds handlers on **target** (abilities, items, volatiles)
   - Does NOT call move.onHit again (no onEffect parameter)

**Rust:**
```rust
// Call move's onHit handler
battle.single_event("Hit", move_id, Some(target), Some(source), None);

// Find handlers on target (NOT on move)
battle.run_event("Hit", Some(target), Some(source), Some(move_id), None);
```

**Bug We Fixed:**
- Original Rust code added sourceEffect to handlers in run_event
- This caused move.onHit to be called TWICE (once by single_event, once by run_event)
- JavaScript only calls it twice when `onEffect=true`, which is NOT used for Hit events
- Fix: Removed sourceEffect from handlers list (run_event.rs lines 340-343)

---

## Common Event Types and Their Behavior

### Events That Use Both singleEvent + runEvent

| Event | singleEvent Target | runEvent Target | Purpose |
|-------|-------------------|-----------------|---------|
| PrepareHit | move | pokemon | Move preparation (e.g., protect stall check) |
| Hit | move | target | Move hit effects |
| Try | move | pokemon | Can move be used? |
| TryHit | condition | target | Can move hit target? (protect blocks) |
| StallMove | N/A | pokemon | Protect stall counter check |

### Events That Use Only runEvent

| Event | Target | Purpose |
|-------|--------|---------|
| Damage | target | Modify damage dealt |
| ModifyAtk | pokemon | Modify attack stat |
| BasePower | pokemon | Modify move base power |
| Accuracy | target | Modify accuracy |

---

## Handler Naming Conventions

JavaScript automatically searches for handlers with these prefixes:

| Prefix | When Found | Example |
|--------|-----------|---------|
| `on{Event}` | On the target itself | `onHit` on target's ability |
| `onAlly{Event}` | On target's allies | `onAllyModifyAtk` |
| `onFoe{Event}` | On target's foes | `onFoeTryHit` |
| `onAny{Event}` | On all active Pokemon | `onAnyPrepareHit` |
| `onSource{Event}` | On the source Pokemon | `onSourceHit` |

**Example:**
```javascript
runEvent('ModifyAtk', attacker, defender, move)
```
Searches for:
- `onModifyAtk` on attacker (target)
- `onAllyModifyAtk` on attacker's allies
- `onFoeModifyAtk` on defender (foe)
- `onAnyModifyAtk` on all active Pokemon
- `onSourceModifyAtk` on source (if provided)

---

## Critical Bugs We've Fixed

### Bug 1: onHit Called Twice

**Symptom:** Stall counter was 9 instead of 3 in turn 3

**Root Cause:**
```rust
// WRONG (old code):
let mut handlers = self.find_event_handlers(event_id, target, source);
if let Some(source_effect_id) = source_effect {
    handlers.insert(0, (event_id.to_string(), source_effect_id.clone(), target));
}
```

**Why Wrong:**
- JavaScript only adds sourceEffect when `onEffect=true`
- Rust doesn't have onEffect parameter
- Adding sourceEffect unconditionally caused duplicate handler calls

**Fix:**
```rust
// CORRECT (lines 337-344 in run_event.rs):
// JavaScript: if (onEffect) { ... handlers.unshift(...) }
// Since Rust doesn't have an onEffect parameter, we should NOT add sourceEffect to handlers
// REMOVED: handlers.insert(0, (event_id.to_string(), source_effect_id.clone(), target));
```

### Bug 2: PrepareHit Routing Wrong

**Symptom:** King's Shield's onPrepareHit not being called in turn 3

**Root Cause:**
```rust
// dispatch_single_event.rs was checking volatiles BEFORE moves
if pokemon.volatiles.contains_key(effect_id) {
    return self.handle_condition_event(...);  // Wrong for PrepareHit!
}
if let Some(_move_def) = self.dex.moves().get(effect_id) {
    return self.handle_move_event(...);
}
```

**Why Wrong:**
- PrepareHit should route to MOVE handler, not volatile handler
- Even if Pokemon has kingsshield volatile, PrepareHit on kingsshield effect should route to move

**Fix:**
```rust
// Special case for PrepareHit: check moves FIRST
if event_id == "PrepareHit" {
    if let Some(_move_def) = self.dex.moves().get(effect_id.as_str()) {
        return self.handle_move_event(event_id, effect_str, target);
    }
}
```

### Bug 3: TryHit Not Running Before Accuracy

**Symptom:** Protected moves made accuracy checks when they shouldn't

**Root Cause:**
- TryHit event runs AFTER accuracy check
- Protect blocks should prevent accuracy check

**Fix:**
- Moved TryHit event BEFORE accuracy check in try_spread_move_hit.rs
- Now follows JavaScript order: TryHit → Accuracy → Hit

### Bug 4: EventResult::NotFail Not Handled

**Symptom:** Protected moves returned Some(1) instead of None

**Root Cause:**
```rust
// MISSING case in run_event match:
match event_result {
    EventResult::Boolean(false) => { ... }
    EventResult::Boolean(true) => { ... }
    // EventResult::NotFail => MISSING!
}
```

**Fix:**
```rust
EventResult::NotFail => {
    // JavaScript: relayVar = this.NOT_FAIL; (NOT_FAIL = '' which is falsy)
    result = None;
    break;
}
```

---

## Testing Event System Correctness

### Verification Method: PRNG Synchronization

We verify event system correctness by comparing PRNG call sequences between JS and Rust:

```bash
# JavaScript
JS: Seed 2 Turn 3 - PRNG calls: 2 (total: 8)

# Rust
RUST: Seed 2 Turn 3 - PRNG calls: 2 (total: 8)
```

**Why This Works:**
- Events trigger handlers in specific order
- Handlers make PRNG calls (randomChance, accuracy checks, etc.)
- Different event order = different PRNG sequence
- PRNG mismatch = event system bug

**Example:**
```
Turn 3 Expected:
PRNG #7: randomChance(1, 3) from StallMove
PRNG #8: randomChance(90, 100) from Accuracy check

Turn 3 Actual (before fixes):
PRNG #7: randomChance(1, 3) from StallMove
[MISSING #8 - accuracy check didn't run]
```

---

## Best Practices for Porting Events

### 1. Always Check JavaScript Source

```javascript
// JavaScript:
const hitResult =
    this.battle.singleEvent("PrepareHit", move, {}, targets[0], pokemon, move) &&
    this.battle.runEvent("PrepareHit", pokemon, targets[0], move);
```

**Porting Checklist:**
- ✅ Are there TWO calls (singleEvent + runEvent)?
- ✅ What order are they in?
- ✅ Are they ANDed together (both must succeed)?
- ✅ What are the target/source parameters?

### 2. Use Correct Event Function

**Use `single_event` when:**
- You want to call a SPECIFIC effect's handler
- Example: Call move.onPrepareHit for King's Shield

**Use `run_event` when:**
- You want to find ALL handlers on target/source/field
- Example: Find all onHit handlers on the target Pokemon

**Use BOTH when:**
- JavaScript uses both (most move events)
- Order matters: usually single_event FIRST, then run_event

### 3. Check Handler Discovery

```rust
// Find handlers on pokemon (abilities, items, volatiles)
find_pokemon_handlers(event_id, pokemon_pos)

// Find handlers on side (side conditions)
find_side_handlers(event_id, side_idx)

// Find handlers on field (weather, terrain)
find_field_handlers(event_id)
```

### 4. Verify Event Prefixes

When porting, check if event should search for prefixed handlers:

```rust
// Events that DON'T use prefixed handlers:
"BeforeTurn" | "Update" | "Weather" | "WeatherChange" | "TerrainChange"

// All other events DO use prefixed handlers:
// - onAlly{Event}
// - onFoe{Event}
// - onAny{Event}
// - onSource{Event}
```

### 5. Test with PRNG Verification

After porting an event:

```bash
# Run comparison test
bash compare-seed.sh 2

# Check for PRNG mismatches
✅ Turn 1: 5 calls (JS: 5, Rust: 5)
✅ Turn 2: 1 call  (JS: 6, Rust: 6)
❌ Turn 3: 2 calls (JS: 8, Rust: 7)  # BUG!
```

---

## Event Execution Order

### Full Event Pipeline for Move Execution

1. **BeforeMove** - Can move be used? (Paralysis, Taunt, etc.)
2. **ModifyType** - Change move type (Normalize, etc.)
3. **ModifyMove** - Modify move properties
4. **TryMove** - Final check before move executes
5. **PrepareHit** (singleEvent + runEvent) - Move preparation (Protect stall, etc.)
6. **UseMoveMessage** - Display move message
7. **TryHit** - Can move hit target? (Protect, immunities)
8. **Accuracy** - Accuracy check (if applicable)
9. **Hit Loop** - For each target that passed:
   - **Invulnerability** - Is target invulnerable?
   - **TryImmunity** - Type immunity check
   - **Hit** (singleEvent + runEvent) - Move hits
   - **Damage** - Calculate and apply damage
   - **AfterHit** - Post-hit effects
10. **AfterMove** - Move completed

---

## Debugging Event Issues

### Step 1: Add Debug Logging

```rust
eprintln!("[RUN_EVENT] event={}, target={:?}, source={:?}",
    event_id, target, source);
eprintln!("[RUN_EVENT] Found {} handlers", handlers.len());
for (event_variant, effect_id, _) in &handlers {
    eprintln!("[RUN_EVENT]   - {} on {}", event_variant, effect_id);
}
```

### Step 2: Compare with JavaScript

```javascript
// Add to JavaScript:
console.log(`[runEvent] ${eventid}, handlers:`, handlers.length);
```

### Step 3: Check PRNG Sequence

```bash
# Rust
[PRNG.random] raw_value=2039291620, n=3, result=1

# JavaScript
PRNG #7: 2039291620
```

If raw values match but sequence differs = event order bug

### Step 4: Trace Handler Discovery

```rust
eprintln!("[FIND_POKEMON_HANDLERS] event_id={}, pokemon={}", event_id, pokemon.name);
eprintln!("[FIND_POKEMON_HANDLERS] Ability {} has callback: {}",
    pokemon.ability, ability_has_callback(event_id));
```

---

## Summary: JavaScript vs Rust Event System

| Aspect | JavaScript | Rust | Status |
|--------|-----------|------|--------|
| Core functions | runEvent, singleEvent | run_event, single_event | ✅ Equivalent |
| onEffect parameter | ✅ Present | ❌ Missing | ⚠️ Workaround: don't add sourceEffect |
| Return values | any (relayVar) | EventResult enum | ✅ Mapped correctly |
| Handler discovery | Full (Pokemon/Side/Battle) | Pokemon only | ⚠️ Sufficient for current needs |
| Prefixed handlers | ✅ Full support | ✅ Implemented | ✅ Working |
| Array targets | ✅ Supported | ❌ Not needed | ⚠️ Not used in Rust |
| Event depth limit | 8 | 8 | ✅ Same |
| Suppression logic | ✅ Full | ✅ Full | ✅ Same |

**Overall Assessment:** Rust event system is functionally equivalent for Pokemon-targeted events. Key difference is missing `onEffect` parameter, which requires careful handling when porting JavaScript code.

---

## Open Issues

### 1. Kingsshield Volatile Duration (Turn 3 PRNG Mismatch)

**Symptom:**
- Turn 3 has 1 PRNG call in Rust, 2 in JavaScript
- Missing: Rock Throw's accuracy check

**Hypothesis:**
- kingsshield volatile from turn 2 persists into turn 3
- Blocks Rock Throw before accuracy check
- Should have duration=1 and expire at end of turn 2

**Next Steps:**
1. Verify kingsshield volatile duration is set to 1
2. Check turn-end volatile cleanup runs correctly
3. Confirm volatile expires at end of turn 2
4. Test that turn 3 has 2 PRNG calls after fix

---

## Conclusion

The event system is the heart of Pokemon Showdown. Understanding the subtle differences between JavaScript and Rust implementations is essential for achieving perfect 1-to-1 parity. Key takeaways:

1. **Use both single_event and run_event when JavaScript does**
2. **Never add sourceEffect to handlers in run_event** (no onEffect parameter)
3. **Match event order exactly** (PrepareHit before accuracy, TryHit before accuracy, etc.)
4. **Test with PRNG verification** to catch event order bugs early
5. **Read JavaScript source carefully** - comments and parameter names are critical

By following these principles, we can achieve perfect event system parity and ensure Rust battles match JavaScript exactly.
