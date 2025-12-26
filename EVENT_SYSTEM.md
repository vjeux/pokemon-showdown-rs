# Event System Analysis

## Overview

The Pokemon Showdown event system is the core architectural component that allows abilities, items, moves, field conditions, and other effects to modify battle behavior. Without it, achieving 1-to-1 translation of battle methods from JavaScript to Rust is impossible.

## Current Status

**JavaScript**: Fully implemented with `singleEvent()`, `runEvent()`, `priorityEvent()`, and `eachEvent()` methods
**Rust**: **NOT IMPLEMENTED** - Methods have stubs or simplified versions without event integration

## Event System Methods

### 1. singleEvent() - battle.ts:571-652

Triggers a single event handler with:
- **Stack overflow protection** (max depth 8)
- **Infinite loop detection** (max 1000 log lines)
- **Context switching** - saves/restores parent effect, effectState, event
- **Suppression logic**:
  - Status changes
  - Mold Breaker suppressing abilities
  - Embargo/Klutz/Magic Room suppressing items
  - Gastro Acid/Neutralizing Gas suppressing abilities
  - Air Lock suppressing weather
- **Callback execution** - calls `on{EventName}` handlers
- **Relay variable passing** - allows events to modify return values

### 2. runEvent() - battle.ts:654+

Core event dispatcher that:
- Triggers global event handlers
- Triggers target-specific handlers (`onBlah`)
- Triggers ally handlers (`onAllyBlah`)
- Triggers foe handlers (`onFoeBlah`)
- Triggers source handlers (`onSourceBlah`)
- Handles event priorities and ordering
- Supports cancellation via return false
- Supports modification via relay variables

### 3. priorityEvent() - battle.ts:943+

Similar to runEvent but with priority-based ordering for:
- Item handlers
- Ability handlers
- Status handlers
- Side condition handlers
- Field condition handlers

### 4. eachEvent() - battle.ts:1095+

Iterates through all active Pokemon triggering events on each.

## Event Usage Statistics

**In battle.ts alone**:
- `runEvent()` calls: 36+
- `singleEvent()` calls: 9+

**Event Types Found** (partial list):
- `TryHeal` - Can healing be prevented?
- `Heal` - After healing occurs
- `Damage` - Can damage be prevented/modified?
- `TryBoost` - Can stat boost be prevented?
- `ChangeBoost` - Modify boost amount
- `AfterBoost` - After boost applied
- `AfterEachBoost` - After each individual stat boost
- `SwitchIn` - Pokemon switched in
- `SwitchOut` - Pokemon switching out
- `Faint` - Pokemon fainted
- `ModifyDamage` - Modify damage calculation
- `ModifyAtk` - Modify attack stat
- `ModifyDef` - Modify defense stat
- `ModifySpA` - Modify special attack
- `ModifySpD` - Modify special defense
- `ModifySpeed` - Modify speed
- `ModifyAccuracy` - Modify accuracy
- `ModifyBoost` - Modify boost effectiveness
- `TryHit` - Can move hit?
- `TryMove` - Can move be used?
- `BeforeMove` - Before move executes
- `AfterMove` - After move executes
- ... and many more

## Impact on Methods

The following methods **CANNOT** be properly translated without the event system:

### Critical (Core Battle Logic)
1. `heal()` - Needs TryHeal and Heal events
2. `damage()` / `spreadDamage()` - Needs Damage event, weather immunity checks
3. `directDamage()` - Needs Gen 1 Substitute logic, damage events
4. `boost()` - Needs ChangeBoost, TryBoost, AfterEachBoost, AfterBoost events
5. `setBoost()` - Needs boost events
6. `modify()` / `chainModify()` - Modify stat calculations
7. `runMove()` - Needs TryMove, BeforeMove, AfterMove events
8. `useMove()` - Needs move-related events
9. `tryMoveHit()` - Needs TryHit event
10. `switchIn()` - Needs SwitchIn event
11. `faint()` - Needs Faint event

### Important (Turn Flow)
12. `endTurn()` - Needs Residual events for end-of-turn effects
13. `turnLoop()` - Coordinates turn actions with event-based ordering
14. `runAction()` - Executes actions with event integration
15. `checkWin()` - May need win condition events

### Supporting
16. All stat modification methods
17. All condition/volatile application methods
18. Weather/terrain changes
19. Status application
20. Ability/item changes

## Implementation Requirements

To implement the event system in Rust, we need:

### 1. Effect System
```rust
pub enum EffectType {
    Ability,
    Item,
    Move,
    Status,
    Volatile,
    SideCondition,
    FieldCondition,
    Weather,
    Terrain,
    // ... more
}

pub struct Effect {
    id: ID,
    name: String,
    effect_type: EffectType,
    // Callback handlers for each event type
    on_try_heal: Option<EventCallback>,
    on_heal: Option<EventCallback>,
    on_damage: Option<EventCallback>,
    on_try_boost: Option<EventCallback>,
    // ... hundreds of possible event handlers
}
```

### 2. Event Context
```rust
pub struct EventContext {
    id: String,
    target: Option<PokemonRef>,
    source: Option<PokemonRef>,
    effect: Option<Effect>,
}

pub struct BattleState {
    effect: Option<Effect>,
    effect_state: Option<EffectState>,
    event: Option<EventContext>,
    event_depth: u8,
}
```

### 3. Callback System
- Dynamic dispatch for event handlers
- Ability to call JavaScript/data-defined callbacks
- Type-safe relay variable passing
- Stack overflow protection
- Infinite loop detection

### 4. Suppression Logic
- Mold Breaker ability suppression
- Item suppression (Embargo, Klutz, Magic Room)
- Ability suppression (Gastro Acid, Neutralizing Gas)
- Weather suppression (Air Lock, Cloud Nine)

### 5. Event Registration
- Global event handlers
- Pokemon-specific handlers (onBlah)
- Ally handlers (onAllyBlah)
- Foe handlers (onFoeBlah)
- Source handlers (onSourceBlah)

## Estimated Complexity

**Lines of Code**: 500-1000+ (event system core)
**Additional Impact**: Most battle methods need refactoring (2000+ lines)
**Data Integration**: All abilities, items, moves need event handlers defined
**Testing**: Every ability/item/move interaction needs testing

## Recommendation

Given the scope and complexity:

1. **Option A: Implement Event System First**
   - Massive undertaking (weeks of work)
   - Enables true 1-to-1 translation of all methods
   - Required for competitive accuracy

2. **Option B: Document Event System Gap**
   - Continue comparing methods that don't require events
   - Mark event-dependent methods as "blocked by event system"
   - Create implementation plan for event system as separate project

3. **Option C: Hybrid Approach**
   - Implement simplified event system (basic runEvent/singleEvent)
   - Gradually add event types as needed
   - Accept some behavior differences initially

## Files to Study

For full event system understanding:
- `pokemon-showdown-js/sim/battle.ts` - lines 571-1122 (event methods)
- `pokemon-showdown-js/sim/pokemon.ts` - Pokemon-level event handling
- `pokemon-showdown-js/sim/side.ts` - Side-level event handling
- `pokemon-showdown-js/sim/field.ts` - Field-level event handling
- `pokemon-showdown-js/data/abilities.ts` - Ability event handlers
- `pokemon-showdown-js/data/items.ts` - Item event handlers
- `pokemon-showdown-js/data/moves.ts` - Move event handlers

## Conclusion

The event system is foundational to Pokemon Showdown's architecture. Without it, we can only achieve:
- ✅ Simple methods (RNG, logging, basic state changes)
- ✅ Data-driven calculations (when events don't modify them)
- ❌ Complex battle logic (damage, healing, stat changes, move execution)
- ❌ Ability/item/status interactions
- ❌ Competitive accuracy

**For true 1-to-1 translation, the event system must be implemented.**
