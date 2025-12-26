# Critical Gaps Found in battle.rs Methods

**Date**: 2025-12-26
**Verification Status**: In Progress (checked methods 1-28, fixed 3 gaps)

## Summary

Despite being marked as 96% complete with methods labeled "MATCH", detailed line-by-line comparison reveals **significant implementation gaps** that were not previously detected.

## Fixes Applied ✅

### 1. findPokemonEventHandlers (#22) - FIXED ✅

**Status**: Was marked as MATCH, actually was SIMPLIFIED
**Fixed**: Added missing species and slot conditions

**Changes Made**:
- ✅ Added species handler: `handlers.push((pokemon.species_id.clone(), Some(target)));`
- ✅ Added slot condition handlers: Loop through `side.slot_conditions.get(pokemon.position)`
- ✅ All 43 tests still passing after fix

**Impact**: Event system now collects all Pokemon-related handlers

---

### 2. damage (#26) - FIXED ✅

**Status**: Was marked as FIXED, but missing event fallback
**Fixed**: Added `this.event` fallback for target/source/effect

**Changes Made**:
- ✅ Extract event context: `(event.target, event.source, event.effect.clone())`
- ✅ Use `.or()` to apply fallbacks: `target.or(event_target)`
- ✅ Handle borrow checker properly by extracting values first
- ✅ All 43 tests still passing after fix

**Impact**: Event-driven damage now uses correct context

---

### 3. directDamage (#28) - FIXED ✅

**Status**: Was marked as FIXED, but missing event fallback
**Fixed**: Added `this.event` fallback for target/source/effect

**Changes Made**:
- ✅ Same pattern as damage() - extract event context first
- ✅ Apply fallbacks before validation
- ✅ All 43 tests still passing after fix

**Impact**: Event-driven direct damage now uses correct context

---

## Major Gaps Remaining

### 1. findEventHandlers (#17) - SEVERELY SIMPLIFIED

**Status in TODO**: ✅ MATCH
**Actual Status**: ❌ MAJOR MISMATCH

**JavaScript** (battle.ts:1036-1096, 60 lines):
- Handles arrays of targets **recursively**
- Distinguishes between Pokemon, Side, and Battle target types
- Implements "bubble down" behavior for Side events
- Checks for **prefixed handlers**: `onAlly${eventName}`, `onFoe${eventName}`, `onAny${eventName}`, `onSource${eventName}`
- Only adds prefixed handlers for certain events (excludes BeforeTurn, Update, Weather, WeatherChange, TerrainChange)
- Calls specialized methods:
  - `findPokemonEventHandlers()`
  - `findSideEventHandlers()`
  - `findFieldEventHandlers()`
  - `findBattleEventHandlers()`

**Rust** (battle.rs:6224-6272, 49 lines):
- ❌ No array recursion
- ❌ No Pokemon/Side/Battle type distinction
- ❌ No bubble down behavior
- ❌ No prefixed handler support
- ❌ No event name filtering
- ❌ Doesn't call specialized find*EventHandlers methods
- ✅ Only collects basic IDs from abilities, items, status, volatiles

**Impact**: Critical - Event system cannot work correctly without proper handler collection

**Complexity**: HIGH - Requires significant refactoring with target type handling

---

## Methods Still To Check

- Methods 1-15: ✅ Already verified in previous sessions
- Methods 16-28: ✅ Checked this session (3 gaps fixed, 1 major gap found)
- **Methods 29-96**: ⏳ NOT YET CHECKED

## Test Status

✅ All 43 tests passing after fixes

## Action Plan

### Phase 1: Fix Remaining Critical Gaps ✅ IN PROGRESS
1. ✅ Fixed findPokemonEventHandlers - added species and slot conditions
2. ✅ Fixed damage - added event context fallback
3. ✅ Fixed directDamage - added event context fallback
4. ⏳ Fix findEventHandlers - add recursion, bubbling, prefixed handlers (COMPLEX)

### Phase 2: Continue Systematic Verification
1. Check methods 29-50
2. Check methods 51-75
3. Check methods 76-96

### Phase 3: Test After Each Fix
- ✅ Tests pass after findPokemonEventHandlers fix
- ✅ Tests pass after damage fix
- ✅ Tests pass after directDamage fix
- Run full test suite after findEventHandlers fix

---

**Progress**: 3/4 gaps in methods 1-28 fixed
**Tests**: 43/43 passing ✅
**Next Step**: Continue checking methods 29-96 OR tackle findEventHandlers rewrite
