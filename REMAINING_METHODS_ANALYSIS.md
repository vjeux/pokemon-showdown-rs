# Remaining Methods Analysis - 25 Methods Left

**Current Progress**: 71/96 (74%) complete
**Last Updated**: 2025-12-26

## Overview

After systematic comparison of all 96 methods in battle.ts and battle.rs, 25 methods remain to achieve 100% parity. This document analyzes each remaining method to determine what is required for implementation.

## Category 1: Event Infrastructure Required (5 methods) - **BLOCKED**

These methods require event state infrastructure (`this.event.modifier`) that doesn't exist in Rust yet.

### 1. chainModify (battle.ts:2291)
**Blocker**: Requires `this.event.modifier` state
```javascript
chainModify(numerator: number, denominator?: number) {
    const m = this.event.modifier = this.event.modifier || 1;
    this.event.modifier = this.chainModify(m, [numerator, denominator]);
}
```
**Required Infrastructure**: Event state object with modifier tracking

### 2. finalModify (battle.ts:2344)
**Blocker**: Requires `this.event.modifier` state
```javascript
finalModify(value: number) {
    return this.modify(value, this.event.modifier);
}
```
**Required Infrastructure**: Event state object with modifier

###3. getActionSpeed (battle.ts:2590)
**Blocker**: Needs ModifyPriority event
```javascript
getActionSpeed(action: AnyObject) {
    // ... calculates priority with ModifyPriority event
    this.runEvent('ModifyPriority', pokemon, null, null, priority);
}
```
**Required Infrastructure**: ModifyPriority event handler, action.zmove/mega checking

### 4. resolvePriority (battle.ts:950)
**Blocker**: Requires event handler priority/order system
```javascript
resolvePriority(h: EventListenerWithoutPriority, callbackName: string) {
    handler.order = handler.effect[`${callbackName}Order`] || false;
    handler.priority = handler.effect[`${callbackName}Priority`] || 0;
    handler.subOrder = handler.effect[`${callbackName}SubOrder`] || 0;
    // ... complex effectType ordering
}
```
**Required Infrastructure**: EventListener system with priority/order/subOrder

### 5. add (battle.ts:3092)
**Blocker**: Needs function parameter support
```javascript
add(...parts: (Part | (() => { side: SideID, secret: string, shared: string }))[]) {
    // ... handles function callbacks for side-specific messages
}
```
**Required Infrastructure**: Function/closure parameter handling, calls addSplit

**Status**: ❌ **Cannot implement without event state infrastructure**

---

## Category 2: Complex Initialization (1 method) - **REQUIRES FORMAT SYSTEM**

### 6. start (battle.ts:1859)
**Blocker**: Requires format callbacks, rule processing
```javascript
start() {
    // Deserialization support
    if (this.deserialized) return;

    // Multi-battle support
    if (this.started) throw new Error("Battle already started");

    // Format callbacks
    this.format.onBegin?.call(this);
    for (const rule of this.ruleTable.keys()) {
        const subFormat = this.dex.formats.get(rule);
        subFormat.onBegin?.call(this);
    }

    // Team validation
    for (const side of this.sides) {
        for (let i = 0; i < side.pokemon.length; i++) {
            this.actions.checkLearnset(move, species, setSources);
        }
    }
}
```
**Required Infrastructure**:
- Format system with onBegin callbacks
- Rule table with subformat callbacks
- Team validation (checkLearnset)
- Multi-battle support

**Status**: ⚠️ **Partially implementable** - basic structure can be done, callbacks need format system

---

## Category 3: Complex Game Logic (7 methods) - **MIXED**

### 7. maybeTriggerEndlessBattleClause (battle.ts:1757)
**Blocker**: Gen 1 specific logic, staleness tracking
- Requires Pokemon.volatileStaleness, Pokemon.staleness fields
- Requires Pokemon.hasType() method
- Requires Side.foe property
- Complex Gen 1 Transform/Ghost/Frozen/PP logic

**Status**: ⚠️ **Can implement basic structure**, needs Pokemon helper methods

### 8. runPickTeam (battle.ts:1931)
**Blocker**: Format callbacks
```javascript
this.format.onTeamPreview?.call(this);
for (const rule of this.ruleTable.keys()) {
    const subFormat = this.dex.formats.get(rule);
    subFormat.onTeamPreview?.call(this);
}
```
**Required Infrastructure**: Format system with onTeamPreview callbacks

**Status**: ❌ **Cannot implement without format callbacks**

### 9. getTarget (battle.ts:2400)
**Current**: Simplified version exists
**Blocker**: Missing Pokemon helper methods
- pokemon.hasAbility(['stalwart', 'propellertail'])
- pokemon.getAtLoc(targetLoc)
- pokemon.getLocOf(pokemon)
- pokemon.volatiles['twoturnmove']
- target.isAlly(pokemon)

**Status**: ✅ **Can implement fully** once Pokemon helpers added

### 10. getRandomTarget (battle.ts:2453)
**Current**: Simplified version exists
**Blocker**: Missing Pokemon helper methods
- pokemon.adjacentAllies()
- pokemon.adjacentFoes()
- pokemon.side.randomFoe()
- pokemon.side.foe.active

**Status**: ✅ **Can implement fully** once Pokemon/Side helpers added

### 11. faintMessages (battle.ts:2498)
**Blocker**: Requires faintQueue system
```javascript
faintMessages(lastFirst = false) {
    if (this.gen <= 1) {
        // Gen 1: implement if last Pokémon faints
        for (const pokemon of this.faintQueue) {
            if (!pokemon.fainted) {
                this.runEvent('BeforeFaint', pokemon);
                this.add('faint', pokemon);
                pokemon.clearVolatile(false);
                pokemon.faint();
            }
        }
        this.faintQueue = [];
        return;
    }
    // ... complex forme regression logic
}
```
**Required Infrastructure**:
- Battle.faintQueue array
- Pokemon.clearVolatile() method
- Forme regression logic (Necrozma, Wishiwashi, etc.)

**Status**: ⚠️ **Can implement basic version**, needs faintQueue system for full parity

### 12. makeRequest (battle.ts:1331)
**Blocker**: Full request generation
```javascript
makeRequest(type?: string, requestDetails?: AnyObject) {
    if (type) {
        this.requestState = type;
        for (const side of this.sides) {
            side.clearChoice();
        }
    } else {
        type = this.requestState;
    }

    // Generate request based on type ('teampreview', 'move', 'switch', 'wait')
    for (const side of this.sides) {
        side.emitRequest(this.getRequest(side));
    }

    if (this.sides.every(side => side.isChoiceDone())) {
        throw new Error("Choices are done immediately after a request!");
    }
}
```
**Required Infrastructure**:
- Side.clearChoice() method
- Side.emitRequest() method
- Full request generation (getRequest)
- Choice validation

**Status**: ⚠️ **Partially implementable** - basic structure exists, needs full request system

### 13. endTurn (battle.ts:1577)
**Current**: Simplified version exists
**Blocker**: Missing Dynamax, Gen 1 logic, staleness tracking
- Dynamax volatile removal (3-turn limit)
- Gen 1 partial trapping logic
- Pokemon field resets (moveThisTurn, newlySwitched, etc.)
- staleness tracking
- Type appearance tracking (Gen 7+)
- DisableMove event processing

**Status**: ⚠️ **Can expand current implementation**, missing several features

---

## Category 4: Feature Gaps (3 methods) - **MIXED**

### 14. addSplit (battle.ts:3082)
**Current**: Simplified version exists
**Blocker**: Full array parameter support
```javascript
addSplit(side: SideID, secret: Part[], shared?: Part[]) {
    this.log.push(`|split|${side}`);
    this.add(...secret);
    if (shared) {
        this.add(...shared);
    } else {
        this.log.push('');
    }
}
```
**Required Infrastructure**: Array parameter handling (currently takes single strings)

**Status**: ✅ **Can implement fully** - straightforward translation

### 15. hint (battle.ts:3070)
**Current**: Has basic implementation
**Blocker**: Depends on addSplit (method #14)
```javascript
hint(hint: string, once?: boolean, side?: Side) {
    if (this.hints.has(side ? `${side.id}|${hint}` : hint)) return;

    if (side) {
        this.addSplit(side.id, ['-hint', hint]);
    } else {
        this.add('-hint', hint);
    }

    if (once) this.hints.add(side ? `${side.id}|${hint}` : hint);
}
```
**Required Infrastructure**: Battle.hints HashSet, addSplit with arrays

**Status**: ✅ **Can implement fully** once addSplit is fixed

### 16. getDebugLog (battle.ts:3153)
**Blocker**: Requires extractChannelMessages utility
```javascript
getDebugLog() {
    const channelMessages = extractChannelMessages(this.log.join('\n'), [-1]);
    return channelMessages[-1].join('\n');
}
```
**Required Infrastructure**: extractChannelMessages external function

**Status**: ❌ **Cannot implement without external utility** (acceptable difference)

---

## Category 5: Different Infrastructure (4 methods) - **ACCEPTABLE DIFFERENCES**

### 17. getTeam (battle.ts:3164)
**Different Purpose**: JS unpacks/generates team, Rust returns side's pokemon array
```javascript
// JS: Takes PlayerOptions, unpacks team, generates Pokemon
getTeam(options: PlayerOptions | null): PokemonSet[]

// Rust: Returns reference to side's pokemon vector
pub fn get_team(&self, side: usize) -> &Vec<Pokemon>
```
**Status**: ✅ **ACCEPTABLE** - Different purposes due to architecture

### 18. initEffectState (battle.ts:3321)
**Different Signature**: JS uses Partial<EffectState>, Rust uses ID
```javascript
// JS
initEffectState(effect: Effect): EffectState

// Rust
pub fn init_effect_state(&self, id: &ID) -> EffectState
```
**Status**: ⚠️ **Can align signatures** if needed, but difference may be acceptable

### 19. clearEffectState (battle.ts:3333)
**Different Signature**: JS takes EffectState object, Rust takes target + effect_id
```javascript
// JS
clearEffectState(effectState: EffectState)

// Rust
pub fn clear_effect_state(&mut self, target: (usize, usize), effect_id: &ID)
```
**Status**: ⚠️ **Can align signatures** if needed, but difference may be acceptable

### 20. toJSON (battle.ts:318)
**Different Implementation**: JS delegates to State.serializeBattle, Rust has custom
```javascript
// JS
toJSON(): AnyObject {
    return State.serializeBattle(this);
}

// Rust - Serde Serialize impl
impl Serialize for Battle { /* custom */ }
```
**Status**: ✅ **ACCEPTABLE** - Rust uses idiomatic Serde serialization

---

## Summary Statistics

| Category | Count | Implementable | Blocked | Notes |
|----------|-------|---------------|---------|-------|
| Event Infrastructure | 5 | 0 | 5 | Requires event state system |
| Complex Initialization | 1 | 0 | 1 | Requires format callbacks |
| Complex Game Logic | 7 | 2 | 2 | 3 partial |
| Feature Gaps | 3 | 2 | 1 | Straightforward |
| Different Infrastructure | 4 | 4 | 0 | Acceptable differences |
| **TOTAL** | **20** | **8** | **9** | **3 partial** |

Note: 5 methods were already counted as acceptable differences in BATTLE_METHODS_TODO.md

---

## Implementation Roadmap

### Phase 1: Immediate (Can Do Now) ✅
1. **addSplit** - Rewrite to accept arrays
2. **hint** - Full implementation once addSplit fixed
3. **getTarget** - Add Pokemon helper methods (hasAbility, getAtLoc, getLocOf, isAlly)
4. **getRandomTarget** - Add Pokemon/Side helper methods (adjacentAllies, adjacentFoes, randomFoe)

**Estimated**: 4 methods → Would bring total to 75/96 (78%)

### Phase 2: Enhanced Pokemon/Side API
Implement helper methods needed by multiple battle methods:
- Pokemon.hasAbility(abilities: &[&str]) -> bool
- Pokemon.getAtLoc(loc: i8) -> Option<&Pokemon>
- Pokemon.getLocOf(&self, pokemon: &Pokemon) -> i8
- Pokemon.isAlly(&self, other: &Pokemon) -> bool
- Pokemon.adjacentAllies() -> Vec<&Pokemon>
- Pokemon.adjacentFoes() -> Vec<&Pokemon>
- Side.randomFoe() -> Option<&Pokemon>
- Side.foe property/method

**Estimated**: Enables 2-4 more methods

### Phase 3: Moderate Complexity
1. **endTurn** - Expand current implementation with:
   - Dynamax 3-turn removal
   - Gen 1 partial trapping cleanup
   - Pokemon field resets
   - DisableMove event

2. **faintMessages** - Implement with:
   - faintQueue system
   - BeforeFaint/Faint/AfterFaint events
   - Basic forme regression

3. **maybeTriggerEndlessBattleClause** - Implement with:
   - Staleness tracking fields
   - Gen 1 specific logic

**Estimated**: 3 methods → Would bring total to ~78-80/96 (81-83%)

### Phase 4: Infrastructure Dependent (Long-term)
Requires major systems to be built:
1. **Event State System** (chainModify, finalModify, getActionSpeed, resolvePriority)
2. **Format Callbacks** (start, runPickTeam)
3. **Function Parameters** (add)

**Estimated**: 7 methods → Would achieve 87/96 (91%) if all implemented

### Phase 5: Acceptable Differences
4 methods have acceptable architectural differences:
- getTeam, initEffectState, clearEffectState, toJSON

These can be marked as MATCH with documentation of differences.

---

## Realistic Target

**Achievable without major infrastructure**: 75-80/96 (78-83%)

**With moderate complexity**: ~80-85/96 (83-89%)

**With full infrastructure**: ~87-91/96 (91-95%)

**100% match**: Requires JavaScript-style function parameters, event state, format callbacks - significant architectural changes

---

## Recommendation

1. **Implement Phase 1** (addSplit, hint, getTarget, getRandomTarget) → 75/96 (78%)
2. **Implement Phase 2** (Pokemon/Side helpers)
3. **Expand Phase 3** (endTurn, faintMessages, maybeTriggerEndlessBattleClause) → ~80-82/96 (83-85%)
4. **Document blockers** for remaining methods
5. **Mark architectural differences as acceptable** where Rust's approach is equally valid

This would achieve **~83-85% method parity** with all achievable methods fully implemented, and clear documentation of what requires larger infrastructure work.
