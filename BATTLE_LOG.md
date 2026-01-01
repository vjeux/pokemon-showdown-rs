# Battle Testing Log

This file documents fixes made to synchronize JavaScript and Rust battle implementations.

## 2026-01-01: Team Generation Synchronization

### Issue
Team generation differed between JavaScript and Rust, preventing battle comparison tests from running.

**Seed 123 differences:**
- Line 100: Vivillon-Sandstorm ability: JS "Shield Dust" vs Rust "No Ability"
- Line 137: Move name: JS "hiddenpower" vs Rust "hiddenpowerpsychic"

### Root Causes

1. **Hidden Power moves**: JavaScript's `Dex.moves.all()` returns normalized move IDs where all Hidden Power variants have ID "hiddenpower". Rust uses raw dex keys with distinct IDs like "hiddenpowerpsychic".

2. **Cosmetic forme abilities**: Species like Vivillon-Sandstorm are cosmetic formes without their own ability definitions. They should inherit from their base species (Vivillon).

### Fixes

**JavaScript (tests/generate-test-teams.js):**
- Changed from `Dex.moves.all().map(m => m.id)` to `Object.keys(Dex.data.Moves)` to get raw move IDs
- Added logic to check `baseSpecies` for abilities when cosmetic forme has none

**Rust (src/team_generator.rs):**
- Added logic to check `base_species` for abilities when cosmetic forme has none

### Result
✅ Team generation now matches exactly between JS and Rust.

### Next Issue
Seed 123 now has a battle divergence starting at turn 5:
- JS: Tinkaton(296/316) - took 20 damage
- Rust: Tinkaton(299/316) - took 17 damage
- PRNG calls match (17->20), so event flow is correct
- Damage calculation difference of 3 HP per turn

---

## 2026-01-01: Display Name and Comparison Script Fixes

### Issue
Test comparison was failing due to:
1. Pokemon name display differences (JS: "Slowbro" vs Rust: "Slowbro-Mega")
2. Comparison script including header lines that differ

### Fixes

**examples/test_battle_rust.rs:**
- Strip forme suffixes using `split('-').next()` for display
- Now shows "Slowbro" instead of "Slowbro-Mega" to match JavaScript

**tests/compare-battles.sh:**
- Changed grep pattern from '^#' to '^#[0-9]'
- Only compares numbered battle state lines, excludes headers

### Results
- ✅ Seed 1: PASS (perfect match!)
- ❌ Seed 42: FAIL (PRNG diverges from turn 1)
- ❌ Seed 100: FAIL (battle logic divergence)
- ❌ Seed 123: FAIL (damage calculation divergence from turn 5)

### Analysis
- Display/comparison issues are resolved
- Seed 1 passes completely - provides a working baseline
- Remaining failures are actual battle logic bugs
- Need to investigate: damage calculations, stat modifiers, or move implementations

---

## 2026-01-01: willCrit Field Implementation

### Issue
Moves with guaranteed critical hits (like Wicked Blow) were not properly applying crits.

**Seed 42 divergence:**
- JS turn 1: prng=0->6
- Rust turn 1: prng=0->7 (extra PRNG call)

### Root Cause
`MoveData` struct was missing the `willCrit` field from JavaScript move data. When `get_active_move` created an `ActiveMove`, it always set `will_crit: None`, causing the crit logic to roll for a random crit instead of using the guaranteed crit from the move data.

### Fix
- Added `will_crit: Option<bool>` to `MoveData` struct in `src/dex.rs`
- Updated `get_active_move` to copy `will_crit` from move_data
- Now moves like Wicked Blow (`"willCrit": true`) properly guarantee critical hits

### Results
- ✅ Seed 1: Still passes (no regression)
- ⚠️ Seed 42: Improved! First 4 turns now match exactly
  - Divergence moved from turn 1 to turn 4-5
  - New issue: Different damage/faint timing (needs investigation)

---
