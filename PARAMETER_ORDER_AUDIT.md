# Parameter Order Audit

This document audits all dispatcher functions to identify parameter naming issues where parameter names don't match the actual data being passed.

## Critical Finding: dispatch_self_on_hit

**Location**: `src/data/move_callbacks/mod.rs:748-752`

**Function Signature**:
```rust
pub fn dispatch_self_on_hit(
    battle: &mut Battle,
    move_id: &str,
    target_pos: (usize, usize),         // ❌ NAME IS MISLEADING
    source_pos: Option<(usize, usize)>, // ❌ NAME IS MISLEADING
) -> EventResult
```

**Call Site**: `src/battle/handle_move_event.rs:117`
```rust
move_callbacks::dispatch_self_on_hit(self, move_str, src, target_pos)
//                                                     ^^^  ^^^^^^^^^^
//                                                   source    target
```

**What Actually Gets Passed**:
- Parameter named `target_pos` RECEIVES: `src` (the SOURCE position - move user)
- Parameter named `source_pos` RECEIVES: `target_pos` (the TARGET position - move target)

**Impact**: 40+ callback implementations have backwards parameter names
- batonpass, burnup, doubleshock
- gmaxbefuddle, gmaxcannonade, gmaxcentiferno, gmaxchistrike, gmaxcuddle
- gmaxdepletion, gmaxfinale, gmaxfoamburst, gmaxgoldrush, gmaxmalodor
- gmaxmeltdown, gmaxreplenish, gmaxsandblast, gmaxsmite, gmaxsteelsurge
- gmaxstonesurge, gmaxstunshock, gmaxsweetness, gmaxtartness, gmaxterror
- gmaxvinelash, gmaxvolcalith, gmaxvoltcrash, gmaxwildfire, gmaxwindrage
- maxairstream, maxdarkness, maxflare, maxflutterby, maxgeyser
- maxhailstorm, maxknuckle, maxlightning, maxmindstorm, maxooze
- maxovergrowth, maxphantasm, maxquake, maxrockfall, maxstarfall
- maxsteelspike, maxstrike, maxwyrmwind
- psychoshift, shedtail, sparklyswirl

**Bugs Already Fixed** (using backwards names):
- `ba9ab35b` - gmaxcannonade self.onHit parameter order
- Other gmax moves likely have similar issues

---

## dispatch_on_hit (CORRECT)

**Location**: `src/data/move_callbacks/mod.rs:810-814`

**Function Signature**:
```rust
pub fn dispatch_on_hit(
    battle: &mut Battle,
    move_id: &str,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult
```

**Call Site**: `src/battle/handle_move_event.rs:110`
```rust
move_callbacks::dispatch_on_hit(self, move_str, target_pos.unwrap_or((0,0)), source_pos)
//                                                ^^^^^^^^^                   ^^^^^^^^^^
//                                                  target                     source
```

**What Actually Gets Passed**:
- Parameter named `target_pos` RECEIVES: `target_pos` ✅ CORRECT
- Parameter named `source_pos` RECEIVES: `source_pos` ✅ CORRECT

**Status**: ✅ No renaming needed - names match actual data

---

## Other Dispatchers to Audit

### dispatch_on_after_hit
**Location**: `src/data/move_callbacks/mod.rs` (need to check)
**Call Site**: `src/battle/handle_move_event.rs:28`
```rust
move_callbacks::dispatch_on_after_hit(self, move_str, target_pos.unwrap_or((0,0)), source_pos.unwrap_or((0,0)))
```
**Status**: ⏳ Need to verify parameter names match

### dispatch_on_after_move
**Location**: `src/data/move_callbacks/mod.rs` (need to check)
**Call Site**: `src/battle/handle_move_event.rs:31`
```rust
move_callbacks::dispatch_on_after_move(self, move_str, target_pos.unwrap_or((0,0)), source_pos)
```
**Status**: ⏳ Need to verify parameter names match

### dispatch_on_after_move_secondary_self
**Location**: `src/data/move_callbacks/mod.rs` (need to check)
**Call Site**: `src/battle/handle_move_event.rs:33-35`
```rust
move_callbacks::dispatch_on_after_move_secondary_self(
    self, move_str, target_pos.unwrap_or((0,0)), source_pos,
)
```
**Status**: ⏳ Need to verify parameter names match

### dispatch_on_after_sub_damage
**Location**: `src/data/move_callbacks/mod.rs` (need to check)
**Call Site**: `src/battle/handle_move_event.rs:44`
```rust
move_callbacks::dispatch_on_after_sub_damage(self, move_str, target_pos.unwrap_or((0,0)), damage, source_pos)
```
**Status**: ⏳ Need to verify parameter names match

### dispatch_on_base_power
**Location**: `src/data/move_callbacks/mod.rs` (need to check)
**Call Site**: `src/battle/handle_move_event.rs:56`
```rust
move_callbacks::dispatch_on_base_power(self, move_str, base_power, target_pos.unwrap_or((0,0)), source_pos)
```
**Status**: ⏳ Need to verify parameter names match

### dispatch_on_damage
**Location**: `src/data/move_callbacks/mod.rs` (need to check)
**Call Site**: `src/battle/handle_move_event.rs:77-84`
```rust
move_callbacks::dispatch_on_damage(
    self,
    move_str,
    damage,
    target_pos.unwrap_or((0,0)),
    source_pos,
    effect_id.as_deref(),
)
```
**Status**: ⏳ Need to verify parameter names match

### dispatch_on_try_hit
**Location**: `src/data/move_callbacks/mod.rs` (need to check)
**Call Site**: `src/battle/handle_move_event.rs:148`
```rust
move_callbacks::dispatch_on_try_hit(self, move_str, target_pos.unwrap_or((0,0)), source_pos.unwrap_or((0,0)))
```
**Status**: ⏳ Need to verify parameter names match

---

## Condition Callbacks to Audit

Need to audit:
- `src/data/condition_callbacks/mod.rs`
- `src/battle/handle_condition_event.rs`

---

## Ability Callbacks to Audit

Need to audit:
- `src/battle/handle_ability_event.rs`

---

## Recommended Fix Approach

1. **DO NOT** change all 40+ callback implementations
2. **INSTEAD**: Rename the dispatcher function parameters to match actual data
3. **UPDATE**: All individual callback implementations will automatically work correctly

**Proposed Fix for dispatch_self_on_hit**:
```rust
pub fn dispatch_self_on_hit(
    battle: &mut Battle,
    move_id: &str,
    user_pos: (usize, usize),           // Renamed from target_pos - receives SOURCE
    target_pos: Option<(usize, usize)>, // Renamed from source_pos - receives TARGET
) -> EventResult {
    match move_id {
        "batonpass" => batonpass::self_callbacks::on_hit(battle, user_pos, target_pos),
        "gmaxcannonade" => gmaxcannonade::self_callbacks::on_hit(battle, user_pos, target_pos),
        // ... all other callbacks updated ...
    }
}
```

Then update call site in `handle_move_event.rs:117`:
```rust
move_callbacks::dispatch_self_on_hit(self, move_str, src, target_pos)
//                                                     ^^^  ^^^^^^^^^^
//                                                    user   target
// NOW MATCHES: user_pos, target_pos - CORRECT!
```

---

## Testing Strategy

After renaming:
1. Run `cargo build` to check compilation
2. Run `./tests/test-100-seeds.sh` to verify no regressions
3. Check that pass rate improves (current: 37%, target: 50%+)
