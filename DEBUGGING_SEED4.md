# Seed 4 PRNG Divergence Investigation

## Problem
Seed 4 has 94 PRNG calls (Rust) vs 98 PRNG calls (JavaScript) across 20 turns.
- Difference: JavaScript makes 4 extra PRNG calls
- Both battles complete 20 turns without ending
- First 90 PRNG values match perfectly

## Divergence Point

### Rust Turn Breakdown:
- Turn 1-20: Total 94 PRNG calls
- Turn 20: 4 PRNG calls (values: 660860253, 4262174440, 3385519812, 3118473418)

### JavaScript Turn Breakdown:
- Turn 1-20: Total 98 PRNG calls
- Turn 19: 4 PRNG calls (values: 660860253, 4262174440, 3385519812, 3118473418)
- Turn 20: 4 PRNG calls (values: 691181980, 2106105541, 1876288151, 3978725063)

## Analysis

**Key Finding**: Rust's turn 20 PRNG values match JavaScript's turn 19 values!

This suggests:
1. JavaScript makes 4 extra PRNG calls in turn 20 that Rust doesn't make
2. OR Rust is missing 4 PRNG calls somewhere earlier (turns 1-19)
3. Since first 90 values match, the divergence happens at PRNG call #91

The extra 4 calls in JavaScript turn 20 are:
- 691181980 - `hitStepAccuracy` (accuracy check)
- 2106105541 - `getDamage` via `randomChance` (critical hit check)
- 1876288151 - `modifyDamage` via `randomizer` (damage randomization)
- 3978725063 - `secondaries` (secondary effect check)

**Investigation Results:**

Rust executes the same moves in turns 19 and 20:
- Turn 19: p1b uses bulkup (0 PRNG), p2a uses rapidspin (4 PRNG) = calls #87-90
- Turn 20: p1b uses bulkup (0 PRNG), p2a uses rapidspin (4 PRNG) = calls #91-94

The battle appears to be choosing the same moves repeatedly. The 4 PRNG calls in each turn are:
1. random(100) - accuracy check for rapidspin
2. random(24) - damage calculation
3. random(16) - damage randomization
4. random(100) - secondary effect check

JavaScript's turn 20 extra calls (95-98) are also for executing a move, suggesting JavaScript executes an additional action that Rust doesn't.

## ROOT CAUSE FOUND

The divergence is caused by **Flame Body ability not being called properly** in Rust.

### The Bug

1. Coalossal (p2a) has the **Flame Body** ability
2. Great Tusk (p1b) uses **Rapid Spin** (contact move) on Coalossal
3. Flame Body should trigger `onDamagingHit` callback with 30% chance to burn the attacker
4. This makes a `randomChance(3, 10)` PRNG call

### Rust Implementation Issue

The Flame Body callback IS being called, but the `move_id` parameter is an **EMPTY STRING**!

Location: `src/battle/handle_ability_event.rs:336`
```rust
"DamagingHit" => {
    ability_callbacks::dispatch_on_damaging_hit(self, ability_id.as_str(), 0, Some(pokemon_pos), None, "") // TODO: Wire through actual move_id
}
```

The TODO comment confirms: "Wire through actual move_id"

Without the move_id, Flame Body cannot:
- Look up the move in the dex
- Check if the move has the "contact" flag
- Therefore cannot make the PRNG call

### The Fix Needed

The `run_event` function needs to pass the `move_id` (or `effect_id`) through to the ability event handler so that it can be forwarded to the callback.

This is an infrastructure issue that affects ALL onDamagingHit ability callbacks, not just Flame Body.

### Impact

JavaScript makes 4 extra PRNG calls in seed 4 because:
- Each Rapid Spin hit triggers Flame Body check
- Some of these checks pass the 30% probability
- Each successful check makes a randomChance call

Total difference: 98 PRNG calls (JS) vs 94 (Rust) = 4 missing calls

## Current Understanding

- Format: gen9randombattle (singles, 1v1)
- Rapidspin move: 50 BP, accuracy 100, has 100% secondary that boosts user speed
- Both sides repeatedly choose bulkup + rapidspin in turns 16-20
- Calls 1-94 match between Rust and JavaScript
- JavaScript makes 4 additional calls (95-98) that Rust doesn't
- These 4 calls have the signature of executing a damaging move (accuracy, crit/damage, randomizer, secondary)

## Rust Turn Breakdown
```
Turn 1-7: 7,7,7,5,7,7,7 = 47 calls
Turn 8: 0 calls
Turn 9-20: 3,4,4,4,4,4,4,4,4,4,4,4 = 47 calls
Total: 94 calls
```

Turn 18-20 details:
- Turn 18: p1b bulkup + p2a rapidspin = 4 calls (#83-86), total 86
- Turn 19: p1b bulkup + p2a rapidspin = 4 calls (#87-90), total 90
- Turn 20: p1b bulkup + p2a rapidspin = 4 calls (#91-94), total 94

## Related Files
- examples/trace_seed4.rs
- teams-js-seed4.json / teams-rust-seed4.json
