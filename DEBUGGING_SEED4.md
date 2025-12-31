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

## Next Steps

1. **Determine exact turn mapping**: Create JavaScript test that logs PRNG count after each turn (turns 1-20)
2. **Find divergence point**: Compare JavaScript turn-by-turn counts with Rust to find where first divergence occurs
3. **Identify missing action**: Once divergence turn is found, add detailed logging to both JS and Rust for that specific turn to see what action Rust is missing or JavaScript is adding extra
4. **Possible causes to investigate**:
   - End-of-turn/residual effects executing differently
   - Move order or priority differences
   - Switch-in or faint mechanics
   - After-move callbacks executing moves
   - Turn structure differences (when turns start/end)

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
