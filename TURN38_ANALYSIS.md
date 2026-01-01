# Turn 38 PRNG Divergence - Root Cause Analysis

## Summary
Rust Turn 38 makes 7 PRNG calls while the corresponding JavaScript turns make 4 + 3 = 7 calls total.
The issue is that **Rust combines two JavaScript battle turns into one Rust turn**.

## Detailed Analysis

### JavaScript Behavior
JavaScript processes the battle in discrete `makeChoices()` calls:

**makeChoices #38** (PRNG cumulative 137→141, 4 calls):
- Battle turn 30 actions:
  - Zacian uses Zen Headbutt on Vivillon (hit, 290→127 HP)
  - Vivillon uses Origin Pulse on Zacian (miss)
- Turn advances to 31

**makeChoices #39** (PRNG cumulative 141→144, 3 calls):
- Battle turn 31 actions:
  - Zacian uses Zen Headbutt on Vivillon (critical hit, Vivillon faints 127→0 HP)

### Rust Behavior
**Rust Turn 38** (PRNG cumulative 137→144, 7 calls):
- Processes BOTH battle turn 30 and 31 actions in a single turn:
  - Zacian Zen Headbutt (4 calls)
  - Vivillon Origin Pulse (included in the 4 calls from turn 30)
  - Zacian Zen Headbutt again (3 calls from turn 31)

### Root Cause
The Rust battle loop continues processing actions beyond where JavaScript stops and requests new input.
JavaScript's `makeChoices()` processes exactly one battle turn's worth of actions.
Rust's `make_choices()` is processing actions from multiple battle turns.

### Next Steps
1. Examine Rust's turn loop implementation to understand why it doesn't stop after one battle turn
2. Compare with JavaScript's turn loop to identify where the boundary check should be
3. Fix Rust to match JavaScript's turn-by-turn processing behavior

## Test Case
- Seed: [0, 0, 0, 1]
- Format: gen9randombattle
- Divergence point: Rust Turn 38 (JavaScript battle turns 30-31)
- Teams: teams-js.json / teams-rust.json
