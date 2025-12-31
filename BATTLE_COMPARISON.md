# Battle Comparison Investigation

## Infrastructure Setup ✓

- `run-battle-full-trace.js`: Runs full JS battles with PRNG tracking
- `tests/battle_state_comparison.rs`: Rust test with PRNG tracking
- `compare-traces.js`: Compares traces and identifies divergences
- `debug-turn.js`: Debug specific turns in JS

## Findings for Seed 1

### First Divergence: Turn 21

**JavaScript (Turn 21):**
- 3 PRNG calls (calls #70, #71, #72)
- Call stack shows:
  1. hitStepAccuracy (accuracy check)
  2. getDamage (critical hit check)
  3. modifyDamage (damage randomization)

**Rust (Turn 21):**
- 7 PRNG calls (calls #70, #71, #72, #73, #74, #75, #76)
- Extra calls: #73, #74, #75, #76
- All extra calls are `random(n=6)` which suggests move/target selection

### Turn Pattern

| Turn | JS Calls | Rust Calls | Difference |
|------|----------|------------|------------|
| 20   | 0        | 0          | 0 (✓)      |
| 21   | 3        | 7          | +4 (✗)     |
| 22   | 0        | 3          | +3 (✗)     |

Turn 20 has 0 calls in both (likely a switch), which matches perfectly.
Turn 21 diverges with Rust making 4 extra PRNG calls.
Turn 22 also diverges as a consequence.

### Hypothesis

The extra `random(n=6)` calls suggest Rust might be:
1. Calling auto_choose multiple times when it shouldn't
2. Choosing moves/targets when JS doesn't
3. Processing turn-end effects differently

### Next Steps

1. Add detailed logging to track WHERE the extra random(n=6) calls originate
2. Compare the turn processing flow between JS and Rust line-by-line
3. Check if there's a difference in how "default" choices are handled
4. Investigate if turn-end processing differs between implementations

## Battle Outcomes

- **JS**: Player 2 wins in 41 turns (148 PRNG calls)
- **Rust**: Player 1 wins in 47 turns (190 PRNG calls)

The divergence causes completely different battle outcomes.
