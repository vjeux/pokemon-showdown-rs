# Event System Architecture Analysis

## JavaScript Event System Overview

### Event Handler Discovery Methods

#### 1. `findEventHandlers(target, eventName, source)`
**Purpose**: Main orchestrator that discovers ALL event handlers for a given event across the entire battle hierarchy.

**Flow**:
```typescript
// For Pokemon targets:
1. Get Pokemon's own handlers: findPokemonEventHandlers(target, "onEventName")
2. If prefixed handlers allowed:
   - For each ally: findPokemonEventHandlers(ally, "onAllyEventName")
   - For each ally: findPokemonEventHandlers(ally, "onAnyEventName")
   - For each foe: findPokemonEventHandlers(foe, "onFoeEventName")
   - For each foe: findPokemonEventHandlers(foe, "onAnyEventName")
3. If source exists: findPokemonEventHandlers(source, "onSourceEventName")
4. Bubble up to Side level

// For Side targets:
1. For each side:
   - For each active Pokemon:
     - Get on/onFoe/onAny handlers
   - Get side-level handlers: findSideEventHandlers
2. Get field handlers: findFieldEventHandlers
3. Get battle handlers: findBattleEventHandlers
```

**Key insight**: Events are **broadcast** to multiple Pokemon/Sides/Field with prefixed variants.

#### 2. `findPokemonEventHandlers(pokemon, callbackName, getKey?)`
**Purpose**: Collect event handlers from a SINGLE Pokemon's effect sources.

**Checks 6 sources in order**:
1. **Status** (burn, paralysis, etc.)
2. **Volatiles** (confusion, substitute, etc.)
3. **Ability**
4. **Item**
5. **Species** (for form changes)
6. **Slot conditions** (Future Sight, Wish, etc.)

For each source:
```typescript
callback = this.getCallback(pokemon, effect, callbackName);
if (callback !== undefined || (getKey && state[getKey])) {
    handlers.push(this.resolvePriority({
        effect,
        callback,  // The actual function to call
        state,     // State object (statusState, abilityState, etc.)
        end,       // Removal function
        effectHolder: pokemon,
    }, callbackName));
}
```

**Returns**: `EventListener[]` - Array of handler objects with metadata.

#### 3. `findBattleEventHandlers(callbackName, getKey?, customHolder?)`
**Purpose**: Collect format-level and custom event handlers.

**Checks**:
1. Format's callback (e.g., Gen 9 Doubles format rules)
2. Custom handlers registered via `onEvent()`

#### 4. `findFieldEventHandlers(field, callbackName, getKey?, customHolder?)`
**Purpose**: Collect field condition handlers.

**Checks**:
1. **Pseudo-weather** (Trick Room, Gravity, etc.)
2. **Weather** (Sun, Rain, etc.)
3. **Terrain** (Electric Terrain, Grassy Terrain, etc.)

#### 5. `findSideEventHandlers(side, callbackName, getKey?, customHolder?)`
**Purpose**: Collect side condition handlers.

**Checks**:
- **Side conditions** (Reflect, Light Screen, Stealth Rock, etc.)

### Event Execution (`runEvent`)

```typescript
runEvent(eventid, target, source, effect, relayVar, onEffect, fastExit) {
    // 1. Find all handlers
    const handlers = this.findEventHandlers(target, eventid, source);

    // 2. Add onEffect handler if specified (for move secondary effects)
    if (onEffect && sourceEffect[`on${eventid}`]) {
        handlers.unshift(...);
    }

    // 3. Sort handlers by priority/speed
    if (['Invulnerability', 'TryHit', ...].includes(eventid)) {
        handlers.sort(Battle.compareLeftToRightOrder);  // Position-based
    } else if (fastExit) {
        handlers.sort(Battle.compareRedirectOrder);     // Priority-based
    } else {
        this.speedSort(handlers);                       // Speed-based
    }

    // 4. Execute each handler
    for (const handler of handlers) {
        // Check suppressions (Mold Breaker, Gastro Acid, etc.)
        if (suppressed) continue;

        // Call the callback
        returnVal = handler.callback.call(this, ...args);

        // Handle return value
        if (returnVal === false) return false;  // Stop propagation
        if (returnVal !== undefined) relayVar = returnVal;
    }

    return relayVar;
}
```

### Key JavaScript Features Used

1. **Dynamic property access**: `effect[callbackName]` or `effect.onStart`
2. **Function storage**: Callbacks are actual JavaScript functions stored in effect objects
3. **`this` context**: Callbacks are called with `this` bound to Battle
4. **Undefined checks**: `callback !== undefined` to check if handler exists
5. **State objects**: Each effect has associated state (statusState, abilityState, etc.)
6. **Priority/Speed sorting**: Handlers sorted before execution
7. **Early return**: `return false` stops event propagation

---

## Rust Architecture Proposals

### Challenge: Static vs Dynamic Dispatch

**JavaScript approach**: Store function pointers, check `undefined`, call dynamically.
**Rust constraint**: No dynamic property access, need compile-time type safety.

### Current Rust Implementation (Partial)

```rust
// Dispatcher pattern (static dispatch)
pub fn dispatch_on_start(battle: &mut Battle, ability_id: &str, pokemon_pos: (usize, usize)) {
    match ability_id {
        "intimidate" => intimidate::on_start(battle, pokemon_pos),
        "trace" => trace::on_start(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

// Filter helpers (current implementation with manual maintenance)
fn ability_has_callback(&self, ability_id: &str, event_id: &str) -> bool {
    if event_id == "onStart" {
        return matches!(ability_id, "intimidate" | "trace" | ...);
    }
    false
}
```

### Proposed Solutions

## **Solution 1: Generated Can-Dispatch Methods (RECOMMENDED)**

**Architecture**:
- Keep dispatcher functions as-is
- Auto-generate `has_callback` methods from dispatcher code
- Single source of truth: dispatcher match statements

**Benefits**:
✅ Zero runtime overhead (const matching)
✅ Always synchronized with dispatchers
✅ Type-safe at compile time
✅ Fast checks (no hashmap lookups)
✅ Already partially implemented

**Implementation**:

```rust
// src/data/ability_callbacks/mod.rs (generated from TS)
pub fn dispatch_on_start(battle: &mut Battle, ability_id: &str, pos: (usize, usize)) {
    match ability_id {
        "intimidate" => intimidate::on_start(battle, pos),
        "trace" => trace::on_start(battle, pos),
        _ => EventResult::Continue,
    }
}

// src/battle/has_callback.rs (auto-generated from dispatchers!)
impl Battle {
    fn ability_has_callback(&self, ability_id: &str, event_id: &str) -> bool {
        if event_id == "onStart" {
            return matches!(ability_id, "intimidate" | "trace");
        }
        false
    }
}

// Usage in find_pokemon_event_handlers
if !pokemon.ability.is_empty() && self.has_callback(&pokemon.ability, event_id) {
    handlers.push((pokemon.ability.clone(), Some(target)));
}
```

**Script**: `scripts/generate-can-dispatch.js` parses dispatcher functions and generates `has_callback.rs`

**Workflow**:
1. Run `generate-abilities.js` → creates ability files + dispatchers
2. Run `generate-can-dispatch.js` → parses dispatchers, generates `has_callback.rs`
3. Compile! Everything is synchronized.

---

## **Solution 2: HashMap Registry (NOT RECOMMENDED)**

**Architecture**:
- Build HashMap<(EffectID, EventID), bool> at runtime
- Check registry instead of match statements

**Drawbacks**:
❌ Runtime overhead (hash lookups)
❌ Memory overhead (hash tables)
❌ Still need to populate from somewhere (dispatcher code parsing at build time?)
❌ Loses compile-time guarantees

---

## **Solution 3: Trait-Based Dynamic Dispatch**

**Architecture**:
```rust
trait EventHandler {
    fn on_start(&self, battle: &mut Battle, pos: (usize, usize)) -> EventResult {
        EventResult::Continue
    }
    fn on_switch_in(&self, battle: &mut Battle, pos: (usize, usize)) -> EventResult {
        EventResult::Continue
    }
    // ... all events
}

struct Intimidate;
impl EventHandler for Intimidate {
    fn on_start(&self, battle: &mut Battle, pos: (usize, usize)) -> EventResult {
        // implementation
    }
}

// Ability registry
fn get_ability_handler(ability_id: &str) -> &'static dyn EventHandler {
    match ability_id {
        "intimidate" => &Intimidate,
        _ => &NoOpHandler,
    }
}
```

**Drawbacks**:
❌ Trait with 100+ methods (one per event type)
❌ Virtual dispatch overhead
❌ Can't return different types per event
❌ Massive trait definition maintenance

---

## Architectural Decisions for Rust

### 1. Event Handler Collection

**Recommendation**: Keep current simplified approach, extend it.

```rust
pub struct EventHandler {
    pub effect_id: ID,
    pub effect_type: EffectType,  // Ability, Item, Move, Condition, etc.
    pub source: Option<(usize, usize)>,  // Pokemon position
}

pub fn find_pokemon_event_handlers(
    &self,
    event_id: &str,
    target: (usize, usize),
) -> Vec<EventHandler> {
    let mut handlers = Vec::new();
    let pokemon = self.pokemon_at(target.0, target.1)?;

    // Check 6 sources with has_callback filtering
    if !pokemon.status.is_empty() && self.has_callback(&pokemon.status, event_id) {
        handlers.push(EventHandler {
            effect_id: pokemon.status.clone(),
            effect_type: EffectType::Status,
            source: Some(target),
        });
    }

    // ... volatiles, ability, item, species, slot conditions

    handlers
}
```

### 2. Event Prefix Broadcasting

**Challenge**: JavaScript dynamically constructs `onAlly${eventName}`, `onAny${eventName}`, etc.

**Recommendation**: Enumerate at call site.

```rust
pub fn find_event_handlers(
    &self,
    target: EventTarget,
    event_name: &str,
    source: Option<(usize, usize)>,
) -> Vec<EventHandler> {
    let mut handlers = Vec::new();

    match target {
        EventTarget::Pokemon(pos) => {
            // Direct handler
            handlers.extend(self.find_pokemon_event_handlers(
                &format!("on{}", event_name), pos
            ));

            // Ally handlers
            for ally_pos in self.get_allies(pos) {
                handlers.extend(self.find_pokemon_event_handlers(
                    &format!("onAlly{}", event_name), ally_pos
                ));
                handlers.extend(self.find_pokemon_event_handlers(
                    &format!("onAny{}", event_name), ally_pos
                ));
            }

            // Foe handlers
            for foe_pos in self.get_foes(pos) {
                handlers.extend(self.find_pokemon_event_handlers(
                    &format!("onFoe{}", event_name), foe_pos
                ));
                handlers.extend(self.find_pokemon_event_handlers(
                    &format!("onAny{}", event_name), foe_pos
                ));
            }

            // Source handlers
            if let Some(src_pos) = source {
                handlers.extend(self.find_pokemon_event_handlers(
                    &format!("onSource{}", event_name), src_pos
                ));
            }
        }
        // ... Side, Field, Battle targets
    }

    handlers
}
```

### 3. Event Execution

**Recommendation**: Simplified dispatcher without state/priority complexity initially.

```rust
pub fn run_event(
    &mut self,
    event_id: &str,
    target: EventTarget,
    source: Option<(usize, usize)>,
    effect: Option<&ID>,
) -> EventResult {
    // 1. Collect handlers
    let handlers = self.find_event_handlers(target, event_id, source);

    // 2. Sort by priority/speed (later)
    // handlers.sort_by(|a, b| self.compare_priority(a, b));

    // 3. Execute each handler
    for handler in handlers {
        let result = self.dispatch_handler(&handler, target, source, effect);

        match result {
            EventResult::Stop => return EventResult::Stop,
            EventResult::Boolean(false) => return EventResult::Boolean(false),
            _ => {}  // Continue to next handler
        }
    }

    EventResult::Continue
}

fn dispatch_handler(
    &mut self,
    handler: &EventHandler,
    target: EventTarget,
    source: Option<(usize, usize)>,
    effect: Option<&ID>,
) -> EventResult {
    match handler.effect_type {
        EffectType::Ability => {
            ability_callbacks::dispatch(
                self,
                handler.effect_id.as_str(),
                event_id,
                target,
                source,
            )
        }
        EffectType::Item => {
            item_callbacks::dispatch(
                self,
                handler.effect_id.as_str(),
                event_id,
                target,
                source,
            )
        }
        // ... other effect types
    }
}
```

### 4. Suppression Checks

**Challenge**: JavaScript checks various suppression conditions (Mold Breaker, Gastro Acid, etc.)

**Recommendation**: Dedicated methods.

```rust
impl Battle {
    fn is_handler_suppressed(
        &self,
        handler: &EventHandler,
        event_id: &str,
        effect_source: Option<&ID>,
    ) -> bool {
        match handler.effect_type {
            EffectType::Ability => {
                let pokemon = self.pokemon_at(handler.source.0, handler.source.1)?;

                // Check Gastro Acid / Neutralizing Gas
                if event_id != "End" && pokemon.ignoring_ability() {
                    return true;
                }

                // Check Mold Breaker
                if self.is_ability_breakable(&handler.effect_id) &&
                   self.suppressing_ability(pokemon) {
                    return true;
                }

                false
            }
            EffectType::Item => {
                // Check Embargo / Klutz / Magic Room
                if !["Start", "SwitchIn", "TakeItem"].contains(&event_id) {
                    let pokemon = self.pokemon_at(handler.source.0, handler.source.1)?;
                    if pokemon.ignoring_item() {
                        return true;
                    }
                }
                false
            }
            // ... other types
        }
    }
}
```

---

## Recommended Phased Implementation

### Phase 1: ✅ Core Infrastructure (DONE)
- [x] Dispatcher functions for abilities, items, moves
- [x] `find_pokemon_event_handlers` simplified
- [x] `has_callback` manual implementation

### Phase 2: 🚧 Auto-Generated Has-Callback (IN PROGRESS)
- [x] Create `generate-can-dispatch.js` script
- [ ] Test and validate generated code
- [ ] Integrate into build pipeline

### Phase 3: Event Broadcasting
- [ ] Implement `find_event_handlers` with prefixes (onAlly, onFoe, onAny, onSource)
- [ ] Implement `find_field_event_handlers`
- [ ] Implement `find_side_event_handlers`
- [ ] Implement `find_battle_event_handlers`

### Phase 4: Event Execution
- [ ] Implement `run_event` with handler collection
- [ ] Add suppression checks (Mold Breaker, Gastro Acid, etc.)
- [ ] Add priority/speed sorting
- [ ] Handle relay variables and return values

### Phase 5: State Management
- [ ] Implement state objects (statusState, abilityState, etc.)
- [ ] Duration tracking (`getKey` parameter)
- [ ] Effect removal callbacks

---

## Performance Considerations

**Rust Advantages**:
- ✅ Compile-time dispatch (match statements)
- ✅ No undefined checks at runtime
- ✅ Stack allocation for handlers
- ✅ Zero-cost abstractions

**Trade-offs**:
- ❌ Code generation required (scripts/generate-*.js)
- ❌ Less dynamic than JavaScript
- ❌ More upfront design needed

**Estimated Performance**: **10-100x faster** than JavaScript for event dispatch due to:
1. No dynamic property lookups
2. Direct function calls (no virtual dispatch if using match)
3. Better memory locality (Vec vs dynamic arrays)
4. Compile-time optimizations

---

## Conclusion

**RECOMMENDED**: **Solution 1 (Generated Can-Dispatch Methods)**

This approach:
1. Maintains compatibility with JavaScript semantics
2. Leverages Rust's strengths (static dispatch, const matching)
3. Avoids runtime overhead
4. Keeps code generation manageable
5. Provides clear upgrade path

**Next Steps**:
1. ✅ Complete `generate-can-dispatch.js` script
2. Test generated code compiles and is correct
3. Implement `find_event_handlers` with prefix broadcasting
4. Implement `run_event` execution loop
5. Add suppression checks
6. Profile and optimize
