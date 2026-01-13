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

### 7. ✅ FIXED: Missing onLockMove dispatch cases
**Commit Reference:** `5ad3bd64`

**Problem:** The dispatch_on_lock_move function was missing cases for moves with onLockMove callbacks.

**JavaScript Pattern:** Moves with `condition: { onLockMove: 'movename' }` need dispatch cases.

**Found and Fixed:**
- [x] bide (already present)
- [x] uproar (already present)
- [x] iceball (added)
- [x] rollout (added)

**Audit Process:** Searched JS data/moves.ts for all onLockMove patterns, added missing cases to dispatch_on_lock_move.

Pass rate: 351/723 (48%) -> 362/723 (50%)

### 8. ✅ FIXED: Wake-Up Slap target/source confusion
**Commit Reference:** `81acb188`

**Problem:** Wake-Up Slap on_hit was using wrong position parameter (same issue as Smelling Salts fix d23c5313).

**Pattern:** For move callbacks, on_hit signature is `(target_pos, source_pos)` where:
- target_pos = Pokemon that was hit
- source_pos = Pokemon that used the move

**Wrong:** Used source_pos to check/cure status
**Correct:** Use target_pos to check/cure status on hit Pokemon

Pass rate: 362/723 -> 363/723

## Patterns Identified from Commit Audit

### 9. event.target vs event.source confusion
**Commit Reference:** `20ce3b23`, `e4cf9abc`

**Problem:** In run_event, the parameter naming is swapped from JavaScript:
- `event.target` = attacker (source in JS)
- `event.source` = defender (target in JS)

**Documentation:** Added comments to handlers noting this swap.

### 10. use_item double boost
**Commit Reference:** `dee359f5`

**Problem:** `use_item()` automatically applies boosts from item data. Callbacks should NOT also call `battle.boost()` manually, or boosts will be applied twice.

**Fixed:** Cell Battery

### 11. is_z vs is_z_or_max_powered
**Commit Reference:** `26b88000`, `152ee3cb`

**Problem:** For checking if a move IS a Z/Max Move (static property), use `is_z.is_some()` / `is_max.is_some()` from dex. For checking if a move is BEING USED as a powered-up move (runtime), use `is_z_or_max_powered` from active_move.

**Fixed:** Me First, Mimic, Mirror Move

### 12. onLockMove dispatch completeness
**Commit Reference:** `85de5077`, `5ad3bd64`

**Problem:** dispatch_on_lock_move needs cases for all moves with `condition: { onLockMove: 'movename' }`.

**Fixed:** bide, uproar, iceball, rollout

### 13. ✅ FIXED: EventResult::Null vs EventResult::Stop for preventing actions
**Commit Reference:** `7d5a3192`, `cb374c85`, `ebcfd181`

**Problem:** In JavaScript, `return null` prevents an action (like adding a volatile or dragging out). The Rust translation should be `EventResult::Null`, NOT `EventResult::Stop`.

**JavaScript:** `return null;` (prevents the action)
**Wrong Rust:** `EventResult::Stop`
**Correct Rust:** `EventResult::Null`

**Files Fixed:**
- [x] electricterrain.rs on_try_add_volatile (prevents yawn)
- [x] mistyterrain.rs on_try_add_volatile (prevents confusion)
- [x] ingrain.rs on_drag_out (prevents forced switch)
- [x] magiccoat.rs (returns Null to prevent original move)

### 14. ✅ FIXED: Quick Claw myceliummight check
**Commit Reference:** `b05f97f7`

**Problem:** Quick Claw should not activate for Status moves when the Pokemon has Mycelium Might ability.

**JavaScript:** `if (move.category === "Status" && pokemon.hasAbility("myceliummight")) return;`
**Fixed:** Added check in quickclaw.rs on_fractional_priority

### 15. || 1 JS falsy pattern
**Commit Reference:** `dedfe001`

**Problem:** In JavaScript, `value || 1` returns 1 if value is 0 (falsy). In Rust, `.unwrap_or(1)` only applies for None.

**JavaScript:** `(damage * 1.5) || 1`
**Wrong Rust:** `(damage as f64 * 1.5) as i32` (returns 0 if damage is 0)
**Correct Rust:** `((damage as f64 * 1.5) as i32).max(1)`

**Files Fixed:**
- [x] counter.rs
- [x] mirrorcoat.rs
- [x] metalburst.rs (already correct)
- [x] comeuppance.rs (already correct)
- [x] painsplit.rs (already correct - uses if/else)

## Progress Log

- 2026-01-13: Created this file
- 2026-01-13: Fixed move_data.move_type -> active_move.move_type in 18 ability callbacks
- 2026-01-13: Fixed move_data.move_type -> active_move.move_type in 3 item callbacks + 3 move callbacks
- 2026-01-13: Fixed EventResult::Number after chain_modify in 5 files
- 2026-01-13: Fixed move_data.category -> active_move.category in 10 ability callbacks
- 2026-01-13: Fixed Protean to use active_move.move_type
- 2026-01-13: Added missing onLockMove cases for iceball and rollout - Pass rate 351 -> 362
- 2026-01-13: Fixed Wake-Up Slap target/source confusion - Pass rate 362 -> 363
- 2026-01-13: Fixed terrain callbacks to use EventResult::Null - Pass rate stable at 363
- 2026-01-13: Fixed ingrain on_drag_out to use EventResult::Null
- 2026-01-13: Added myceliummight check to Quick Claw
- 2026-01-13: Current pass rate: 363/723 (50%)
