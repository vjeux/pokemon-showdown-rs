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

### 4. ✅ FIXED: is_z/is_max property checks
**Commit Reference:** `152ee3cb`, `26b88000`, `830a410d`

**Problem:** There are TWO related patterns:
1. For checking if a move IS a Z/Max Move (static property), use `is_z.is_some()` / `is_max.is_some()` from dex
2. For checking if a move is BEING USED as a powered-up move (runtime), use `is_z_or_max_powered` from ActiveMove

**Key distinction:**
- `move_data.is_z_or_max_powered` from dex = WRONG (static property doesn't reflect runtime state)
- `active_move.is_z_or_max_powered` = CORRECT for runtime checks
- `target_pokemon.last_move_used.is_z_or_max_powered` = CORRECT for checking last used move

**Files Fixed:**
- [x] Me First
- [x] Mimic
- [x] Mirror Move
- [x] disable.rs - Use `last_move_used.is_z_or_max_powered` instead of dex lookup
- [x] banefulbunker.rs - Use `active_move.is_z_or_max_powered` instead of dex lookup
- [x] burningbulwark.rs - Use `active_move.is_z_or_max_powered` instead of dex lookup

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

### 11. ✅ FIXED: is_z vs is_z_or_max_powered
**Commit Reference:** `26b88000`, `152ee3cb`, `830a410d`

**Problem:** For checking if a move IS a Z/Max Move (static property), use `is_z.is_some()` / `is_max.is_some()` from dex. For checking if a move is BEING USED as a powered-up move (runtime), use `is_z_or_max_powered` from active_move.

**Important:** Never use `move_data.is_z_or_max_powered` after a dex lookup - this is always wrong. Use `active_move.is_z_or_max_powered` or `last_move_used.is_z_or_max_powered`.

**Fixed:** Me First, Mimic, Mirror Move, disable.rs, banefulbunker.rs, burningbulwark.rs (see Pattern #4)

### 12. onLockMove dispatch completeness
**Commit Reference:** `85de5077`, `5ad3bd64`

**Problem:** dispatch_on_lock_move needs cases for all moves with `condition: { onLockMove: 'movename' }`.

**Fixed:** bide, uproar, iceball, rollout

### 13. ✅ FIXED: EventResult::Null vs EventResult::Stop for preventing actions
**Commit Reference:** `7d5a3192`, `cb374c85`, `ebcfd181`, `0732ff6e`

**Problem:** In JavaScript, `return null` prevents an action (like adding a volatile or dragging out). The Rust translation should be `EventResult::Null`, NOT `EventResult::Stop`.

**JavaScript:** `return null;` (prevents the action)
**Wrong Rust:** `EventResult::Stop`
**Correct Rust:** `EventResult::Null`

**Files Fixed (bulk fix 0732ff6e - 38 files):**

Two-turn moves (return null during charge turn):
- [x] bounce.rs
- [x] dig.rs
- [x] dive.rs
- [x] electroshot.rs
- [x] fly.rs
- [x] freezeshock.rs
- [x] geomancy.rs
- [x] iceburn.rs
- [x] meteorbeam.rs
- [x] phantomforce.rs
- [x] razorwind.rs
- [x] shadowforce.rs
- [x] skullbash.rs
- [x] skyattack.rs
- [x] solarbeam.rs
- [x] solarblade.rs

Other moves:
- [x] aurawheel.rs (on_try)
- [x] burnup.rs (on_try_move)
- [x] clangoroussoul.rs (on_try_hit - boost fail)
- [x] defensecurl.rs (on_restart)
- [x] doubleshock.rs (on_try_move)
- [x] focuspunch.rs (on_try_add_volatile - flinch prevention)
- [x] gastroacid.rs (on_try_hit - Ability Shield block)
- [x] grassknot.rs (on_try_hit - Dynamax fail)
- [x] heatcrash.rs (on_try_hit - Dynamax fail)
- [x] heavyslam.rs (on_try_hit - Dynamax fail)
- [x] hyperspacefury.rs (on_try - form check)
- [x] lowkick.rs (on_try_hit - Dynamax fail)
- [x] magnetrise.rs (on_try - Gravity check)
- [x] minimize.rs (on_restart)
- [x] rest.rs (on_try - heal fail, insomnia, vital spirit)
- [x] sappyseed.rs (on_hit - Grass type immunity)
- [x] shelltrap.rs (on_try_move - not hit)
- [x] snatch.rs (on_any_prepare_hit - snatched move)
- [x] soak.rs (on_hit - already Water type)
- [x] splash.rs (on_try - Gravity check)
- [x] stuffcheeks.rs (on_hit - boost fail)

Also fixed:
- [x] throatchop.rs - Changed `// return false;` from Stop to Boolean(false)

Condition callbacks (earlier fix):
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

### 21. ✅ FIXED: get_immunity must use get_types() not .types
**Commit Reference:** `0f09d77a`

**Problem:** When calling `dex.getImmunity(source, target)` in JavaScript, if target is a Pokemon object, it internally calls `target.getTypes()` which runs the Type event. This allows Arceus/Silvally to change type based on held items.

**JavaScript:**
```javascript
const targetTyping: string[] | string = target.getTypes?.() || target.types || target;
```

**Wrong Rust:**
```rust
battle.dex.get_immunity("powder", &target_pokemon.types)
```

**Correct Rust:**
```rust
let types = target_pokemon.get_types(battle, false);
battle.dex.get_immunity("powder", &types)
```

**Files Fixed:**
- [x] hit_step_try_immunity.rs - powder, prankster, and status immunity checks
- [x] safetygoggles.rs - powder immunity check
- [x] octolock.rs - trapped immunity check

**Impact:** Pass rate: 393/723 -> 395/723 (+2 seeds)

### 22. ✅ FIXED: stored_stats.spe changes must update speed field
**Commit Reference:** `562cc3e9`, `c935293b`

**Problem:** When `stored_stats.spe` is modified, the `pokemon.speed` field must also be updated for correct speed sorting during action resolution. Transform was fixed to do this, but Speed Swap was not.

**JavaScript:** In JavaScript, the battle system reads `storedStats.spe` directly. In Rust, we have a separate `speed` field used for sorting.

**Pattern:** Whenever `stored_stats.spe` is modified, immediately update `speed`:
```rust
pokemon.stored_stats.spe = new_value;
pokemon.speed = pokemon.stored_stats.spe as i32;
```

**Files Fixed:**
- [x] transform_into.rs - Already fixed (line 243)
- [x] set_species.rs - Already fixed (lines 128, 288)
- [x] speedswap.rs - Fixed to update speed for both Pokemon

### 23. NEEDS INVESTIGATION: Imprison onFoeDisableMove/onFoeBeforeMove not triggering correctly
**Commit Reference:** `d3293969`

**Problem:** Imprison's onFoeDisableMove and onFoeBeforeMove callbacks are defined and detected, but Staryu still uses Petal Blizzard instead of being forced to Struggle.

**Investigation Notes:**
- The callbacks exist in moves.json (`condition.onFoeDisableMove: true`, `condition.onFoeBeforeMove: true`)
- `has_volatile_callback` finds the callbacks correctly
- But the actual handlers don't seem to be executed
- Simply changing `prefixed_handlers=true` for BeforeMove/DisableMove causes regressions
- Need to understand how JavaScript's event system runs foe handlers

**Failing Seed:** 1249 (Mr. Mime-Galar with Imprison vs Staryu with Petal Blizzard)

### 24. ✅ FIXED: last_move_used vs last_move for runtime flags
**Commit Reference:** `df44f08b`

**Problem:** JavaScript's `target.lastMove` is the full ActiveMove with runtime flags (isZ, isMax, isZOrMaxPowered). Some Rust callbacks were using `last_move` (just the ID) and looking up dex data, missing runtime flags.

**JavaScript:** `target.lastMove.isZ || target.lastMove.isMax` (checks runtime flags)
**Wrong Rust:** `battle.dex.moves().get_by_id(&last_move_id).is_z.is_some()` (checks dex static data)
**Correct Rust:** `last_move_used.is_z.is_some() || last_move_used.is_max.is_some()` (checks runtime ActiveMove)

**Files Fixed:**
- [x] instruct.rs - Now uses last_move_used for is_z/is_max checks
- [x] encore.rs - Now uses last_move_used for is_z/is_max checks

### 25. ✅ FIXED: on_hit parameter order (target first, source second)
**Commit Reference:** `30822986`, `81acb188`

**Problem:** The dispatch_on_hit passes (target_pos, source_pos) matching JavaScript's onHit(target, source, move). Some callbacks had the parameters backwards, using the first param as source or the second param as target.

**JavaScript:** `onHit(target, source, move)` - target is first param
**Dispatch:** `dispatch_on_hit(battle, active_move, target_pos, source_pos)` - target first
**Correct Rust:** First param is target (defender), second param is source (attacker)

**Files Fixed:**
- [x] wakeupslap.rs - Earlier fix
- [x] skydrop.rs - Was using second param for target HP check
- [x] afteryou.rs - Was using second param for queue prioritization
- [x] substitute.rs - Was using second param for damage calculation
- [x] quash.rs - Was using second param for queue delay

**Note:** For self-targeting moves where target == source, the bug is masked. It only manifests for moves that actually hit a different Pokemon.

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
- 2026-01-13: Fixed get_immunity to use get_types() - Pass rate 393 -> 395
- 2026-01-13: Fixed Speed Swap to update speed field after stored_stats.spe change
- 2026-01-13: Audited 100+ commits for additional patterns
- 2026-01-13: Current pass rate: 395/723 (54%)
- 2026-01-13: Fixed is_z_or_max_powered pattern in disable.rs, banefulbunker.rs, burningbulwark.rs
- 2026-01-13: Verified slot position pattern is correctly implemented (wish, futuremove, etc.)
- 2026-01-13: Fixed EventResult::Stop to EventResult::Null in 38 move callbacks (bulk fix for return null pattern)
- 2026-01-13: Fixed on_hit parameter order in Max moves (9 files: genesissupernova, maxovergrowth, maxstarfall, maxmindstorm, maxflare, maxlightning, maxrockfall, maxgeyser, maxhailstorm) - they were using first param as source but dispatch passes target first
- 2026-01-13: Fixed Instruct and Encore to use last_move_used for is_z/is_max runtime flags instead of dex lookup
- 2026-01-13: Fixed on_hit parameter order in skydrop.rs, afteryou.rs, substitute.rs - they were using second param as target but dispatch passes target first

## Patterns Verified as Already Fixed

The following patterns were identified from commit history and verified to be already implemented correctly:

### EventResult::Float for fractional priority
- All on_fractional_priority callbacks correctly return EventResult::Float
- Checked: quickclaw, laggingtail, fullincense, custapberry, myceliummight, quickdraw

### EventResult::Stop is_truthy
- EventResult::Stop correctly returns false in is_truthy() (commit 41c2b99a)
- This enables move-blocking conditions like throatchop, taunt to work

### source_slot uses active slot position
- add_volatile, set_status, set_ability correctly use pokemon.position (commit 615b7b41)
- No remaining instances of incorrect party index usage

### slot condition operations use pokemon.position
- **Verified:** remove_slot_condition, add_slot_condition use `pokemon.position` (slot on field) not `target.1` (party index)
- Lunar Dance and Healing Wish were fixed to use `pokemon.position` (commit `a508b21a`)
- Wish was already correct - uses `pokemon.position` at line 52
- Future Move was already correct - uses `pokemon.position` at line 87
- Pattern applies to all slot condition operations in singles/doubles

### Contrary/Simple boost relay_var
- run_event correctly updates event.relay_var when handlers return EventResult::Boost (commit 2adfbcf5)
- Infrastructure-level fix, no callback-level changes needed

### Magic Guard event.effect.effect_type
- Correctly checks battle.event.effect.effect_type for damage source (commit 78192710)
- Other on_damage callbacks check effect_id, not effect_type - this is correct

### ModifySTAB event for Adaptability
- Adaptability correctly returns EventResult::Float for STAB modification (commit c492a0cb)
- Only ability with onModifySTAB callback

### target/source on_hit parameter order
- dispatch_on_hit passes (target_pos, source_pos) - target first
- Callbacks like clearsmog, worryseed, coreenforcer correctly fixed (commit e1b19666)
- Audited Max move callbacks - they use self_callbacks with correct parameter handling

## Still Needs Investigation

### Pattern #23: Imprison onFoeDisableMove/onFoeBeforeMove
- Callbacks exist and are detected but not executed
- Simple prefixed_handlers=true change causes regressions
- Need deeper investigation of foe handler execution

### Pattern #24: Turn flow after faint switch (MAJOR BUG)
- **Seed 9931**: When a Pokemon faints mid-turn and replacement switches in
- **Bug**: Rust allows actions on the same turn as the switch, and also
  calculates type effectiveness using wrong Pokemon's types
- **Expected**: JS advances to a new turn before replacement can act
- **Example**:
  - JS Turn 8: Kyogre faints, Cinderace switches in (0 PRNG calls, switch only)
  - JS Turn 9: Cinderace uses Surging Strikes against Corphish
  - Rust Turn 8: Both switch AND attack happen (4 PRNG calls), and on subsequent
    turns Surging Strikes calculates type effectiveness against Scraggy (Dark/Fighting)
    instead of Corphish (Water) due to target resolution issues after fainting
- **Root cause**: After faint switch, the target_pos for moves may be pointing to
  wrong Pokemon in the party when the active slot changes

