# Refactoring Plan: Shared Reference Types

This document tracks the refactoring of frequently-cloned types to use `Rc<RefCell<>>` or `Rc<>` wrappers, similar to the `SharedEffectState` pattern already implemented.

## Motivation

Many types are cloned repeatedly during event dispatch, causing unnecessary allocations. By using reference-counted wrappers, we can share these values cheaply (clone just increments ref count).

## Types to Refactor

### 1. ActiveMove → SharedActiveMove (HIGH PRIORITY)

**Status:** In Progress

**Stats:**
- 100+ fields (very large struct)
- 229+ clones across 104 files
- IS mutated in 5 places (needs RefCell)

**Implementation:**
- Create `SharedActiveMove` wrapper: `Rc<RefCell<ActiveMove>>`
- Pattern matches `SharedEffectState`
- Provides `borrow()` and `borrow_mut()` methods

**Key files to update:**
- `src/battle/battle_struct.rs` - `active_move` and `last_move` fields
- `src/battle/handle_ability_event.rs` - major clone site
- `src/battle/handle_item_event.rs` - major clone site
- `src/battle/handle_move_event.rs` - clone site
- `src/battle/handle_condition_event.rs` - clone site
- `src/battle_actions/*.rs` - many clone sites
- `src/data/move_callbacks/*.rs` - mutation sites
- `src/data/item_callbacks/*.rs` - mutation sites

### 2. Effect → SharedEffect (MEDIUM PRIORITY)

**Status:** Pending

**Stats:**
- 6 fields (moderate size)
- 58+ clones across 24 files
- NOT mutated after creation

**Implementation:**
- Create `SharedEffect` wrapper: `Rc<Effect>` (no RefCell needed - immutable)
- Simpler than SharedActiveMove since no mutation
- Just provides `Deref` to inner Effect

**Key files to update:**
- `src/battle/battle_struct.rs` - `effect` field
- `src/battle/run_event.rs` - EventListener creation
- `src/battle/damage.rs` - effect cloning
- Event handler files

## Implementation Order

1. [x] SharedEffectState (already done)
2. [ ] SharedActiveMove (this PR)
3. [ ] SharedEffect (future PR)

## Testing

After each refactoring:
```bash
# Build
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build 2>&1" | tail -40

# Run tests
./tests/test-unified.sh 1 500
```
