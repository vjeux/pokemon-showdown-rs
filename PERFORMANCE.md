# Performance Analysis & Optimization Proposal

This document analyzes the performance bottlenecks in the Rust Pokemon Showdown battle simulator and proposes architectural changes to improve performance.

## Current Performance

**Profiling Results** (1000 battles, single-threaded, release build):

| Metric | Value |
|--------|-------|
| Battles/sec | 48.7 |
| Turns/sec | 2,381 |
| ms/battle | 20.55 |
| ms/turn | 0.42 |
| Avg turns/battle | 48.9 |

**Time Breakdown:**

| Phase | Time | % |
|-------|------|---|
| Dex load | 11ms | 0.1% |
| Team generation | 1,896ms | 9.2% |
| Battle creation | 1,051ms | 5.1% |
| **Battle execution** | **16,938ms** | **82.4%** |
| **Total** | **20,548ms** | 100% |

The vast majority of time (82%) is spent in battle execution, making it the primary optimization target.

---

## Root Cause Analysis

Analysis reveals that **60-80% of battle execution time** is spent in event dispatch infrastructure, not actual game logic:

| Bottleneck | Time % | Issue |
|------------|--------|-------|
| Handler discovery (`find_*_event_handlers`) | 20-25% | 48+ Vec allocations per residual event, clones Effect/State each time |
| Speed sorting | 15-20% | O(n²) selection sort, closure call per comparison, PRNG tie-breaking |
| Effect state cloning | 15-20% | EffectState contains HashMap, cloned 100+ times per turn |
| Dex lookups | 10-15% | 4-5 HashMap lookups per handler dispatch, often duplicated |
| Vec operations | 5-10% | `remove(0)` is O(n), `concat()` allocates new vectors |

### Detailed Breakdown

#### 1. Handler Discovery (20-25%)

**Location:** `src/battle/find_pokemon_event_handlers.rs`, `find_side_event_handlers.rs`, etc.

For every event (Damage, Residual, ModifyAtk, etc.), the system:
1. Allocates a new `Vec<EventListener>`
2. Iterates through all Pokemon's abilities, items, volatiles, status
3. Clones `Effect` and `EffectState` for each handler found
4. Repeats for sides, field, and battle-level handlers

**Example flow for `field_event("Residual")`:**
```
field_event("Residual")
├─ find_field_event_handlers()     → Vec allocation + clones
├─ for each side (2):
│  ├─ find_side_event_handlers()   → Vec allocation + clones
│  └─ for each active Pokemon (up to 6):
│     ├─ find_pokemon_event_handlers() → Vec allocation + clones
│     ├─ find_side_event_handlers()    → Vec allocation + clones
│     ├─ find_field_event_handlers()   → Vec allocation + clones
│     └─ find_battle_event_handlers()  → Vec allocation + clones
```

**Total: 48+ separate handler discovery calls per residual event**

#### 2. Speed Sorting (15-20%)

**Location:** `src/battle/speed_sort.rs`

Uses selection sort O(n²) with:
- Closure call for every comparison (`get_priority()`)
- New `Vec` allocation per iteration for tracking equal-priority items
- PRNG calls for shuffling ties

For 50 handlers: ~1,200 comparisons, ~50 Vec allocations, up to 500 PRNG calls.

#### 3. EffectState Cloning (15-20%)

**Location:** `src/event_system.rs`

```rust
pub struct EffectState {
    pub boosts: HashMap<String, i32>,  // Expensive to clone
    pub costOf: HashMap<String, i32>,  // Expensive to clone
    pub duration: Option<i32>,
    pub turns: Option<i32>,
    // ... 15+ other fields
}
```

This struct is cloned:
- In `single_event()` for each handler execution
- In `run_event()` for each handler in the loop
- In handler discovery when creating `EventListener`

**Result: 100+ clones per turn, each copying HashMap data**

#### 4. Dex Lookups (10-15%)

**Location:** `src/battle/dispatch_single_event.rs`

```rust
// Same effect_id looked up multiple times:
if self.dex.moves().get(effect_id) { ... }      // Lookup 1
if self.dex.abilities().get(effect_id) { ... }  // Lookup 2
if self.dex.items().get(effect_id) { ... }      // Lookup 3
// ... later in same function:
if self.dex.abilities().get(effect_id) { ... }  // Lookup 4 (duplicate!)
if self.dex.items().get(effect_id) { ... }      // Lookup 5 (duplicate!)
```

---

## Proposed Architectural Changes

### 1. Handler Registry Cache

**Impact: 15-20% speedup**

**Problem:** Every event triggers 4-6 `find_*_event_handlers()` calls, each allocating vectors and cloning data.

**Solution:** Pre-compute handler registrations at battle start and invalidate on state changes.

```rust
struct HandlerRegistry {
    // Indexed by event_id
    handlers_by_event: HashMap<ID, Vec<HandlerRef>>,
    // Dirty flag for invalidation
    dirty: bool,
}

struct HandlerRef {
    effect_type: EffectType,
    effect_id: ID,           // Interned, no clone needed
    holder: EffectHolder,
    priority: i32,
    sub_order: i32,
    // EffectState fetched lazily when needed
}
```

**Benefits:**
- `find_event_handlers()` becomes O(1) HashMap lookup + filter
- Rebuild only when abilities/items/volatiles change
- Eliminates 48+ allocations per turn

### 2. Replace Selection Sort with Merge Sort

**Impact: 10-15% speedup**

**Problem:** O(n²) selection sort with per-comparison closures.

**Solution:** Use stable merge sort with cached priority values.

```rust
fn speed_sort_optimized<T>(
    &mut self,
    list: &mut Vec<T>,
    get_priority: impl Fn(&T) -> PriorityItem
) {
    // Pre-compute priorities once (not per comparison)
    let mut indexed: Vec<(usize, PriorityItem)> = list.iter()
        .enumerate()
        .map(|(i, item)| (i, get_priority(item)))
        .collect();

    // Stable sort O(n log n)
    indexed.sort_by(|(_, a), (_, b)| Self::compare_priority(a, b));

    // Shuffle equal-priority groups with PRNG
    // ...

    // Reorder list based on sorted indices
}
```

**Benefits:**
- Reduces comparisons from ~1,200 to ~200 for 50 handlers
- Eliminates per-iteration Vec allocations
- Priority computed once per handler, not per comparison

### 3. Simplify EffectState Structure

**Impact: 10-15% speedup**

**Problem:** `EffectState` contains `HashMap<String, i32>` which is expensive to clone.

**Solution:** Replace HashMap with fixed-size arrays.

```rust
// Before: ~200+ bytes, expensive clone
pub struct EffectState {
    pub boosts: HashMap<String, i32>,
    pub costOf: HashMap<String, i32>,
    // ...
}

// After: ~64 bytes, trivial clone
pub struct EffectState {
    pub boosts: [i8; 7],  // atk, def, spa, spd, spe, accuracy, evasion
    pub cost_of: [i16; 4], // Fixed set of cost types
    // ...
}
```

**Alternative:** Use arena allocation with indices instead of cloning.

```rust
struct EffectStateArena {
    states: Vec<EffectState>,
}

type EffectStateRef = u32;  // Index into arena, Copy instead of Clone
```

### 4. String Interning for Effect IDs

**Impact: 5-10% speedup**

**Problem:** Effect IDs created as `String` and cloned repeatedly.

**Solution:** Ensure `ID` type is used consistently as an interned index.

```rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ID(u32);  // Index into global intern table

impl ID {
    fn as_str(&self) -> &'static str { /* fast lookup */ }
}
```

**Benefits:**
- `ID` is `Copy`, no allocation on clone
- Comparison is integer comparison, not string comparison
- Memory: 4 bytes vs 24+ bytes for String

### 5. Dex Lookup Caching

**Impact: 5-10% speedup**

**Problem:** Same effect_id looked up 4-5 times in `dispatch_single_event()`.

**Solution:** Cache effect type determination.

```rust
struct EffectCache {
    effect_types: HashMap<ID, EffectType>,
}

fn dispatch_single_event(&mut self, effect_id: &ID) -> EventResult {
    let effect_type = self.effect_cache
        .get(effect_id)
        .copied()
        .unwrap_or_else(|| self.determine_and_cache_effect_type(effect_id));

    match effect_type {
        EffectType::Move => self.handle_move_event(...),
        EffectType::Ability => self.handle_ability_event(...),
        EffectType::Item => self.handle_item_event(...),
        EffectType::Condition => self.handle_condition_event(...),
    }
}
```

### 6. Use VecDeque for Handler Processing

**Impact: 3-5% speedup**

**Problem:** `handlers.remove(0)` is O(n) because it shifts all elements.

**Solution:** Use `VecDeque` or reverse iteration.

```rust
// Option A: VecDeque with O(1) pop_front
let mut handlers: VecDeque<Handler> = handlers.into();
while let Some(handler) = handlers.pop_front() {
    // process handler
}

// Option B: Reverse and use pop() which is O(1)
handlers.reverse();
while let Some(handler) = handlers.pop() {
    // process handler
}
```

---

## Expected Performance Gains

| Optimization | Est. Speedup | Implementation Complexity |
|--------------|--------------|---------------------------|
| Handler Registry Cache | 15-20% | Medium (requires invalidation logic) |
| Merge Sort for speed_sort | 10-15% | Low |
| EffectState simplification | 10-15% | Medium (touches many files) |
| String interning (ID) | 5-10% | Low (mostly already done) |
| Dex lookup caching | 5-10% | Low |
| VecDeque for handlers | 3-5% | Low |
| **Combined (optimistic)** | **40-60%** | - |

**Target Performance:** 80-120 battles/sec (single-threaded), up from 48.7

---

## Recommended Implementation Order

### Phase 1: Quick Wins (Low complexity, immediate gains)

1. **VecDeque for handler processing** - Simple change, 3-5% gain
2. **Merge sort for speed_sort** - Replace algorithm, 10-15% gain
3. **Dex lookup caching** - Add cache layer, 5-10% gain

### Phase 2: Medium Effort

4. **EffectState simplification** - Replace HashMap with arrays, 10-15% gain
5. **Ensure ID is used consistently** - Audit String usage, 5-10% gain

### Phase 3: Architectural Refactor

6. **Handler Registry Cache** - Largest change, largest gain, 15-20%

---

## Profiling Commands

Run the profiling tool:
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --release --example profile_battle 1000 2>&1"
```

Run parallel battle tests:
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --release --example test_unified 1000 2>&1"
```

---

## Appendix: Key Files by Performance Impact

| File | Impact | Description |
|------|--------|-------------|
| `src/battle/field_event.rs` | 12-15% | Field event dispatch, calls find_* functions |
| `src/battle/speed_sort.rs` | 15-20% | Selection sort implementation |
| `src/battle/run_event.rs` | 15-20% | Main event loop, handler cloning |
| `src/battle/find_pokemon_event_handlers.rs` | 8-10% | Pokemon handler discovery |
| `src/battle/dispatch_single_event.rs` | 5-8% | Dex lookups for dispatch |
| `src/battle/single_event.rs` | 5-8% | State management per event |
| `src/event_system.rs` | 10-15% | EffectState struct definition |
