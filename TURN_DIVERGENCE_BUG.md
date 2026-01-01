# Turn Divergence Bug - Secondary Effect Check

## Bug Summary
At turn 30 (PRNG 137-144), Rust makes an extra PRNG call that JavaScript doesn't, causing Origin Pulse to hit instead of miss.

**Rust:** Zen Headbutt uses PRNG #138-141 (4 calls), Origin Pulse uses #142-144 (3 calls) = 7 total
**JavaScript:** Zen Headbutt uses PRNG #138-140 (3 calls), Origin Pulse uses #141 (1 call, misses) = 4 total

**Result:**
- Rust: Origin Pulse hits (PRNG #142: roll=49 < 85%), Zacian dies
- JavaScript: Origin Pulse misses (PRNG #141: roll=90 >= 85%), Zacian survives

## Root Cause

**PRNG #141 in Rust:** `random(100) = 90` - This is Zen Headbutt's secondary effect check (20% flinch)
**PRNG #141 in JavaScript:** Used for Origin Pulse accuracy check (85%)

Zen Headbutt has `secondary: { chance: 20, volatileStatus: 'flinch' }`.

**JavaScript processing:**
1. Zen Headbutt hits Vivillon
2. Damage calculation runs
3. **Targets are filtered** (set to `false`) based on damage array
4. `spreadMoveHit` calls `this.secondaries(targets, ...)`
5. `secondaries()` checks `if (target === false) continue;` - **SKIPS the secondary check**
6. No PRNG call for flinch
7. Origin Pulse uses PRNG #141 for accuracy check → MISS

**Rust processing:**
1. Zen Headbutt hits Vivillon
2. Damage calculation runs
3. `spread_move_hit` uses `final_targets` (built early, not filtered)
4. Secondary check processes all targets in `final_targets`
5. **Makes PRNG call for flinch** (PRNG #141)
6. Origin Pulse uses PRNG #142 for accuracy check → HIT

## The Fix Needed

In Rust's `spread_move_hit.rs`, the secondary effect check (line 365+) uses `final_targets` which doesn't have the proper filtering.

**JavaScript filtering logic** (battle-actions.ts lines 1086-1105):
```js
// After various stages, filter targets where damage is falsy
if (!damage[i]) targets[i] = false;  // Line 1086
if (damage[i] === false) targets[i] = false;  // Line 1091-1093, 1096-1098
if (!damage[i] && damage[i] !== 0) targets[i] = false;  // Line 1103-1105
```

Then `secondaries()` is called with the filtered `targets` array.

**Solution:**
1. Track which targets are still valid after `run_move_effects`
2. Use the filtered target list when checking secondary effects
3. Match JavaScript's filtering logic exactly

## Key JavaScript Code

**dex-moves.ts conversion:**
```ts
this.secondaries = data.secondaries || (this.secondary && [this.secondary]) || null;
```
JavaScript converts `secondary` (singular) to `secondaries` (array with one element).

**spreadMoveHit filtering:**
```js
// Step 3: onHit event
damage = this.runMoveEffects(...);

for (const i of targets.keys()) {
    if (!damage[i] && damage[i] !== 0) targets[i] = false;
}

// Step 5: secondary effects
if (moveData.secondaries) this.secondaries(targets, pokemon, move, moveData, isSelf);
```

**secondaries() function:**
```js
secondaries(targets, source, move, moveData, isSelf) {
    if (!moveData.secondaries) return;
    for (const target of targets) {
        if (target === false) continue;  // ← KEY: Skip filtered targets
        const secondaries = this.battle.runEvent('ModifySecondaries', target, source, moveData, moveData.secondaries.slice());
        for (const secondary of secondaries) {
            const secondaryRoll = this.battle.random(100);
            // ... check if secondary applies
        }
    }
}
```

## Files to Modify

1. `src/battle_actions/spread_move_hit.rs` - Add target filtering before secondary check
2. `src/battle_actions/run_move_effects.rs` - Check if it returns filtered damages

## Testing

After fix, verify:
- Rust PRNG 137→141 (4 calls total for the turn)
- Origin Pulse uses PRNG #141 and MISSES
- Zacian survives with 104 HP
- Turn advances to 31
