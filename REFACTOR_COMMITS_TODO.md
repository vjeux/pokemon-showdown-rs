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

### 17. ✅ FIXED: Terrain condition data lookup in resolve_priority
**Commit Reference:** (current)

**Problem:** `get_callback_order`, `get_callback_priority`, and `get_callback_sub_order` were not handling `EffectType::Terrain`. Terrain conditions like Grassy Terrain are defined in moves.json as embedded `condition` blocks, not in conditions.json. This caused terrain residual callbacks to have wrong ordering (None/default instead of proper order).

**JavaScript:** `dex.conditions.getByID('grassyterrain')` looks up the move's embedded condition:
```js
if (this.dex.data.Moves.hasOwnProperty(id) && (found = this.dex.data.Moves[id]).condition) {
    condition = new Condition({ name: found.name || id, ...found.condition });
}
```

**Wrong Rust:** `EffectType::Terrain` was not in match arm, so returned None (default order)
**Correct Rust:** Added `EffectType::Terrain` case that looks up move embedded condition data

**Files Fixed:**
- [x] resolve_priority.rs - Added Terrain handling to all three callback lookup functions

**Impact:** Pass rate: 368/723 -> 387/723 (19 more seeds fixed - Grassy Terrain, Electric Terrain, Misty Terrain, Psychic Terrain related)

### 18. ✅ FIXED: Missing FoeDisableMove and FoeBeforeMove dispatchers
**Commit Reference:** `d3293969`

**Problem:** Condition callbacks like `onFoeDisableMove` and `onFoeBeforeMove` need dispatcher functions and handle_condition_event cases. When missing, the handlers are found by find_event_handlers but not executed, causing conditions like Imprison to silently fail.

**When a condition has onFoe* callbacks:**
1. Add dispatch_on_foe_* function in condition_callbacks/mod.rs
2. Add "Foe*" case in handle_condition_event.rs
3. Add condition_id case to the dispatch function

**Example - Imprison:**
- JavaScript: `onFoeDisableMove(pokemon) { ... pokemon.disableMove(moveSlot.id); }`
- Missing: No dispatch_on_foe_disable_move or FoeDisableMove case
- Fixed: Added both dispatcher and handler case

**Files Fixed:**
- [x] condition_callbacks/mod.rs - Added dispatch_on_foe_disable_move with imprison case
- [x] handle_condition_event.rs - Added "FoeDisableMove" case
- [x] condition_callbacks/mod.rs - Added imprison case to dispatch_on_foe_before_move

**Impact:** Imprison now blocks moves (forces Struggle) but there's a separate Struggle damage calculation issue

**Other Foe* callbacks to check:**
- onFoeRedirectTarget
- onFoeTrapPokemon
- onFoeMaybeTrapPokemon
- (search for "onFoe" in moves.json and conditions.json to find all)

### 19. ✅ FIXED: Item onHit triggered for status/self moves
**Commit Reference:** `3f3a407d`

**Problem:** Item onHit callbacks like Enigma Berry check move effectiveness using `run_effectiveness()`, but this doesn't match JavaScript's `getMoveHitData().typeMod`. JavaScript's getMoveHitData() only returns data for moves that dealt damage - status moves don't have hit data.

**Example - Enigma Berry:**
- JavaScript: `target.getMoveHitData(move).typeMod > 0` - only checks actual damage hits
- Wrong Rust: `Pokemon::run_effectiveness(battle, target_pos, active_move)` - calculates for any move

**The fix adds guards:**
1. Skip if move category is "Status" (no damage dealt)
2. Skip if source == target (self-targeting moves)

**Impact:** Pass rate: 387/723 -> 392/723 (5 more seeds fixed)

### 16. ✅ FIXED: last_move_used stores full ActiveMove
**Commit Reference:** `fc8a82bd`

**Problem:** JavaScript stores the full ActiveMove object in `pokemon.lastMoveUsed`, including runtime type modifications (from abilities like Pixilate, etc.). Rust was storing just the ID, preventing access to runtime type via `last_move_used.type`.

**JavaScript:** `pokemon.lastMoveUsed = move;` (full ActiveMove)
**Wrong Rust:** `last_move_used: Option<ID>`
**Correct Rust:** `last_move_used: Option<Box<ActiveMove>>`

**Files Fixed:**
- [x] pokemon_struct.rs - Changed type from Option<ID> to Option<Box<ActiveMove>>
- [x] new.rs - Initialize as None
- [x] clear_volatile.rs - Clear full object
- [x] move_used.rs - Set full active_move
- [x] use_move_inner.rs - Set after ModifyType
- [x] run_move.rs - Remove early ID setting
- [x] switch_in.rs - Clear on switch
- [x] faint_messages.rs - Clear on faint
- [x] conversion2.rs - Access move_type directly from last_move_used
- [x] mysteryberry.rs - Access .id from last_move_used

**Impact:** Pass rate: 363/723 -> 368/723 (5 more seeds fixed, mostly Conversion2 and Revival Blessing related)

### 20. ✅ FIXED: Supersweet Syrup using ability_state instead of Pokemon field
**Commit Reference:** `be64f55a`

**Problem:** Once-per-battle ability flags must be stored as Pokemon struct fields, not in ability_state or volatiles. Intrepid Sword and Dauntless Shield use `pokemon.swordBoost` and `pokemon.shieldBoost` (Pokemon properties), but Supersweet Syrup was incorrectly using `ability_state.syrup_triggered`.

**JavaScript:**
- `pokemon.swordBoost` - Pokemon property, persists across switches and ability changes
- `this.effectState.embodied` - Ability effect state, reset when ability changes

**Pattern:**
- Pokemon properties (like `pokemon.syrupTriggered`) must use Pokemon struct fields
- Ability effect state (like `this.effectState.embodied`) correctly uses ability_state

**Files Fixed:**
- [x] supersweetsyrup.rs - Changed from `ability_state.syrup_triggered` to `pokemon.syrup_triggered`

**Impact:** Pass rate: 392/723 -> 393/723 (+1 seed)

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
- 2026-01-13: Changed last_move_used from ID to full ActiveMove - Pass rate 363 -> 368
- 2026-01-13: Fixed Terrain condition data lookup in resolve_priority - Pass rate 368 -> 387
- 2026-01-13: Fixed FoeDisableMove/FoeBeforeMove dispatchers for Imprison
- 2026-01-13: Fixed Enigma Berry to skip status/self moves - Pass rate 387 -> 392
- 2026-01-13: Fixed Supersweet Syrup to use Pokemon field - Pass rate 392 -> 393
- 2026-01-13: Current pass rate: 393/723 (54%)
