# Refactoring Plan: 1-to-1 Method Translation

**Date**: 2025-12-26
**Goal**: Ensure every method in battle.rs is a direct 1-to-1 translation of battle.ts
**Status**: 27/104 methods compared (26%)

## Current State Summary

### Progress
- ✅ **9 methods matching** (8.7%)
- ⚠️ **3 methods with minor mismatches** (2.9%)
- ❌ **14 methods with major mismatches** (13.5%)
- 📝 **1 method N/A** (0.96%)
- 🔍 **77 methods not yet compared** (74%)

### Critical Discovery

**The event system is the primary blocker for achieving 1-to-1 translation.**

Approximately 60-70% of battle methods depend on:
- `runEvent()` - 36+ calls in battle.ts
- `singleEvent()` - 9+ calls in battle.ts
- `priorityEvent()` - Multiple calls
- `eachEvent()` - Multiple calls

Without these, methods like `heal()`, `damage()`, `boost()`, `checkWin()`, `hint()`, and many others cannot match JavaScript behavior.

## Refactoring Priority

### Phase 1: Complete Method Comparison (Current Phase)
**Estimated Time**: Ongoing
**Goal**: Document all 104 methods

1. Continue systematic comparison of remaining 77 methods
2. Check battle-actions.ts for delegated methods (runMove, useMove, tryMoveHit, etc.)
3. Document each mismatch with specific differences
4. Update BATTLE_METHODS_TODO.md and COMPARISON_SUMMARY.md

### Phase 2: Fix Non-Event-Dependent Mismatches
**Estimated Time**: Days
**Blocking**: None

Methods that can be fixed without the event system:

1. **setPlayer** (battle.rs:315)
   - Add avatar support
   - Add edit existing player support
   - Add JSON logging
   - Add "player" log message with rating

2. **win** (battle.rs:2987)
   - Add ally side handling for team battles ("Player1 & Player2" format)

3. **modify** (battle.rs:2960)
   - Add array parameter support (optional, low priority - can use modify_f)

4. **checkWin** (battle.rs:694)
   - Implement proper logic: check all-sides-out + foe-Pokemon-left
   - Currently uses has_lost() which doesn't match

5. **getPokemon** (battle.rs:4319)
   - Returns tuple (architectural difference - ACCEPTABLE)

### Phase 3: Implement Event System (CRITICAL BLOCKER)
**Estimated Time**: Weeks (500-1000+ lines of core system)
**Blocking**: 60-70% of all methods

Required components:

1. **Core Event Methods**
   - `single_event()` - With stack overflow protection, context switching, suppression logic
   - `run_event()` - Global + target-specific + ally + foe + source handlers
   - `priority_event()` - Priority-based ordering
   - `each_event()` - Iterate through all active Pokemon

2. **Effect System**
   ```rust
   pub enum EffectType {
       Ability, Item, Move, Status, Volatile,
       SideCondition, FieldCondition, Weather, Terrain
   }

   pub struct Effect {
       id: ID,
       name: String,
       effect_type: EffectType,
       // Hundreds of possible event handlers
       on_try_heal: Option<EventCallback>,
       on_heal: Option<EventCallback>,
       on_damage: Option<EventCallback>,
       // ... etc
   }
   ```

3. **Event Context**
   ```rust
   pub struct EventContext {
       id: String,
       target: Option<PokemonRef>,
       source: Option<PokemonRef>,
       effect: Option<Effect>,
   }
   ```

4. **Callback System**
   - Dynamic dispatch for event handlers
   - Type-safe relay variable passing
   - Stack overflow protection (max depth 8)
   - Infinite loop detection (max 1000 log lines)

5. **Suppression Logic**
   - Mold Breaker ability suppression
   - Item suppression (Embargo, Klutz, Magic Room)
   - Ability suppression (Gastro Acid, Neutralizing Gas)
   - Weather suppression (Air Lock, Cloud Nine)

### Phase 4: Refactor Event-Dependent Methods
**Estimated Time**: Weeks
**Blocking**: Requires Phase 3 complete

Methods that require event system (partial list):

1. **damage** - Add Damage event, weather immunity, special logging
2. **spreadDamage** - **MISSING** - Implement full method with events
3. **directDamage** - Add Gen 1 Substitute cases, Pokemon.damage() delegation
4. **heal** - Add TryHeal/Heal events, special logging, Pokemon.heal() delegation
5. **boost** - Add 4 boost events (ChangeBoost, TryBoost, AfterEachBoost, AfterBoost)
6. **hint** - Add addSplit() call for side-specific hints
7. **addSplit** - Implement full split logging (secret/shared protocol)
8. **chainModify** - Modify this.event.modifier (state mutation)
9. **getActionSpeed** - Implement ModifyPriority events
10. **runMove**, **useMove**, **tryMoveHit** - All depend on move events
11. **endTurn** - Requires Residual events
12. ... and ~50-60 more methods

### Phase 5: Verify and Test
**Estimated Time**: Days
**Blocking**: Requires Phases 2-4 complete

1. Run full test suite
2. Add integration tests for event system
3. Test all ability/item/move interactions
4. Verify competitive accuracy

## Recommended Approach

### Option A: Implement Event System First (Recommended)
**Pros**:
- Unblocks majority of methods
- Enables true 1-to-1 translation
- Required for competitive accuracy

**Cons**:
- Large undertaking (weeks of work)
- Complex architecture

### Option B: Continue Current Approach
**Pros**:
- Complete documentation of all gaps
- Can fix non-event methods immediately

**Cons**:
- Many methods will remain "blocked"
- Limited progress toward 1-to-1 translation goal

### Option C: Hybrid (Current Strategy)
1. ✅ Complete method comparison (Phase 1)
2. ✅ Fix simple non-event mismatches (Phase 2)
3. Document event system requirements (Phase 3 planning)
4. Decide: implement event system or accept limitations

## Files to Study

For event system implementation:
- `pokemon-showdown-js/sim/battle.ts` - lines 571-1122 (event methods)
- `pokemon-showdown-js/sim/pokemon.ts` - Pokemon-level event handling
- `pokemon-showdown-js/sim/side.ts` - Side-level event handling
- `pokemon-showdown-js/sim/field.ts` - Field-level event handling
- `pokemon-showdown-js/data/abilities.ts` - Ability event handlers (examples)
- `pokemon-showdown-js/data/items.ts` - Item event handlers (examples)
- `pokemon-showdown-js/data/moves.ts` - Move event handlers (examples)

## Success Metrics

- [ ] All 104 methods compared and documented
- [ ] All non-event mismatches fixed (6-8 methods)
- [ ] Event system implemented (if pursuing full 1-to-1 translation)
- [ ] All event-dependent methods refactored (50-60 methods)
- [ ] All tests passing
- [ ] Competitive accuracy verified

## Current Blocking Issues

1. **Event System**: Blocks 60-70% of methods
2. **Data Integration**: Abilities/items/moves need event handlers defined
3. **Testing**: Need comprehensive integration tests for event system

## Next Immediate Steps

1. Continue comparing remaining 77 methods
2. Check battle-actions.ts for move execution methods
3. Document all findings in BATTLE_METHODS_TODO.md
4. Update COMPARISON_SUMMARY.md with full results
5. Decide on event system implementation approach
