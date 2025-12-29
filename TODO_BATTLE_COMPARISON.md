# Battle State Comparison - Critical Issues

## Summary

Both JavaScript and Rust implementations now use identical teams and make identical move choices (PRNG working correctly). However, the battle outcomes differ significantly:

- **JavaScript**: Battle ends in 39 turns, Player 2 wins
- **Rust**: Battle runs full 100 turns without ending, no winner

## Root Cause: Stats Are Never Calculated

**CONFIRMED:** Pokemon stats (HP, Attack, Defense, Sp.Atk, Sp.Def, Speed) are never calculated from base stats + EVs + IVs + level + nature.

### Evidence Trail

1. **Pokemon::new() (`src/pokemon.rs:236-277`)**:
   - Line 272: `stored_stats: StatsTable::default()` - All stats = 0
   - Line 274: `maxhp: 100` - Hardcoded
   - Line 277: `hp: 100` - Hardcoded

2. **Side::new() (`src/side.rs:141-145`)**:
   - Calls `Pokemon::new(set, n, i)` directly
   - No stat calculation performed

3. **No initialization after creation**:
   - Searched all code - `stored_stats` is NEVER populated except by move effects (Power Trick, Guard Split, etc.)
   - Battle::start_battle() does NOT initialize stats
   - No function calculates stats from dex base stats + set EVs/IVs

## Root Causes Identified

### 1. HP System Mismatch

**JavaScript**:
- Uses actual HP stat values (e.g., Passimian at level 83 has 302 max HP)
- HP calculated from base stats + level + EVs + IVs

**Rust**:
- Uses normalized HP system (100 = full HP)
- This suggests HP calculation is not implemented or uses wrong values

**Evidence**:
```
Turn 0 - Passimian HP:
  JavaScript: 302/302
  Rust: 100/100

Turn 1 - Passimian HP:
  JavaScript: 302/302
  Rust: 18/100 (took 82 damage)
```

### 2. HP Can Go Negative

**Issue**: Rust allows HP to become negative without fainting the Pokemon

**Evidence**:
```
Turn 2 - Passimian:
  HP: -61/100
  Fainted: false
```

**Expected Behavior**:
- HP should be clamped to minimum of 0
- Pokemon should faint when HP reaches 0
- Battle should end when all Pokemon on one side faint

### 3. Missing Faint Logic

**Issue**: Pokemon with negative HP are not marked as fainted

**Impact**:
- Battle never ends
- No forced switches
- Game state becomes invalid

### 4. Frozen HP After Going Negative

**Issue**: Once HP goes negative, it stays at the same negative value

**Evidence**:
```
Turn 2: HP = -61
Turn 3: HP = -61
Turn 4: HP = -61
...
Turn 100: HP = -61
```

**This suggests**: Once in an invalid state, the battle simulation stops processing HP changes correctly

## Implementation Tasks

### Priority 1: HP Calculation
- [ ] Port actual HP stat calculation from JavaScript
  - File: `/Users/vjeux/random/showdown/pokemon-showdown-ts/dist/sim/pokemon.js`
  - Calculate max HP from: base stats + level + EVs + IVs + nature
  - Formula: `floor(floor((2 * base + IV + floor(EV / 4)) * level / 100) + level + 10)`

### Priority 2: Faint Logic
- [ ] Implement HP floor (clamp to 0)
- [ ] Mark Pokemon as fainted when HP ≤ 0
- [ ] Trigger faint events
- [ ] Check for battle end condition (all Pokemon fainted on one side)

### Priority 3: Damage Calculation
- [ ] Verify damage formula is 1-to-1 with JavaScript
  - Base damage formula
  - Type effectiveness
  - STAB (Same Type Attack Bonus)
  - Critical hits
  - Random damage variation

### Priority 4: HP Display
- [ ] Fix HP serialization in battle state recording
  - Use actual HP values, not normalized
  - Match JavaScript output format

## Testing Approach

1. Fix HP calculation first
2. Re-run both tests and compare turn 0 HP values (should match)
3. Fix faint logic
4. Re-run and verify battle ends when Pokemon faint
5. Compare turn-by-turn HP values to identify damage calculation differences
6. Iterate until battles produce identical results

## Reference Files

**JavaScript Implementation**:
- `/Users/vjeux/random/showdown/pokemon-showdown-ts/dist/sim/pokemon.js` - HP calculation
- `/Users/vjeux/random/showdown/pokemon-showdown-ts/dist/sim/battle.js` - Battle logic
- `/Users/vjeux/random/showdown/pokemon-showdown-ts/dist/sim/battle-actions.js` - Damage and faint

**Rust Implementation**:
- `src/battle/pokemon.rs` - Pokemon struct and HP logic
- `src/battle/mod.rs` - Battle struct and turn execution
- `src/battle/damage.rs` - Damage calculation (if exists)
