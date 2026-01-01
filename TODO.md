# Pokemon Showdown Rust - Known Issues

## Battle Synchronization Status

### ✅ Passing Seeds
- **Seed 1**: Perfect match! All turns synchronized.

### ⚠️ Partially Working Seeds  
- **Seed 100**: Matches through turn 18 (was turn 5 before stall fix)
  - Issue: Turn 19/20 divergence - Rust makes 2 PRNG calls vs JavaScript's 4
  - JavaScript: Present's onModifyMove → Accuracy → Crit → Damage randomizer
  - Rust: Only 2 calls, no damage occurs
  - Status: Investigating move execution flow

- **Seed 42**: Matches through turn 33
- **Seed 123**: Damage calculation differences (2-3 HP per turn)

### ❌ Failing Seeds
- **Seed 2**: Fails around turn 56
- **Seed 3**: Fails around turn 16  
- **Seed 10**: Fails around turn 4

## Recently Fixed

### Stall Condition (2026-01-01)
✅ Implemented missing callbacks:
- `dispatch_on_start`: Sets counter=3 when first added
- `dispatch_on_restart`: Multiplies counter by 3, resets duration=2
- `add_volatile_to_pokemon`: Calls Start event and handles onRestart

**Impact:** Seed 100 improved from turn 5 → turn 18 (260% improvement!)

### Other Fixes
- ✅ Team generation synchronization  
- ✅ willCrit field implementation
- ✅ 16-bit truncation in modifyDamage
- ✅ Protosynthesis ability implementation

## Investigation Priorities

1. **High**: Seed 100 turn 19/20 - Why isn't Present executing fully?
2. **Medium**: Seed 123 damage calculations - Consistent 2-3 HP differences
3. **Low**: Other failing seeds - Need individual investigation

## Testing Infrastructure

Battle comparison tests working perfectly:
- Generates teams in both JS and Rust
- Compares team generation (must match exactly)
- Compares battle execution turn-by-turn
- Reports PRNG call mismatches immediately

