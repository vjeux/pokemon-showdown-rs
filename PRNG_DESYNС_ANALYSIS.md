# PRNG Desynchronization Analysis

## Summary

**Turn 11 (Battle Turn 10)** shows PRNG desynchronization:
- JavaScript: 6 PRNG calls (total 36→42)
- Rust: 4 PRNG calls (total 36→40)
- **Missing in Rust**: Calls #41 and #42

## Battle State

**Before Turn 10:**
- P1 (Grubbin): 240/240 HP
- P2 (Latias-Mega): 4/168 HP

**Moves Executed:**
1. Latias-Mega uses Water Gun → Grubbin takes damage (240→189 HP)
2. Grubbin uses Knock Off → Latias-Mega faints (4→0 HP)

## PRNG Calls Analysis

### Expected Calls (Turn 10)
1. **Speed tie resolution** (if needed): randomChance
2. **Latias Water Gun accuracy**: randomChance
3. **Latias Water Gun damage**: randomizer (0.85-1.0× multiplier)
4. **Grubbin Knock Off accuracy**: randomChance
5. **Grubbin Knock Off damage**: randomizer (0.85-1.0× multiplier)

### Actual Calls

**JavaScript** (6 calls):
- #37: randomChance (speed or accuracy)
- #38: randomChance (accuracy)
- #39: randomizer (damage roll)
- #40: randomChance (accuracy)
- #41: randomChance ← **MISSING IN RUST**
- #42: randomizer (damage roll) ← **MISSING IN RUST**

**Rust** (4 calls):
- #37: randomChance
- #38: randomChance
- #39: randomizer
- #40: randomChance
- [stops here]

## Hypothesis

The missing calls (#41, #42) are from **Grubbin's Knock Off damage calculation**.

Possible causes:
1. Rust is skipping damage calculation when target HP ≤ damage
2. Early return when Pokemon is going to faint
3. Missing accuracy or crit check

## Next Steps

1. Check if `get_damage` or `spread_damage` has early returns for fainting Pokemon
2. Add detailed logging around damage calculation for Knock Off
3. Compare JavaScript battle-actions.ts with Rust implementation

## Files to Investigate

- `src/battle_actions/get_damage.rs` - Main damage calculation
- `src/battle/spread_damage.rs` - Damage application (has HP check at lines 171-174)
- `src/battle_actions/spread_move_hit.rs` - Move execution flow
