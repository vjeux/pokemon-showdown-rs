# Refactor Commits TODO

This file tracks patterns found from auditing recent git commits that need to be fixed at scale.

## Patterns Identified

### 1. ✅ FIXED: move_data.move_type vs active_move.move_type
**Commit Reference:** `3c765d1e`, `bcb3b065`

**Problem:** Ability callbacks check `move_data.move_type` (from dex) instead of `active_move.move_type` (runtime). This ignores type changes from abilities like Electrify, Pixilate, Refrigerate, etc.

**JavaScript:** `move.type === 'Fire'` (checks active move's current type)
**Wrong Rust:** `move_data.move_type == "Fire"` (checks dex type)
**Correct Rust:** `active_move.map(|m| m.move_type == "Fire").unwrap_or(false)`

**Files Fixed:**
- [x] waterbubble.rs
- [x] torrent.rs
- [x] overgrow.rs
- [x] blaze.rs
- [x] swarm.rs
- [x] dragonsmaw.rs
- [x] transistor.rs
- [x] rockypayload.rs
- [x] steelworker.rs
- [x] steamengine.rs
- [x] watercompaction.rs
- [x] justified.rs
- [x] thermalexchange.rs
- [x] rattled.rs
- [x] stormdrain.rs
- [x] wellbakedbody.rs
- [x] sapsipper.rs
- [x] steelyspirit.rs
- [x] cellbattery.rs (item)
- [x] dracoplate.rs (item)
- [x] dreadplate.rs (item)
- [x] charge.rs (move - 3 functions)

### 2. ✅ FIXED: EventResult after chain_modify
**Commit Reference:** `bfc1a9e2`, `233e99c0`

**Problem:** `chain_modify()` in JS returns undefined, so `return this.chainModify(...)` returns undefined. The modifier accumulates in event.modifier. Rust callbacks should call `chain_modify` and return `EventResult::Continue`, NOT return the modifier value.

**Wrong Rust:**
```rust
let result = battle.chain_modify(2.0);
return EventResult::Number(result);
```

**Correct Rust:**
```rust
battle.chain_modify(2.0);
// Don't return - continue to EventResult::Continue
```

**Files Fixed:**
- [x] charge.rs
- [x] venoshock.rs
- [x] bounce.rs
- [x] thickclub.rs
- [x] twistedspoon.rs

### 3. ✅ FIXED: move_data.category vs active_move.category
**Commit Reference:** `fe40f792`

**Problem:** Ability callbacks check `move_data.category` (from dex) instead of `active_move.category` (runtime). This ignores category changes from moves like Shell Side Arm which can change category at runtime.

**JavaScript:** `move.category === 'Physical'` (checks active move's current category)
**Wrong Rust:** `move_data.category == "Physical"` (checks dex category)
**Correct Rust:** `active_move.map(|m| m.category == "Physical").unwrap_or(false)`

**Files Fixed:**
- [x] wonderskin.rs
- [x] quickdraw.rs
- [x] wonderguard.rs
- [x] telepathy.rs
- [x] goodasgold.rs
- [x] weakarmor.rs
- [x] icescales.rs
- [x] toxicboost.rs
- [x] hustle.rs
- [x] flareboost.rs

**Files already correct (check moveSlots, not active move):**
- anticipation.rs
- forewarn.rs
- myceliummight.rs (already checks active_move.category first)

### 4. is_z/is_max property checks
**Commit Reference:** `152ee3cb`, `26b88000`

**Problem:** Moves like Me First, Mimic, Mirror Move should check `is_z` and `is_max` (dex properties) not `is_z_or_max_powered` (runtime flag).

**Already Fixed:** Me First, Mimic, Mirror Move

### 5. Source/Target parameter order
**Commit Reference:** `20ce3b23`, `e4cf9abc`

**Problem:** In Rust events, `event.target` is the attacker and `event.source` is the defender (swapped from intuitive naming).

**Files to audit:** Handle on case-by-case basis.

### 6. AfterFaint event relay_var
**Commit Reference:** `78ae25b1`

**Problem:** AfterFaint event should pass `length` as relay_var for abilities like Battle Bond, Moxie, Beast Boost.

**Already Fixed.**

## Progress Log

- 2026-01-13: Created this file
- 2026-01-13: Fixed move_data.move_type -> active_move.move_type in 18 ability callbacks
- 2026-01-13: Fixed move_data.move_type -> active_move.move_type in 3 item callbacks + 3 move callbacks
- 2026-01-13: Fixed EventResult::Number after chain_modify in 5 files
- 2026-01-13: Fixed move_data.category -> active_move.category in 10 ability callbacks
- 2026-01-13: Current pass rate: 351/723 (48%)
