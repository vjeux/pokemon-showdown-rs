# PRNG Desynchronization Issue - Seed [0,0,0,2]

## Status: ❌ BUG FOUND

Seed [0,0,0,1] matches perfectly, but seed [0,0,0,2] has differences.

## Symptoms

HP values are consistently off by 1 HP for P2 across all turns:

| Turn | JavaScript P2 HP | Rust P2 HP | Difference |
|------|------------------|------------|------------|
| 2    | 264/302          | 263/302    | -1         |
| 3    | 237/302          | 236/302    | -1         |
| 4    | 212/302          | 211/302    | -1         |
| 5    | 186/302          | 185/302    | -1         |

P1 HP matches perfectly at all turns.

## PRNG Desynchronization

The PRNG calls during team preview are different:

**JavaScript:**
```
PRNG: random(0, 2) = 1
PRNG: random(0, 2) = 0  # ← Different!
PRNG: random(0, 2) = 1
PRNG: random(0, 2) = 1
PRNG: random(100) = 55  # Damage roll
PRNG: random(24) = 17
PRNG: random(16) = 11
```

**Rust:**
```
PRNG: random(Some(0), Some(2)) = 1
PRNG: random(Some(0), Some(2)) = 1  # ← Different! Should be 0
PRNG: random(Some(0), Some(2)) = 0
PRNG: random(Some(0), Some(2)) = 0
PRNG: random(Some(100), None) = 82  # Wrong damage roll
PRNG: random(Some(24), None) = 13
```

The second `random(0,2)` call returns different values:
- JavaScript: 0
- Rust: 1

This cascades into all subsequent PRNG calls being wrong.

## Root Cause

The PRNG is being called a different number of times or in a different order during battle initialization, likely in one of these areas:

1. **Gender randomization** (set_player.rs:109, 119, 128)
   - Possible issue: Checking species gender incorrectly
   - Need to verify gender assignment matches JS exactly

2. **Team shuffling/ordering**
   - Different logic for selecting lead Pokemon

3. **Side initialization**
   - Some initialization step consuming different PRNG calls

## Next Steps

1. Add detailed PRNG logging to Rust during setPlayer and team initialization
2. Compare exact sequence of PRNG calls between JS and Rust
3. Find where the divergence occurs
4. Port the exact JavaScript logic 1-to-1

## Files to Investigate

- `src/battle/set_player.rs` - Gender randomization (lines 109, 119, 128)
- `src/battle/new.rs` - Battle initialization
- `src/battle/start.rs` - Battle start logic
- `src/side/new.rs` - Side initialization
- `src/pokemon/new.rs` - Pokemon creation

## Test Command

```bash
node test-seeds.js
```

This will test seeds [0,0,0,1] through [0,0,0,100] and stop at the first mismatch.
