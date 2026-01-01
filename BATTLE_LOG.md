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
